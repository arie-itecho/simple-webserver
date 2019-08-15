use actix_files::NamedFile;
use actix_web::{web, App, Responder, HttpServer, HttpRequest};

use std::path::PathBuf;

fn index() -> impl Responder {
    NamedFile::open("frontend/index.html")
}

fn static_file(name: HttpRequest) -> impl Responder {
    let in_path = name.match_info().path()
        .parse::<PathBuf>().unwrap();
    let mut path = "frontend".parse::<PathBuf>().unwrap();
    path.push(in_path.strip_prefix("/").unwrap());
    NamedFile::open(path)
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::to(index))
            .route("/scripts/.*", web::to(static_file))
            .route("/styles/.*", web::to(static_file))
    })
    .bind("127.0.0.1:8808")
    .unwrap()
    .run()
    .unwrap();
}
