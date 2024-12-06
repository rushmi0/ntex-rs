use ntex::web;

/// Handler for the "/{name}" endpoint
#[web::get("/{name}")]
pub async fn hello(name: web::types::Path<String>) -> impl web::Responder {
    format!("Hello {}!", &name)
}
