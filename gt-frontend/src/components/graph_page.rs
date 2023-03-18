#![allow(non_snake_case)]
use anyhow::{anyhow, Result};
use chrono::{Duration, NaiveDate};
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
    util::lerp,
};
use gt_core::models;

#[derive(Props)]
pub struct GraphProps<'a> {
    data: &'a (String, models::ExerciseGraphQuery),
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
        let data = cx.props.data.clone();

        async move {
            //
            match draw(&data.0, &data.1) {
                Ok(()) => (),
                Err(e) => error!("{}", e),
            }
        }
    });

    cx.render(rsx! {
        div {
            h3 {
                cx.props.data.1.name.clone()
            }
            canvas {
                id: cx.props.data.0.as_str(),
                height: 500,
                width: 500
            }
        }
    })
}

pub fn draw(canvas_id: &str, data: &models::ExerciseGraphQuery) -> Result<()> {
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    let font: FontDesc = ("sans-serif", 20.0).into();

    root.fill(&WHITE)?;

    if data.per_date.len() < 1 {
        return Err(anyhow!("No data available."));
    }

    let (from_date, to_date) = (
        data.per_date.first().unwrap().date - Duration::days(1),
        data.per_date.last().unwrap().date + Duration::days(1),
    );

    // TODO If I don't want to draw each individual date I have to give a slice of NaiveDate to build_cartesian_2d.
    // However, this leads to some trait bound error in draw_series I have not been able to solve.
    // let x_dates = data
    //     .per_date
    //     .iter()
    //     .map(|exg| exg.date)
    //     .collect::<Vec<_>>()
    //     .as_slice();

    let (from_kg, to_kg) = (
        data.per_date
            .iter()
            .flat_map(|exg| exg.weights.iter().map(|(weight, _)| OrderedFloat(*weight)))
            .min()
            .map(|f| f - 5.0)
            .unwrap_or(OrderedFloat(0.0))
            .0,
        data.per_date
            .iter()
            .flat_map(|exg| exg.weights.iter().map(|(weight, _)| OrderedFloat(*weight)))
            .max()
            .map(|f| f + 5.0)
            .unwrap_or(OrderedFloat(100.0))
            .0,
    );

    let mut chart = ChartBuilder::on(&root)
        .margin(10u32)
        .caption(format!("{} Progress", data.name), font)
        .x_label_area_size(30u32)
        .right_y_label_area_size(30u32)
        .build_cartesian_2d(from_date..to_date, from_kg..to_kg)?;

    chart
        .configure_mesh()
        .y_max_light_lines(2)
        .x_max_light_lines(0)
        // TODO RotateAngle(45) would have been nice.
        // Also, rotating by 90 degrees and using an offset also rotates the offset. This seems like a bug.
        // .x_label_offset(30)
        // .x_label_style(
        //     ("sans-serif", 10)
        //         .into_font()
        //         .transform(FontTransform::Rotate90),
        // )
        .x_label_formatter(&|date| date.format("%d. %b").to_string())
        .draw()?;

    let points = data.per_date.iter().flat_map(|exg| {
        exg.weights
            .iter()
            .map(|(weight, reps)| (exg.date, *weight, *reps))
            .sorted_by(|a, b| a.1.total_cmp(&b.1))
            .scan(
                (LabelPosition::new(), None),
                |(label_pos, opt_last), coord| {
                    if let Some(last) = opt_last {
                        if *last == coord.1 {
                            if let Some(next_pos) = label_pos.next() {
                                *label_pos = next_pos;
                            } else {
                                // When there is no next label position we just return None to not print a label.
                                return Some((coord.0, coord.1, coord.2, None));
                            }
                        } else {
                            *label_pos = LabelPosition::new();
                            *opt_last = Some(coord.1);
                        }
                    } else {
                        *label_pos = LabelPosition::new();
                        *opt_last = Some(coord.1);
                    }
                    Some((coord.0, coord.1, coord.2, Some(*label_pos)))
                },
            )
    });

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
                        ("sans-serif", 10).into_font(),
                    )
            } else {
                element + Text::new("".to_string(), (0, 0), ("sans-serif", 10).into_font())
            }
        },
    ))?;

    root.present()?;
    Ok(())
}

pub fn GraphPage<'a>(cx: Scope<'a, MessageProps<'a>>) -> Element<'a> {
    let auth_token = use_read(&cx, ACTIVE_AUTH_TOKEN);
    let graph_data = use_state(&cx, || Vec::<(String, models::ExerciseGraphQuery)>::new());
    // let search_term = use_state(&cx, || "".to_string());

    let fetch = use_future(&cx, (), |()| {
        to_owned![auth_token, graph_data];
        // let display_message = cx.props.display_message.clone();

        // async move {
        //     let client = reqwest::Client::new();
        //     let res = client
        //         .get(api::EXERCISE_GRAPH.as_str())
        //         .bearer_auth(auth_token.clone().unwrap_or("".into()))
        //         .send()
        //         .await
        //         .handle_result(UIMessage::error(
        //             "Requesting exercise graph failed.".to_string(),
        //         ))
        //         .await;

        //     match res {
        //         Ok(data) => graph_data.set(data),
        //         Err(e) => {
        //             display_message.send(e);
        //         }
        //     }
        // }

        async move {
            let dummy_data = models::ExerciseGraphQuery {
                name: "Bench Press".to_string(),
                per_date: vec![
                    models::ExerciseGraphQueryPerDate {
                        date: NaiveDate::from_ymd_opt(2023, 2, 27).unwrap(),
                        weights: vec![(50.0, 12), (50.0, 10), (60.0, 5)],
                    },
                    models::ExerciseGraphQueryPerDate {
                        date: NaiveDate::from_ymd_opt(2023, 3, 1).unwrap(),
                        weights: vec![(60.0, 5), (60.0, 5), (60.0, 4)],
                    },
                    models::ExerciseGraphQueryPerDate {
                        date: NaiveDate::from_ymd_opt(2023, 3, 6).unwrap(),
                        weights: vec![(60.0, 4), (60.0, 4), (60.0, 3)],
                    },
                ],
            };
            graph_data.set(vec![(format!("canvas-{}", dummy_data.name), dummy_data)]);
        }
    });

    // We render a list of checkboxes to toggle the visibility of graphs for individual exercises.
    // let exercise_names: Vec<String> = graph_data.iter().map(|exg| exg.name.clone()).collect();

    // let dummy_data = graph_data.iter().find(|exg| exg.name == "Bench Press");

    let graphs = graph_data
        .get()
        .iter()
        .map(|exg| rsx! { Graph { data: &exg } });
    cx.render(rsx! {
        graphs
    })
}
