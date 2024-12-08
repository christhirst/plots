use proto::plot_server::Plot;
use std::env;
//use proto::admin_server::{Admin, AdminServer};

mod charms;
mod grpc;
mod web;
use charms::image_charms::{charta, render_image};
use charms::types::Rendertype;
use web::{serve, two_serve_dirs};

mod proto {
    tonic::include_proto!("plots");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("plot_descriptor");
}

#[derive(Debug, Default)]
struct PlotsService {
    //state: State,
}

#[tonic::async_trait]
impl Plot for PlotsService {
    async fn svg_chart(
        &self,
        request: tonic::Request<proto::ChartRequest>,
    ) -> Result<tonic::Response<proto::SvgResponse>, tonic::Status> {
        // Your implementation here
        //todo!("Not yet implemented")
        let r = request.get_ref().clone().y;

        //eee("y=x^2", (-1f32..6f32, -1f32..10f32), ww, "y = x^2");

        Ok(tonic::Response::new(proto::SvgResponse {
            path: "buffer/2323232.svg".to_string(),
            file: "svg".to_string(),
        }))
    }

    // Implement other required methods
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    let mut dates = Vec::new();
    dates.push("2015/12/31");
    dates.push("2016/01/01");
    dates.push("2015/01/02");
    dates.push("2016/01/03");
    dates.push("2015/01/04");
    dates.push("2016/01/05");
    dates.push("2015/01/06");
    dates.push("2016/01/07");
    let mut data = Vec::new();
    let mut dd = Vec::new();
    dd.push(3570.47);
    dd.push(3539.18);
    dd.push(3538.35);
    dd.push(3580.6);
    data.push(dd);
    let mut dd = Vec::new();
    dd.push(3566.73);
    dd.push(3572.88);
    dd.push(3538.11);
    dd.push(3573.68);
    data.push(dd);
    let mut dd = Vec::new();
    dd.push(3570.47);
    dd.push(3539.18);
    dd.push(3538.35);
    dd.push(3580.6);
    data.push(dd);
    let mut dd = Vec::new();
    dd.push(3566.73);
    dd.push(3572.88);
    dd.push(3538.11);
    dd.push(3573.68);
    data.push(dd);
    let mut dd = Vec::new();
    dd.push(3570.47);
    dd.push(3539.18);
    dd.push(3538.35);
    dd.push(3580.6);
    data.push(dd);
    let mut dd = Vec::new();
    dd.push(3566.73);
    dd.push(3572.88);
    dd.push(3538.11);
    dd.push(3573.68);
    data.push(dd);
    let mut dd = Vec::new();
    dd.push(3566.73);
    dd.push(3572.88);
    dd.push(3538.11);
    dd.push(3573.68);
    data.push(dd);
    let mut dd = Vec::new();
    dd.push(3566.73);
    dd.push(3572.88);
    dd.push(3538.11);
    dd.push(3573.68);
    data.push(dd);

    let chart = charta(dates, data);
    render_image(Rendertype::HTML, chart);

    tokio::join!(serve(two_serve_dirs(), 3005));

    Ok(())
}
