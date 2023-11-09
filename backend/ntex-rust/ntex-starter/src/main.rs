use ntex::web::{self, App, HttpServer};

#[ntex::main]
async fn main() -> std::io::Result<()> {
    // Configure ur port using the PORT environment variable
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a valid number");

    HttpServer::new(|| {
        App::new()
            .configure(routes)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").to(hello));
}

async fn hello() -> &'static str {
    "Hello, World!"
}
