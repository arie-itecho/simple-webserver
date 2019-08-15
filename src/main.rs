use actix_files::NamedFile;
use actix_web::{web, App, Responder, HttpServer, HttpRequest};

use std::path::PathBuf;

fn index() -> impl Responder {
    find_file("/index.html")
}

fn static_file(name: HttpRequest) -> impl Responder {
    find_file(name.path())
}

fn find_file(path: &str) -> impl Responder {
    let in_path = path.parse::<PathBuf>().unwrap();
    let mut final_path = "frontend".parse::<PathBuf>().unwrap();
    final_path.push(in_path.strip_prefix("/").unwrap());

    match NamedFile::open(&final_path) {
        Ok(x) => Ok(x),
        Err(x) => {
            println!("Error finding file: {}", final_path.display());
            println!("{}", x);
            Err(x)
        }
    }
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
