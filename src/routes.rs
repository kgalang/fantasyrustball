//! Place all Actix routes here, multiple route configs can be used and
//! combined.

use crate::handlers::{
    health::get_health,
    leagues::{create_league, get_league, get_leagues},
    users::{create_user, delete_user, get_user, get_users, update_user},
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
                    web::scope("/users")
                        .route("/{id}", web::get().to(get_user))
                        .route("/{id}", web::put().to(update_user))
                        .route("/{id}", web::delete().to(delete_user))
                        .route("", web::get().to(get_users))
                        .route("", web::post().to(create_user)),
                )
                // LEAGUE routes
                .service(
                    web::scope("/leagues")
                        .route("/{id}", web::get().to(get_league))
                        .route("", web::get().to(get_leagues))
                        .route("", web::post().to(create_league)),
                ),
        );
}
