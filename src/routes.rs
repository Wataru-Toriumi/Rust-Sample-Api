use std::sync::Mutex;
use actix_web::web;
use crate::handlers::*;

pub fn init(cfg: &mut web::ServiceConfig) {
    let db: web::Data<Db> = web::Data::new(Mutex::new(vec![]));

    cfg.app_data(db.clone())
        .service(
            web::scope("/books")
                .route("", web::get().to(get_books))
                .route("", web::post().to(create_book))
                .route("/{id}", web::get().to(get_book))
                .route("/{id}", web::put().to(update_book))
                .route("/{id}", web::delete().to(delete_book)),
        );
}