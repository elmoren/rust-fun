mod waveform;

use waveform::*;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut wf = Waveform::new(
        200e3,
        10e3,
        10e3,
        0.001,
        0.0,
        ComplexType::Real,
        WindowingType::None,
        1.0,
    );

    wf.generate_waveform();

    let root = BitMapBackend::new("plots/wf.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Generated Waveform", ("sans-serif", 30).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..wf.samples().len() as f32, -1.1f32..1.1f32)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(
        LineSeries::new(
            (0..).zip(wf.samples()).map(|(idx, y)| {(idx as f32, y.x)}),
            &BLUE,
        ))?;

    Ok(())
}
