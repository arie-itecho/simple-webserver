use actix_files::NamedFile;
use actix_web::{error, web, App, HttpRequest, HttpServer, Result};

use std::path::PathBuf;

async fn index() -> Result<NamedFile> {
    find_file("index.html").await
}

async fn static_file(name: HttpRequest) -> Result<NamedFile> {
    find_file(&name.path()[1..]).await
}

async fn find_file(path: &str) -> Result<NamedFile> {
    let mut final_path = "frontend".parse::<PathBuf>().unwrap();
    final_path.push(path.parse::<PathBuf>().unwrap());

    match NamedFile::open(&final_path) {
        Ok(x) => Ok(x),
        Err(x) => {
            println!("Error finding file: {}", final_path.display());
            println!("{}", x);
            Err(error::ErrorNotFound("File Not Found"))
        }
    }
}

async fn serve(address: &str) {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/scripts/.*", web::get().to(static_file))
            .route("/styles/.*", web::get().to(static_file))
            .route("/dist/.*", web::get().to(static_file))
    })
    .bind(address)
    .unwrap()
    .run()
    .await
    .unwrap();
}

#[actix_rt::main]
async fn main() {
    let address = "127.0.0.1:8808";
    let server = serve(address);
    println!("Listening on {a}", a = address);
    server.await
}
