use charming::datatype::{DataFrame, DataPointItem};
use charming::df;
use charming::element::ItemStyle;
use charming::{component::Axis, element::AxisType, series::Bar, Chart, HtmlRenderer};
use charming::{
    component::{DataZoom, Grid, Legend},
    datatype::CompositeValue,
    element::{
        AreaStyle, AxisPointer, AxisPointerType, DataBackground, LineStyle, SplitLine, TextStyle,
        Tooltip, Trigger,
    },
    series::{Candlestick, Line},
};

use crate::grpc::types::StockPoint;

use super::types::{PlotFormat, Rendertype};

//TODO barchart
/*            df![ 120,
    DataPointItem::new(200).item_style(ItemStyle::new().color("#a90000")),
    150,
    80,
    70,
    110,
    130,
] */
pub fn bar_chart(x: Vec<&str>, y: DataFrame) -> Chart {
    Chart::new()
        .x_axis(Axis::new().type_(AxisType::Category).data(x))
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(Bar::new().data(y))
}

pub fn render_image(rt: Rendertype, chart: Chart) {
    // Chart dimension 1000x800.

    let mut renderer = HtmlRenderer::new("my charts", 1000, 800);
    // Render the chart as HTML string.
    let html_str = renderer.render(&chart).unwrap();
    // Save the chart as HTML file.
    renderer.save(&chart, "./assets/chart.html").unwrap();
}

pub fn stock_chart(dates: Vec<&str>, raw_data: Vec<StockPoint>, format: PlotFormat) -> Chart {
    //let mut raw_data = SOURCE;
    //raw_data.reverse();

    //let dates: Vec<&str> = raw_data.iter().map(|row| row[0]).collect();
    println!("dates: {:?}", dates);
    println!("dates: {:?}", raw_data);
    let data: Vec<Vec<f64>> = raw_data
        .iter()
        .map(|row| {
            vec![
                row.open, row.close, row.low,
                row.high,
                /* row[1].parse::<f64>().unwrap_or(0.),
                row[2].parse::<f64>().unwrap_or(0.),
                row[5].parse::<f64>().unwrap_or(0.),
                row[6].parse::<f64>().unwrap_or(0.), */
            ]
        })
        .collect();
    println!("data: {:?}", data);
    Chart::new()
        .legend(
            Legend::new()
                .inactive_color("#777")
                .data(vec!["日K", "MA5", "MA10", "MA20", "MA30"]),
        )
        .tooltip(
            Tooltip::new().trigger(Trigger::Axis).axis_pointer(
                AxisPointer::new()
                    .animation(false)
                    .type_(AxisPointerType::Cross)
                    .line_style(LineStyle::new().color("#376df4").width(2).opacity(1)),
            ),
        )
        .x_axis(Axis::new().type_(AxisType::Category).data(dates))
        .y_axis(
            Axis::new()
                .scale(true)
                .split_line(SplitLine::new().show(false)),
        )
        .grid(Grid::new().bottom(200))
        .data_zoom(
            DataZoom::new()
                .handle_icon(ICON)
                .text_style(TextStyle::new().color("#8392A5"))
                .data_background(
                    DataBackground::new()
                        .area_style(AreaStyle::new().color("#8392A5"))
                        .line_style(LineStyle::new().color("#8392A5").opacity(0.8)),
                )
                .brush_select(true),
        )
        .series(Candlestick::new().name("Day").data(data.clone()))
        .series(
            Line::new()
                .name("MA5")
                .data(calculate_ma(5, &data))
                .smooth(true)
                .show_symbol(false)
                .line_style(LineStyle::new().width(1)),
        )
        .series(
            Line::new()
                .name("MA10")
                .data(calculate_ma(10, &data))
                .smooth(true)
                .show_symbol(false)
                .line_style(LineStyle::new().width(1)),
        )
        .series(
            Line::new()
                .name("MA20")
                .data(calculate_ma(20, &data))
                .smooth(true)
                .show_symbol(false)
                .line_style(LineStyle::new().width(1)),
        )
        .series(
            Line::new()
                .name("MA30")
                .data(calculate_ma(30, &data))
                .smooth(true)
                .show_symbol(false)
                .line_style(LineStyle::new().width(1)),
        )
}

fn calculate_ma(day_count: usize, data: &[Vec<f64>]) -> Vec<CompositeValue> {
    let mut result = vec![];
    for i in 0..data.len() {
        if i < day_count {
            result.push("-".into());
            continue;
        }
        let mut sum = 0.;
        for j in 0..day_count {
            sum += data[i - j][1];
        }
        result.push((sum / day_count as f64).into());
    }

    result
}

static ICON: &str = "path://M10.7,11.9v-1.3H9.3v1.3c-4.9,0.3-8.8,4.4-8.8,9.4c0,5,3.9,9.1,8.8,9.4v1.3h1.3v-1.3c4.9-0.3,8.8-4.4,8.8-9.4C19.5,16.3,15.6,12.2,10.7,11.9z M13.3,24.4H6.7V23h6.6V24.4z M13.3,19.6H6.7v-1.4h6.6V19.6z";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bar_chart() {
        let x = vec!["日K", "MA5", "MA10", "MA20", "MA30"];
        let y = df![
            120,
            DataPointItem::new(200).item_style(ItemStyle::new().color("#a90000")),
            150,
            80,
            70,
            110,
            130,
        ];

        let char = bar_chart(x, y);

        render_image(Rendertype::HTML, char);
    }
}
