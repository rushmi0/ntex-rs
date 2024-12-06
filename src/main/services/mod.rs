pub mod index;
pub mod hello;

use ntex::web::ServiceConfig;

pub fn init_services(cfg: &mut ServiceConfig) {
    cfg.service(index::index);
    cfg.service(hello::hello);
}
