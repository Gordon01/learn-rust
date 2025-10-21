use std::time::Duration;

use plotters::prelude::*;

pub fn plot_results(ts: &[(usize, (Duration, Duration))], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Convert to (x, y1, y2) in milliseconds
    let data: Vec<(f64, f64, f64)> = ts.iter()
        .map(|(chunk, (t1, t2))| (*chunk as f64, t1.as_secs_f64() * 1000.0, t2.as_secs_f64() * 1000.0))
        .collect();

    let max_y = data.iter().flat_map(|(_, a, b)| [*a, *b]).fold(0.0, f64::max);

    let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Sequential vs Parallel", ("sans-serif", 24))
        .margin(10)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0f64..data.last().map(|d| d.0).unwrap_or(1000.0), 0f64..max_y * 1.1)?;

    chart.configure_mesh()
        .x_desc("Chunk size")
        .y_desc("Time (ms)")
        .draw()?;

    chart.draw_series(LineSeries::new(data.iter().map(|(x, y, _)| (*x, *y)), &RED))?
        .label("Sequential")
        .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], &RED));

    chart.draw_series(LineSeries::new(data.iter().map(|(x, _, y)| (*x, *y)), &BLUE))?
        .label("Parallel")
        .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], &BLUE));

    chart.configure_series_labels().border_style(&BLACK).draw()?;

    Ok(())
}
