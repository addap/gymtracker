#![allow(non_snake_case)]
use anyhow::{anyhow, Result};
use chrono::Duration;
use dioxus::prelude::*;
use fermi::use_read;
use itertools::Itertools;
use log::{error, info};
use ordered_float::OrderedFloat;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;

use crate::{
    api,
    auth::ACTIVE_AUTH_TOKEN,
    messages::{MessageProps, UIMessage},
    request_ext::RequestExt,
    scroll_to_end,
    util::lerp,
};
use gt_core::models;

const PADDING_DAYS: i64 = 1;
const PADDING_KG: f64 = 5.0;

#[derive(Props)]
pub struct GraphProps<'a> {
    canvas_id: &'a String,
    canvas_wrapper_id: &'a String,
    data: &'a models::ExerciseGraphQuery,
}

#[derive(Debug, Clone, Copy)]
enum LabelPosition {
    LowerRight,
    Right,
    UpperRight,
}

impl LabelPosition {
    fn into_coord(self) -> (i32, i32) {
        match self {
            Self::LowerRight => (10, 10),
            Self::Right => (10, -5),
            Self::UpperRight => (10, -20),
        }
    }

    fn next(self) -> Option<Self> {
        match self {
            Self::LowerRight => Some(Self::Right),
            Self::Right => Some(Self::UpperRight),
            Self::UpperRight => None,
        }
    }

    fn new() -> Self {
        Self::LowerRight
    }
}

pub fn Graph<'a>(cx: Scope<'a, GraphProps<'a>>) -> Element<'a> {
    use_future(&cx, (), |()| {
        let canvas_wrapper_id = cx.props.canvas_wrapper_id.clone();
        let canvas_id = cx.props.canvas_id.clone();
        let data = cx.props.data.clone();

        async move {
            //
            match draw(&canvas_id, &data) {
                Ok(()) => {
                    // JS function to scroll the canvas all the way to the right.
                    scroll_to_end(&canvas_wrapper_id);
                }
                Err(e) => error!("{}", e),
            }
        }
    });

    let from_date = cx.props.data.per_date.first().unwrap().date - Duration::days(PADDING_DAYS);
    let to_date = (cx.props.data.per_date.last().unwrap().date + Duration::days(PADDING_DAYS))
        .max(from_date + Duration::days(7));
    let width = (to_date - from_date).num_days() * 60;

    cx.render(rsx! {
        div {
            style: "overflow-x: auto; overflow-y: hidden;",
            id: cx.props.canvas_wrapper_id.as_str(),
            h3 {
                cx.props.data.name.clone()
            }
            canvas {
                id: cx.props.canvas_id.as_str(),
                height: 500,
                width: width
            }
        }
    })
}

pub fn draw(canvas_id: &str, data: &models::ExerciseGraphQuery) -> Result<()> {
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    let font_big: FontDesc = ("sans-serif", 20.0).into();
    let font_small: FontDesc = ("sans-serif", 10.0).into();

    root.fill(&WHITE)?;

    if data.per_date.len() < 1 {
        return Err(anyhow!("No data available."));
    }

    // On the x-axis we render at least a week and leave PADDING_DAYS free to the left and right.
    let from_date = data.per_date.first().unwrap().date - Duration::days(PADDING_DAYS);
    let to_date = (data.per_date.last().unwrap().date + Duration::days(PADDING_DAYS))
        .max(from_date + Duration::days(7));

    // TODO If I don't want to draw each individual date I have to give a slice of NaiveDate to build_cartesian_2d.
    // However, this leads to some trait bound error in draw_series I have not been able to solve.
    // let x_dates = data
    //     .per_date
    //     .iter()
    //     .map(|exg| exg.date)
    //     .collect::<Vec<_>>()
    //     .as_slice();

    // On the y-axis we render the max and min of submitted weights +- PADDING_KG.
    let (from_kg, to_kg) = (
        data.per_date
            .iter()
            .flat_map(|exg| exg.weights.iter().map(|(weight, _)| OrderedFloat(*weight)))
            .min()
            .map(|f| f - PADDING_KG)
            .unwrap_or(OrderedFloat(0.0))
            .0,
        data.per_date
            .iter()
            .flat_map(|exg| exg.weights.iter().map(|(weight, _)| OrderedFloat(*weight)))
            .max()
            .map(|f| f + PADDING_KG)
            .unwrap_or(OrderedFloat(100.0))
            .0,
    );

    // Create the chart with y-axes on both sides.
    // TODO The caption should be positioned to the right eventually.
    let mut chart = ChartBuilder::on(&root)
        .margin(10u32)
        .caption(
            format!("{} Progress", data.name),
            font_big, // anchoring caption at the right does not seem to work.
                      // .with_anchor::<RGBColor>(text_anchor::Pos {
                      //     h_pos: text_anchor::HPos::Right,
                      //     v_pos: text_anchor::VPos::Center,
                      // }),
        )
        .x_label_area_size(30u32)
        .y_label_area_size(30u32)
        .right_y_label_area_size(30u32)
        .build_cartesian_2d(from_date..to_date, from_kg..to_kg)?;

    // Further configure the chart.
    // We want labels on every day and almost no light lines between the datapoints.
    // TODO The date labels should be rotated by 45 degrees eventually.
    chart
        .configure_mesh()
        .x_labels((to_date - from_date).num_days() as usize)
        .y_max_light_lines(2)
        .x_max_light_lines(0)
        // RotateAngle(45) would have been nice.
        // Also, rotating by 90 degrees and using an offset also rotates the offset. This seems like a bug.
        // .x_label_offset(30)
        // .x_label_style(
        //     ("sans-serif", 10)
        //         .into_font()
        //         .transform(FontTransform::Rotate90),
        // )
        .x_label_formatter(&|date| date.format("%d. %b").to_string())
        .draw()?;

    // Compute coordinates for points that signify a set.
    // x = date of submission
    // y = weight of the set
    // We add several labels (# of reps) to each coordinate, the position of the labels is also computed here.
    let points = data.per_date.iter().flat_map(|exg| {
        exg.weights
            .iter()
            .sorted_by(|a, b| a.0.total_cmp(&b.0).then(b.1.cmp(&a.1)))
            .scan(
                (LabelPosition::new(), None),
                |(label_pos, opt_last_weight), (weight, reps)| {
                    if let Some(last) = opt_last_weight {
                        if *last == weight {
                            if let Some(next_pos) = label_pos.next() {
                                *label_pos = next_pos;
                            } else {
                                // When there is no next label position we just return None to not print a label.
                                return Some((weight, reps, None));
                            }
                        } else {
                            *label_pos = LabelPosition::new();
                            *opt_last_weight = Some(weight);
                        }
                    } else {
                        *label_pos = LabelPosition::new();
                        *opt_last_weight = Some(weight);
                    }
                    Some((weight, reps, Some(*label_pos)))
                },
            )
            .map(|(weight, reps, label_pos)| (exg.date, *weight, *reps, label_pos))
    });

    // Draw the points for each set. The circle is scaled according to the number of reps.
    chart.draw_series(PointSeries::of_element(
        points,
        5.0,
        &RED,
        &|(x, y, reps, opt_label_pos), s, st| {
            let element = EmptyElement::at((x, y))
                + Circle::new((0, 0), lerp(s, 3.0 * s, reps as f64 / 16.0), st.filled());
            if let Some(label_pos) = opt_label_pos {
                element
                    + Text::new(
                        format!("×{}", reps),
                        label_pos.into_coord(),
                        font_small.clone(),
                    )
            } else {
                element + Text::new("".to_string(), (0, 0), font_small.clone())
            }
        },
    ))?;

    root.present()?;
    Ok(())
}

pub fn GraphPage<'a>(cx: Scope<'a, MessageProps<'a>>) -> Element<'a> {
    let auth_token = use_read(&cx, ACTIVE_AUTH_TOKEN);
    let graph_data = use_state(&cx, || {
        Vec::<(String, String, models::ExerciseGraphQuery)>::new()
    });
    // let search_term = use_state(&cx, || "".to_string());

    let fetch = use_future(&cx, (), |()| {
        to_owned![auth_token, graph_data];
        let display_message = cx.props.display_message.clone();

        async move {
            let client = reqwest::Client::new();
            let res = client
                .get(api::EXERCISE_GRAPH.as_str())
                .bearer_auth(auth_token.clone().unwrap_or("".into()))
                .send()
                .await
                .handle_result::<Vec<models::ExerciseGraphQuery>>(UIMessage::error(
                    "Requesting exercise graph failed.".to_string(),
                ))
                .await;

            match res {
                Ok(data) => {
                    let data_with_id = data
                        .into_iter()
                        .map(|exg| {
                            (
                                format!("canvas-{}", exg.name),
                                format!("canvas-wrapper-{}", exg.name),
                                exg,
                            )
                        })
                        .collect();
                    graph_data.set(data_with_id);
                }
                Err(e) => {
                    display_message.send(e);
                }
            }
        }
    });

    // We render a list of checkboxes to toggle the visibility of graphs for individual exercises.
    // let exercise_names: Vec<String> = graph_data.iter().map(|exg| exg.name.clone()).collect();

    let graphs = graph_data
        .get()
        .iter()
        .map(|(canvas_id, canvas_wrapper_id, exg)| {
            rsx! {
                Graph {
                    canvas_id: canvas_id,
                    canvas_wrapper_id: canvas_wrapper_id,
                    data: exg
                }
            }
        });
    cx.render(rsx! {
        graphs
    })
}
