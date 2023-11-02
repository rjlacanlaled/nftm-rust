mod strings;
mod pages;
mod components;

use actix_files::Files as ActixFiles;
use actix_web::web::route;
use actix_web::{ App, HttpServer };
use pages::not_found::not_found;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(pages::home::page)
            .service(pages::collections::page)
            .service(pages::collection::page)
            .service(pages::assets::page)
            .service(pages::asset::page)
            .service(ActixFiles::new("/", "./src/static").prefer_utf8(true))
            .default_service(route().to(not_found))
    })
        .bind(("127.0.0.1", 8080))?
        .run().await
}
