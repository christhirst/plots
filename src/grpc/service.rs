use proto::plot_server::Plot;

use crate::charms::image_charms::stock_chart;

use super::types::Actionss;

pub mod proto {
    tonic::include_proto!("plots"); // The string specified here must match the proto package name

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("plot_descriptor");
}

#[derive(Debug, Default)]
pub struct PlotsService {
    //state: State,
}

#[tonic::async_trait]
impl Plot for PlotsService {
    async fn svg_chart(
        &self,
        request: tonic::Request<proto::ChartRequest>,
    ) -> Result<tonic::Response<proto::DataResponse>, tonic::Status> {
        // Your implementation here
        //todo!("Not yet implemented")
        let r = request.get_ref().clone();

        //eee("y=x^2", (-1f32..6f32, -1f32..10f32), ww, "y = x^2");

        Ok(tonic::Response::new(proto::DataResponse {
            path: "buffer/2323232.svg".to_string(),
            file: "svg".to_string(),
        }))
    }

    async fn html_chart(
        &self,
        request: tonic::Request<proto::ChartRequest>,
    ) -> Result<tonic::Response<proto::DataResponse>, tonic::Status> {
        // Your implementation here
        //todo!("Not yet implemented")
        let r: Actionss = request.get_ref().clone().try_into().unwrap();
        let x = r.x.unwrap();
        let x: Vec<&str> = x.iter().map(AsRef::as_ref).collect();
        stock_chart(x, r.y.unwrap());

        Ok(tonic::Response::new(proto::DataResponse {
            path: "buffer/2323232.svg".to_string(),
            file: "svg".to_string(),
        }))
    }

    // Implement other required methods
}
