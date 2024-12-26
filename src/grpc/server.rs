use tonic::transport::Server;

use crate::grpc::service::{
    proto::{self, plot_server::PlotServer},
    PlotsService,
};

pub async fn serve() {
    let addr = "[::1]:50051".parse().unwrap();
    //GRPC server
    let calc = PlotsService::default();
    //GRPC reflection
    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .accept_http1(true)
        //.layer(tower_http::cors::CorsLayer::permissive())
        .add_service(service)
        .add_service(PlotServer::new(calc))
        //.add_service(tonic_web::enable(CalculatorServer::new(calc)))
        //.add_service(AdminServer::with_interceptor(admin, check_auth))
        .serve(addr)
        .await
        .unwrap();

    println!("Hello, world!");

    /* let ww = vec![(0.0, -1.0), (1.0, 1.0), (2.0, 4.0), (3.0, 5.0), (5.0, 9.0)];
       eee("y=x^2", (-1f32..6f32, -1f32..10f32), ww, "y = x^2");

       // --snip--
       println!("In file 2323232.svg");
       fs::create_dir_all("./buffer")?;
       let contents =
           fs::read_to_string("2323232.svg").expect("Should have been able to read the file");

    println!("With text:\n{contents}");
    */
}
