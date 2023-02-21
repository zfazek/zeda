use actix_web::{web, HttpResponse, Responder};
use rusqlite::Connection;
use sailfish::TemplateOnce;

use crate::constant::*;
use crate::candidate::*;

#[derive(TemplateOnce)]
#[template(path = "add.stpl")]
struct Empty {}

pub async fn add() -> impl Responder {
    HttpResponse::Ok().body(Empty {}.render_once().unwrap())
}

pub async fn add_to_candidates(candidate: web::Form<Candidate>) -> impl Responder {
    let conn = Connection::open(DB_PATH).unwrap();
    match conn.execute(
        "INSERT INTO candidate (name, email, phone)
         VALUES (?1, ?2, ?3)",
        &[&candidate.name, &candidate.email, &candidate.phone],
    ) {
        Ok(updated) => println!("{} rows were inserted", updated),
        Err(err) => println!("{} error occured", err),
    }
    HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish()
}
