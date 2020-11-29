use actix_web::{get, web, App, HttpResponse, HttpServer};

use crate::{cli::Config, gpu::GPU};

pub struct Server {
    config: Config,
}

impl Server {
    pub fn new(config: Config) -> Self {
        Server { config }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let binding = format!("127.0.0.1:{}", self.config.server_port);
        let gpu = GPU::new(self.config.gpu_card_id).unwrap();
        HttpServer::new(move || App::new().data(gpu.clone()).service(index))
            .bind(binding)?
            .run()
            .await
    }
}

#[get("/")]
pub fn index(gpu: web::Data<GPU>) -> HttpResponse {
    HttpResponse::Ok().json(gpu.clone().get_status().unwrap())
}
