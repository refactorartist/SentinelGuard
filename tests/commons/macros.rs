#[macro_export]
macro_rules! create_test_app {
    ($services:expr, $routes:expr) => {{
        let mut app = actix_web::App::new();

        for service in $services {
            app = app.app_data(actix_web::web::Data::new(service));
        }

        for route in $routes {
            app = app.configure(route);
        }

        actix_web::test::init_service(app).await
    }};
}