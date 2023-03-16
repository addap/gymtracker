#![allow(non_snake_case)]
use dioxus::prelude::*;
use log::error;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;

pub fn GraphPage(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            canvas {
                id: "mycanvas",
                height: 500,
                width: 500
            }
            button {
                onclick: move |_| {
                    match draw("mycanvas", 3) {
                        Ok(()) => (),
                        Err(e) => error!("{}", e)
                    }
                },
                "Draw!"
            }
        }
    })
}

/// Draw power function f(x) = x^power.
pub fn draw(canvas_id: &str, power: i32) -> Result<(), Box<dyn std::error::Error>> {
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    let font: FontDesc = ("sans-serif", 20.0).into();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20u32)
        .caption(format!("y=x^{}", power), font)
        .x_label_area_size(30u32)
        .y_label_area_size(30u32)
        .build_cartesian_2d(-1f32..1f32, -1.2f32..1.2f32)?;

    chart.configure_mesh().x_labels(3).y_labels(3).draw()?;

    chart.draw_series(LineSeries::new(
        (-50..=50)
            .map(|x| x as f32 / 50.0)
            .map(|x| (x, x.powf(power as f32))),
        &RED,
    ))?;

    root.present()?;
    Ok(())
}
