#![allow(non_snake_case)]
use chrono::NaiveDate;
use dioxus::prelude::*;
use fermi::use_read;
use log::error;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;

use crate::{
    api,
    auth::ACTIVE_AUTH_TOKEN,
    messages::{MessageProps, UIMessage},
    request_ext::RequestExt,
};
use gt_core::models;

#[derive(Props)]
pub struct GraphProps<'a> {
    data: &'a (String, models::ExerciseGraphQuery),
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

pub fn draw(
    canvas_id: &str,
    data: &models::ExerciseGraphQuery,
) -> Result<(), Box<dyn std::error::Error>> {
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    let font: FontDesc = ("sans-serif", 20.0).into();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20u32)
        .caption("Bench Press Progress".to_string(), font)
        .x_label_area_size(30u32)
        // .y_label_area_size(30u32)
        .build_cartesian_2d(-1f32..1f32, -1.2f32..1.2f32)?;

    chart.configure_mesh().x_labels(3).y_labels(3).draw()?;

    // chart.draw_series(LineSeries::new(
    //     (-50..=50)
    //         .map(|x| x as f32 / 50.0)
    //         .map(|x| (x, x.powf(power as f32))),
    //     &RED,
    // ))?;

    root.present()?;
    Ok(())
}

pub fn GraphPage<'a>(cx: Scope<'a, MessageProps<'a>>) -> Element<'a> {
    let auth_token = use_read(&cx, ACTIVE_AUTH_TOKEN);
    let graph_data = use_state(&cx, || Vec::<(String, models::ExerciseGraphQuery)>::new());
    let search_term = use_state(&cx, || "".to_string());

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
