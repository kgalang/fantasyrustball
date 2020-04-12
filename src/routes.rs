//! Place all Actix routes here, multiple route configs can be used and
//! combined.

use crate::handlers::{
    health::get_health,
    user::{create_user, delete_user, get_user, get_users, update_user},
};
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Healthcheck
        .route("/health", web::get().to(get_health))
        // /api/v1 routes
        .service(
            web::scope("/api/v1")
                // USER routes
                .service(
                    web::scope("/user")
                        .route("/{id}", web::get().to(get_user))
                        .route("/{id}", web::put().to(update_user))
                        .route("/{id}", web::delete().to(delete_user))
                        .route("", web::get().to(get_users))
                        .route("", web::post().to(create_user)),
                ),
        );
}
