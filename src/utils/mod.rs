use actix_web::HttpRequest;
use peak_alloc::PeakAlloc;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;


pub fn extract_db_name(req: &HttpRequest) -> String {
    req.headers()
        .get("_db")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("default_db")
        .to_string()
}




pub fn log_memory_usage() {
    let current_mem = PeakAlloc.current_usage_as_mb();
	println!("This program currently uses {} MB of RAM.", current_mem);
	let peak_mem = PeakAlloc.peak_usage_as_gb();
	println!("The max amount that was used {}", peak_mem);
}