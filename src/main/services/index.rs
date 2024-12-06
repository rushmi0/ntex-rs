use ntex::web;

/// Handler for the root ("/") endpoint
#[web::get("/")]
pub async fn index() -> impl web::Responder {
    "Hello, World!"
}
