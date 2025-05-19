use actix_web::web;
use crate::hello;
use crate::handers::{ warehouse,sku,inventory };

pub fn configure_services(cfg: &mut web::ServiceConfig) {
    cfg.service(hello)
        .service(inventory::import_excel_to_db)
        .service(sku::skus)
        .service(sku::create_sku)
        .service(sku::get_sku)
        .service(warehouse::warehouse)
        .service(inventory::inventories);
}