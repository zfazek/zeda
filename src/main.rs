use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rusqlite::Connection;
use sailfish::TemplateOnce;

mod add_candidate;
mod candidate;
mod constant;
mod update_candidate;

use crate::add_candidate::*;
use crate::candidate::*;
use crate::constant::*;
use crate::update_candidate::*;

#[derive(TemplateOnce)]
#[template(path = "home.stpl")]
struct List {
    candidates: Vec<Candidate>,
}

async fn get_candidates() -> impl Responder {
    let conn = Connection::open(DB_PATH).unwrap();
    let mut stmt = conn
        .prepare("SELECT id, name, email, phone FROM candidate")
        .unwrap();
    let mut rows = stmt.query([]).unwrap();
    let mut candidates: Vec<Candidate> = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let id: usize = row.get(0).unwrap();
        let id = id.to_string();
        let name = row.get(1).unwrap();
        let email = row.get(2).unwrap();
        let phone = row.get(3).unwrap();
        let c = Candidate {
            id,
            name,
            email,
            phone,
        };
        candidates.push(c);
    }
    HttpResponse::Ok().body(List { candidates }.render_once().unwrap())
}

async fn delete_candidate(id: web::Path<usize>) -> impl Responder {
    let conn = Connection::open(DB_PATH).unwrap();
    match conn.execute("DELETE FROM candidate WHERE id = (?1)", &[&id.to_owned()]) {
        Ok(updated) => println!("{} rows were deleted", updated),
        Err(err) => println!("{} error occured", err),
    }
    HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish()
}

#[actix_web::main]
async fn main() {
    let conn = Connection::open(DB_PATH).unwrap();
    conn.execute("PRAGMA encoding = 'UTF-8'", ()).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS candidate (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            email TEXT NOT NULL,
            phone TEXT NOT NULL
        )",
        (),
    )
    .unwrap();
    let addr = "0.0.0.0:8081";
    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(get_candidates))
            .route("/add", web::get().to(add))
            .route("/add_c", web::post().to(add_to_candidates))
            .service(web::resource("/update/{id}").route(web::get().to(update)))
            .route("/update_c", web::post().to(update_candidate))
            .service(web::resource("/delete/{id}").route(web::get().to(delete_candidate)))
            .service(Files::new("/static", "./static"))
    })
    .bind(addr)
    .unwrap()
    .run();
    println!("Server live at http://{}", addr);
    server.await.unwrap();
}
