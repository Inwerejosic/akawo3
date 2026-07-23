use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use std::time::{SystemTime, UNIX_EPOCH};

fn format_log_entry(message: &str) -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    format!("[{timestamp}] {message}")
}

fn log_message(message: &str) {
    println!("{}", format_log_entry(message));
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    let path = req.path().to_string();
    log_message(&format!("Handling request for {path} with name={name}"));
    format!("Hello {}!", &name)
}

async fn health_check() -> HttpResponse {
    log_message("Health check requested");
    HttpResponse::Ok().finish()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    log_message("Server starting on http://127.0.0.1:8000");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health", web::get().to(health_check))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}


#[cfg(test)]
mod tests {
    use crate::{format_log_entry, health_check};

    #[tokio::test]
    async fn health_check_success() {
        let response = health_check().await;

        assert!(response.status().is_success())
    }

    #[test]
    fn log_entry_contains_message_and_timestamp() {
        let line = format_log_entry("startup");

        assert!(line.contains("startup"));
        assert!(line.starts_with("["));
    }
}
