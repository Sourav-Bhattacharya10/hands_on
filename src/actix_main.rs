// use actix_web::{App, HttpRequest, HttpServer, Responder, web};
// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", name)
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         println!("Server started at http://0.0.0.0:8080");
//         App::new()
//             .route("/", web::get().to(greet))
//             .route("/{name}", web::get().to(greet))
//             .route("/say/hello", web::get().to(|| async { "Hello Again!" }))
//     })
//     .bind("0.0.0.0:8080")?
//     .run()
//     .await
// }

use actix_web::{App, HttpServer, Responder, web};
use futures::future;

async fn utils_one() -> impl Responder {
    "Utils one reached\n"
}

async fn health() -> impl Responder {
    "All good\n"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let s1 = HttpServer::new(move || {
        println!("Server1 started at http://0.0.0.0:3006");
        App::new().service(web::scope("/utils").route("/one", web::get().to(utils_one)))
    })
    .bind("0.0.0.0:3006")?
    .run();
    let s2 = HttpServer::new(move || {
        println!("Server2 started at http://0.0.0.0:8080");
        App::new().service(web::resource("/health").route(web::get().to(health)))
    })
    .bind("0.0.0.0:8080")?
    .run();
    future::try_join(s1, s2).await?;
    Ok(())
}
