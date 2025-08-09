#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use actix_files as fs;
use actix_web::{error, web, App, Error, HttpResponse, HttpServer, Result};
use actix_web::web::Data;
use tera::Tera;



#[actix_web::main]
async
fn main() -> std::io::Result<()> {
    println!("Listening on: 127.0.0.1:8080, open browser and visit have a try!");
    // 프로젝트 경로  D:\gitroot\rust_projects\scenario2\fend
    HttpServer::new(|| {
        // let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*")).unwrap();
        // let tera = Tera::new(concat!(env!(path), "/static/**/*")).unwrap();
        let tera = Tera::new("D:/gitroot/rust_projects/scenario2/fend/static/**/*").unwrap();

        App::new()
        .app_data(Data::new(tera))
        .service(web::resource("/").route(web::get().to(index)))
        .service(web::resource("/auth/").route(web::get().to(login)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async
fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("name", "김가을");
    let s = tmpl
        .render("index.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async
fn login(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("name", "김가을");
    let s = tmpl
        .render("auth/login.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}