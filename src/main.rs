#![allow(dead_code)]
#[macro_use]
extern crate percent_encoding;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

extern crate log;
extern crate env_logger;

mod api;
mod common;
mod model;
use common::connection::Database;
use std::io;

fn main()-> io::Result<()> {
    let db = Database::new("root", "root", "common", "localhost", 5432);
    let pool = db.connection().expect("Cannot connect to database");
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let app = move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(
                web::resource("/clients")
                    .route(web::get().to_async(api::clients::index))
                    .route(web::post().to_async(api::clients::new_customer)),
            )
            .service(
                web::resource("/get_client").route(web::get().to_async(api::clients::get_by_id)),
            )
            .service(
                web::resource("/del_client").route(web::post().to_async(api::clients::del_by_id)),
            )
            .service(
                web::resource("/update_client")
                    .route(web::post().to_async(api::clients::update_customer)),
            )
    };
    HttpServer::new(app)
        .bind("localhost:8088")?
        .run()
}
