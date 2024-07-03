use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use askama_actix::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate;

#[derive(Template)]
#[template(path = "blog.html")]
struct BlogLandingTemplate;

#[derive(Template)]
#[template(path = "mac.html")]
struct MacTemplate;

#[get("/")]
async fn index() -> impl Responder {
    let index_page = IndexTemplate;
    HttpResponse::Ok().body(index_page.render().unwrap())
}

#[get("/about")]
async fn about() -> impl Responder {
    let about_page = AboutTemplate;
    HttpResponse::Ok().body(about_page.render().unwrap())
}

#[get("/blog")]
async fn blog() -> impl Responder {
    let blog_landing = BlogLandingTemplate;
    HttpResponse::Ok().body(blog_landing.render().unwrap())
}

#[get("/mac")]
async fn mac() -> impl Responder {
    let mac = MacTemplate;
    HttpResponse::Ok().body(mac.render().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(about)
            .service(blog)
            .service(mac)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
