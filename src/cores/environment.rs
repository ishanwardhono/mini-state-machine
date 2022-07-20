pub fn set_env() {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
}