fn eee(
    title: &str,
    coordinatess_xy: (Range<f32>, Range<f32>),
    v: Vec<(f32, f32)>,
    label: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let oooo = (-1f32..6f32, -1f32..6f32);
    let root = SVGBackend::new("buffer/2323232.svg", (640, 480)).into_drawing_area();
    //let root = BitMapBackend::new("2323232.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(coordinatess_xy.0, coordinatess_xy.1)?;

    chart.configure_mesh().draw()?;
    //x, y

    //let ii = ww.iter().map(|x| (x, x));
    //let ooo = (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x));

    chart
        .draw_series(LineSeries::new(v, &RED))?
        .label(label)
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}
