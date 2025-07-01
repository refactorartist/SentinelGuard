pub mod repositories;
pub mod routes;

// Define the macro here instead of including it from a file
#[macro_export]
macro_rules! create_test_app {
    ($repository:expr, $routes:expr) => {{
        let mut app = actix_web::App::new();

        app = app.app_data(actix_web::web::Data::new($repository));
        app = app.configure($routes);

        actix_web::test::init_service(app).await
    }};
}
