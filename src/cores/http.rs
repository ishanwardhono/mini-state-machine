pub mod middleware {
    pub mod auth;
    pub mod span;
}

pub mod entity {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct ErrorResponse {
        pub error: String,
        pub message: String,
    }
}
