use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::App;

use crate::routes::{
    environment_route, project_access_route, project_access_scopes_route, project_route,
    project_scope_route, service_account_route,
};


pub fn register_routes<T>(app: App<T>) -> App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Response = ServiceResponse, Error = actix_web::Error, InitError = ()>,
{
    let routes = [
        project_route::configure_routes,
        service_account_route::configure_routes,
        project_scope_route::configure_routes,
        environment_route::configure_routes,
        project_access_route::configure_routes,
        project_access_scopes_route::configure_routes,
    ];

    app.configure(|config| {
        for route in routes {
            route(config);
        }
    })
}