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

    /* let mut dates = Vec::new();
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
    */
    //let chart = stock_chart(dates, data);
    //render_image(Rendertype::HTML, chart);

    //thread::spawn(|| tokio::join!(serve(two_serve_dirs(), 3005)));
    tokio::spawn(async move { serve(two_serve_dirs(), 3005).await });
    grpc::server::serve().await;

    Ok(())
}
