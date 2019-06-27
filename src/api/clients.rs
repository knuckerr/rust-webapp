use crate::common::connection;
use crate::model::customer;
use actix_web::{web, Error, HttpResponse};
use actix_web::web::{Json,Query};
use futures::future::Future;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct ClientQuery {
   id: i32,
}

pub fn index(
    pool: web::Data<connection::PgPool>) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || customer::get_customers(&pool))
        .from_err()
        .then(move |res| match res {
            Ok(tasks) => {

                Ok(HttpResponse::Ok().json(tasks))
            }
            Err(e) => Err(e),
        })
}

pub fn get_by_id(
    pool: web::Data<connection::PgPool>,clientquery:Query<ClientQuery>) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || customer::get_customer_by_id(&pool,clientquery.id))
        .from_err()
        .then(move |res| match res {
            Ok(tasks) => {

                Ok(HttpResponse::Ok().json(tasks))
            }
            Err(e) => Err(e),
        })
}

pub fn del_by_id(
    pool: web::Data<connection::PgPool>,clientquery:Query<ClientQuery>) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || customer::del_customer_by_id(&pool,clientquery.id))
        .from_err()
        .then(move |res| match res {
            Ok(tasks) => {

                Ok(HttpResponse::Ok().json(tasks))
            }
            Err(e) => Err(e),
        })
}


pub fn new_customer(
    pool: web::Data<connection::PgPool>,customer:Json<customer::Costumer>) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || customer::new_customer(&pool,&customer))
        .from_err()
        .then(move |res| match res {
            Ok(msg) => {

                Ok(HttpResponse::Ok().json(msg))
            }
            Err(e) => Err(e),
        })
}

pub fn update_customer(
    pool: web::Data<connection::PgPool>,customer:Json<customer::Costumer>,query:Query<ClientQuery>) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || customer::update_customer(&pool,&customer,query.id))
        .from_err()
        .then(move |res| match res {
            Ok(msg) => {

                Ok(HttpResponse::Ok().json(msg))
            }
            Err(e) => Err(e),
        })
}

