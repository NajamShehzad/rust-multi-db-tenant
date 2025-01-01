use actix_web::HttpRequest;

pub fn extract_db_name(req: &HttpRequest) -> String {
    req.headers()
        .get("_db")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("default_db")
        .to_string()
}