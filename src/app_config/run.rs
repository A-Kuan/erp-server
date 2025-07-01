use actix_web::web;
use crate::hello;
use crate::handers::{ sku };

pub fn configure_services(cfg: &mut web::ServiceConfig) {
    cfg.service(hello)
        .service(sku::skus);

}