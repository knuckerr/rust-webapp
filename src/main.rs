#![allow(dead_code)]
#[macro_use] extern crate percent_encoding;
use actix_web::{App,web,HttpServer};
use actix_web::middleware::Logger;

mod model;
mod common;
mod api;
use common::connection::Database;


fn main() {
    let db = Database::new("root","root","common","localhost",5432);
    let pool = db.connection().expect("Cannot connect to database");
    let app = move || {
          App::new()
              .data(pool.clone())
              .wrap(Logger::default())
              .service(web::resource("/clients")
                       .route(web::get().to_async(api::clients::index))
                       .route(web::post().to_async(api::clients::new_customer)))

              .service(web::resource("/get_client").route(web::get().to_async(api::clients::get_by_id)))
    };
    dbg!("Constructing the App");
    HttpServer::new(app).bind("localhost:8088").expect("cannot_bind").run().expect("Error running server");
}


