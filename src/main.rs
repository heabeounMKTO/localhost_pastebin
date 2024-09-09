use actix_files::NamedFile;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use serde_json::json;
use clap::Parser;
use actix_web::middleware::Logger;
use env_logger;

#[derive(Parser)]
struct CliArgs {
    #[arg(long)]
    host: Option<String>,
    
    #[arg(long)]
    port: Option<u32>
}

#[derive(Deserialize)]
struct TextData {
    text: String,
}

struct AppState {
    stored_text: Arc<Mutex<String>>,
}

async fn index() -> impl Responder {
    NamedFile::open("./index.html")
}
async fn update_text(data: web::Form<TextData>, state: web::Data<AppState>) -> impl Responder {
    let mut text = state.stored_text.lock().unwrap();
    *text = data.text.clone();
    HttpResponse::Ok().json(json!({
        "status": "success",
        "text": *text
    }))
}

// Route to get the current stored text
async fn get_text(state: web::Data<AppState>) -> impl Responder {
    let text = state.stored_text.lock().unwrap().clone();
    HttpResponse::Ok().json(json!({
        "text": text
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = CliArgs::parse();
    std::env::set_var("RUST_LOG", "actix_web=debug");
env_logger::init();
    let host = match &args.host {
        Some(ref _String) => args.host,
        None => Some(String::from("0.0.0.0")),
    };
    let port = match &args.port {
        Some(ref _u32) => args.port,
        None => Some(9990),
    };

    let app_state = web::Data::new(AppState {
        stored_text: Arc::new(Mutex::new(String::new())),
    });
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())             
            .route("/", web::get().to(index))             
            .route("/update_text", web::post().to(update_text)) 
            .route("/get_text", web::get().to(get_text)) 
            .wrap(Logger::default())
    })
    .bind(format!("{}:{}", host.unwrap(), port.unwrap()))?
    .run()
    .await
}
