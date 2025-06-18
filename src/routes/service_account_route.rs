use crate::models::pagination::Pagination;
use crate::models::service_account::{
    ServiceAccountCreatePayload, ServiceAccountFilter, ServiceAccountResponse,
    ServiceAccountSortOrder, ServiceAccountSortableFields, ServiceAccountUpdatePayload,
};
use crate::models::sort::SortOrder;
use crate::services::base::Service;
use crate::services::service_account_service::ServiceAccountService;
use actix_web::{Error, HttpResponse, web};

#[utoipa::path(
    post,
    path = "/service-accounts",
    tag = "Service Accounts",
    request_body = ServiceAccountCreatePayload,
    responses(
        (status = 201, description = "Service account created", body = ServiceAccountResponse),
        (status = 400, description = "Invalid request", body = String),
    ),
)]
pub async fn post(
    service: web::Data<ServiceAccountService>,
    payload: web::Json<ServiceAccountCreatePayload>,
) -> Result<HttpResponse, Error> {
    let service_account = service
        .create(payload.into_inner())
        .await
        .map_err(actix_web::error::ErrorBadRequest)?;
    Ok(HttpResponse::Created().json(service_account))
}

#[utoipa::path(
    get,
    path = "/service-accounts/{id}",
    tag = "Service Accounts",
    responses(
        (status = 200, description = "Service account found", body = ServiceAccountResponse),
        (status = 404, description = "Service account not found", body = String),
    ),
    params(
        ("id" = String<uuid::Uuid>, Path, description = "Project ID"),
    ),
)]
pub async fn get(
    service: web::Data<ServiceAccountService>,
    id: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, Error> {
    let project = service
        .read(id.into_inner())
        .await
        .map_err(actix_web::error::ErrorNotFound)?;
    Ok(HttpResponse::Ok().json(project))
}

#[utoipa::path(
    patch,
    path = "/service-accounts/{id}",
    tag = "Service Accounts",
    responses(
        (status = 200, description = "Service account updated", body = ServiceAccountResponse),
        (status = 400, description = "Invalid request", body = String),
        (status = 404, description = "Service account not found", body = String),
        (status = 409, description = "Service account name already exists", body = String),
        (status = 409, description = "Service account email already exists", body = String),
        (status = 500, description = "Internal server error", body = String),
    ),
    params(
        ("id" = String<uuid::Uuid>, Path, description = "Project ID"),
    ),
)]
pub async fn patch(
    service: web::Data<ServiceAccountService>,
    id: web::Path<uuid::Uuid>,
    payload: web::Json<ServiceAccountUpdatePayload>,
) -> Result<HttpResponse, Error> {
    let project = service.update(id.into_inner(), payload.into_inner()).await;

    if project.is_err() {
        let error_message = project.unwrap_err().to_string();
        match error_message.as_str() {
            "No changes to update" => return Err(actix_web::error::ErrorBadRequest(error_message)),
            "Service account not found" => return Err(actix_web::error::ErrorNotFound(error_message)),
            "Service account name already exists" => {
                return Err(actix_web::error::ErrorConflict(error_message));
            },
            "Service account email already exists" => {
                return Err(actix_web::error::ErrorConflict(error_message));
            }
            _ => return Err(actix_web::error::ErrorInternalServerError(error_message)),
        }
    }

    Ok(HttpResponse::Ok().json(project.unwrap()))
}

#[utoipa::path(
    delete,
    path = "/service-accounts/{id}",
    tag = "Service Accounts",
    responses(
        (status = 204, description = "Service account deleted", body = ()),
        (status = 404, description = "Service account not found", body = String),
    ),
    params(
        ("id" = String<uuid::Uuid>, Path, description = "Service account ID"),
    )
)]
pub async fn delete(
    service: web::Data<ServiceAccountService>,
    id: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, Error> {
    let result = service.delete(id.into_inner()).await;

    match result {
        Ok(true) => Ok(HttpResponse::NoContent().finish()),
        Ok(false) => Err(actix_web::error::ErrorNotFound("Project not found")),
        Err(error) => Err(actix_web::error::ErrorInternalServerError(error)),
    }
}

#[utoipa::path(
    get,
    path = "/projects",
    tag = "Projects",
    responses(
        (status = 200, description = "Projects found", body = Vec<ServiceAccountResponse>),
    ),
    params(
        ("name" = Option<String>, Query, description = "Filter projects by name"),
        ("description" = Option<String>, Query, description = "Filter projects by description"),
        ("enabled" = Option<bool>, Query, description = "Filter projects by enabled"),
        ("offset" = Option<u32>, Query, description = "Offset for pagination"),
        ("limit" = Option<u32>, Query, description = "Number of items per page"),
    )
)]
pub async fn list(
    service: web::Data<ServiceAccountService>,
    filter: web::Query<ServiceAccountFilter>,
    pagination: web::Query<Pagination>,
) -> Result<HttpResponse, Error> {
    let sort = vec![ServiceAccountSortOrder::new(
        ServiceAccountSortableFields::Id,
        SortOrder::Asc,
    )];
    let projects = service
        .find(
            filter.into_inner(),
            Some(sort),
            Some(pagination.into_inner()),
        )
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(projects))
}

pub fn configure_routes(config: &mut actix_web::web::ServiceConfig) {
    config.service(
        web::scope("/service-accounts")
            .service(
                actix_web::web::resource("")
                    .route(actix_web::web::post().to(post))
                    .route(actix_web::web::get().to(list)),
            )
            .service(
                actix_web::web::resource("/{id}")
                    .route(actix_web::web::get().to(get))
                    .route(actix_web::web::patch().to(patch))
                    .route(actix_web::web::delete().to(delete)),
            ),
    );
}
