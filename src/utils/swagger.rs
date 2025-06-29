use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::routes::{
    environment_route, project_access_route, project_access_scopes_route, project_route,
    project_scope_route, service_account_route,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        project_route::post,
        project_route::get,
        project_route::patch,
        project_route::delete,
        project_route::list,
        service_account_route::post,
        service_account_route::get,
        service_account_route::patch,
        service_account_route::delete,
        service_account_route::list,
        project_scope_route::post,
        project_scope_route::get,
        project_scope_route::patch,
        project_scope_route::delete,
        project_scope_route::list,
        environment_route::post,
        environment_route::get,
        environment_route::patch,
        environment_route::delete,
        environment_route::list,
        project_access_route::post,
        project_access_route::get,
        project_access_route::patch,
        project_access_route::delete,
        project_access_route::list,
        project_access_scopes_route::post,
        project_access_scopes_route::get,
        project_access_scopes_route::patch,
        project_access_scopes_route::delete,
        project_access_scopes_route::list,
    ),
    tags(
        (name = "SentinelGuard", description = "SentinelGuard API documentation.")
    ),
)]
pub struct OpenApiDoc;

pub fn get_swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", OpenApiDoc::openapi())
}
