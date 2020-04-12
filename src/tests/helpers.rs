#[cfg(test)]
pub mod tests {
    use crate::config::CONFIG;
    use crate::database::{add_pool, init_pool, Pool};
    use crate::routes::routes;
    use actix_web::dev::ServiceResponse;
    use actix_web::{test, web::Data, App};
    use diesel::pg::PgConnection;
    use serde::Serialize;

    /// Helper for HTTP GET integration tests
    pub async fn test_get(route: &str) -> ServiceResponse {
        let mut app = test::init_service(
            App::new()
                .configure(add_pool)
                .configure(routes),
        )
        .await;

        test::call_service(
            &mut app,
            test::TestRequest::get()
                .uri(route)
                .to_request(),
        )
        .await
    }

    /// Helper for HTTP POST integration tests
    pub async fn test_post<T: Serialize>(route: &str, params: T) -> ServiceResponse {
        let mut app = test::init_service(
            App::new()
                .configure(add_pool)
                .configure(routes),
        )
        .await;

        test::call_service(
            &mut app,
            test::TestRequest::post()
                .set_json(&params)
                .uri(route)
                .to_request(),
        )
        .await
    }

    /// Helper to login for tests
    // pub fn login_request() -> Request {
    //     let login_request = LoginRequest {
    //         email: "satoshi@nakamotoinstitute.org".into(),
    //         password: "123456".into(),
    //     };
    //     test::TestRequest::post()
    //         .set_json(&login_request)
    //         .uri("/api/v1/auth/login")
    //         .to_request()
    // }

    /// Assert that a route is successful for HTTP GET requests
    pub async fn assert_get(route: &str) -> ServiceResponse {
        let response = test_get(route).await;
        assert!(response.status().is_success());
        response
    }

    /// Assert that a route is successful for HTTP POST requests
    pub async fn assert_post<T: Serialize>(route: &str, params: T) -> ServiceResponse {
        let response = test_post(route, params).await;
        assert!(response.status().is_success());
        response
    }

    /// Returns a r2d2 Pooled Connection to be used in tests
    pub fn get_pool() -> Pool<PgConnection> {
        init_pool::<PgConnection>(CONFIG.clone()).unwrap()
    }

    /// Returns a r2d2 Pooled Connection wrappedn in Actix Application Data
    pub fn get_data_pool() -> Data<Pool<PgConnection>> {
        Data::new(get_pool())
    }
}
