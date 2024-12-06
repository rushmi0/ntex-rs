mod app;
mod services;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    app::run().await
}
