use actix_web::{web, HttpResponse, Responder};
use rusqlite::Connection;
use sailfish::TemplateOnce;

use crate::candidate::*;
use crate::constant::*;

#[derive(TemplateOnce)]
#[template(path = "update.stpl")]
struct List1 {
    c: Candidate,
}

pub async fn update(id: web::Path<usize>) -> impl Responder {
    let conn = Connection::open(DB_PATH).unwrap();
    let mut stmt = conn
        .prepare("SELECT id, name, email, phone FROM candidate WHERE id = ?")
        .unwrap();
    let mut rows = stmt.query([id.to_owned()]).unwrap();
    let mut c = Candidate {
        id: "".to_string(),
        name: "".to_string(),
        email: "".to_string(),
        phone: "".to_string(),
    };
    while let Some(row) = rows.next().unwrap() {
        let id: usize = row.get(0).unwrap();
        let id = id.to_string();
        let name = row.get(1).unwrap();
        let email = row.get(2).unwrap();
        let phone = row.get(3).unwrap();
        let c_tmp = Candidate {
            id,
            name,
            email,
            phone,
        };
        c = c_tmp;
    }
    HttpResponse::Ok().body(List1 { c }.render_once().unwrap())
}

pub async fn update_candidate(candidate: web::Form<Candidate>) -> impl Responder {
    let conn = Connection::open(DB_PATH).unwrap();
    match conn.execute(
        "UPDATE candidate
         SET name = ?2, email = ?3, phone = ?4
         WHERE id = ?1",
        &[
            &candidate.id.to_owned(),
            &candidate.name.to_owned(),
            &candidate.email.to_owned(),
            &candidate.phone.to_owned(),
        ],
    ) {
        Ok(updated) => println!("{} rows were updated", updated),
        Err(err) => println!("{} error occured", err),
    }
    HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish()
}
