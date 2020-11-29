use actix_web::{get, App, HttpServer, Responder};

pub struct Server {
    port: u32,
}

impl Server {
    pub fn new(port: u32) -> Self {
        Server { port }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let binding = format!("127.0.0.1:{}", self.port);
        HttpServer::new(|| App::new().service(index))
            .bind(binding)?
            .run()
            .await
    }
}

#[get("/")]
pub async fn index() -> impl Responder {
    format!("Hello from Linkux Server!")
}
