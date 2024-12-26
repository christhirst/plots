use std::{env, thread};
//use proto::admin_server::{Admin, AdminServer};

mod charms;
mod grpc;
mod web;
use charms::image_charms::{render_image, stock_chart};
use charms::types::Rendertype;
use web::web::{serve, two_serve_dirs};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    //thread::spawn(|| tokio::join!(serve(two_serve_dirs(), 3005)));
    tokio::spawn(async move { serve(two_serve_dirs(), 3005).await });
    grpc::server::serve().await;

    Ok(())
}
