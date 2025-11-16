use actix_web::{web, App, HttpServer, HttpResponse, Result};
use serde_json::json;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Pepakura Next API Gateway starting on 0.0.0.0:8080...");
    
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/api/status", web::get().to(get_status))
            .route("/api/proxy/ai", web::post().to(proxy_ai))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "ok",
        "service": "api_gateway",
        "version": "1.0.0"
    })))
}

async fn get_status() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "service": "Pepakura Next Gateway",
        "version": "1.0.0",
        "ai_engine": "http://localhost:9000",
        "frontend": "http://localhost:3000"
    })))
}

async fn proxy_ai(body: web::Bytes) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "proxied",
        "message": "Request forwarded to AI Engine"
    })))
}
