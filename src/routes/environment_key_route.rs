use crate::models::environment_key::{
    EnvironmentKeyCreatePayload, EnvironmentKeyFilter, EnvironmentKeyResponse,
    EnvironmentKeySortOrder, EnvironmentKeySortableFields, EnvironmentKeyUpdatePayload,
};
use crate::models::pagination::Pagination;
use crate::models::sort::SortOrder;
use crate::services::base::Service;
use crate::services::environment_key_service::EnvironmentKeyService;
use actix_web::{Error, HttpResponse, web};
use serde_json::json;
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/environment-keys",
    tag = "EnvironmentKeys",
    request_body = EnvironmentKeyCreatePayload,
    responses(
        (status = 201, description = "Environment key created", body = EnvironmentKeyResponse),
        (status = 400, description = "Invalid request", body = String),
    ),
)]
pub async fn post(
    service: web::Data<EnvironmentKeyService>,
    payload: web::Json<EnvironmentKeyCreatePayload>,
) -> Result<HttpResponse, Error> {
    let environment_key = service
        .create(payload.into_inner())
        .await
        .map_err(actix_web::error::ErrorBadRequest)?;
    Ok(HttpResponse::Created().json(EnvironmentKeyResponse::from(environment_key)))
}

#[utoipa::path(
    post,
    path = "/environment-keys/{id}/rotate",
    tag = "EnvironmentKeys",
    responses(
        (status = 200, description = "Environment key rotated", body = EnvironmentKeyResponse, example = json!({
            "id": "123e4567-e89b-12d3-a456-426614174000",
            "message": "Environment key rotated successfully",
        })),
        (status = 400, description = "Invalid request", body = String),
    ),
)]
pub async fn rotate_key(
    service: web::Data<EnvironmentKeyService>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let environment_key = service
        .rotate_key(id.into_inner())
        .await
        .map_err(actix_web::error::ErrorBadRequest)?;
    Ok(HttpResponse::Ok().json(json!({
        "id": environment_key.id.unwrap(),
        "message": "Environment key rotated successfully",
    })))
}

#[utoipa::path(
    get,
    path = "/environment-keys/{id}",
    tag = "EnvironmentKeys",
    responses(
        (status = 200, description = "Environment key retrieved", body = EnvironmentKeyResponse),
        (status = 400, description = "Invalid request", body = String),
    ),
)]
pub async fn get(
    service: web::Data<EnvironmentKeyService>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let environment_key = service
        .read(id.into_inner())
        .await
        .map_err(actix_web::error::ErrorBadRequest)?;
    Ok(HttpResponse::Ok().json(environment_key.map(EnvironmentKeyResponse::from)))
}

#[utoipa::path(
    patch,
    path = "/environment-keys/{id}",
    tag = "EnvironmentKeys",
    request_body = EnvironmentKeyUpdatePayload,
    responses(
        (status = 200, description = "Environment key updated", body = EnvironmentKeyResponse),
        (status = 400, description = "Invalid request", body = String),
    ),
)]
pub async fn patch(
    service: web::Data<EnvironmentKeyService>,
    id: web::Path<Uuid>,
    payload: web::Json<EnvironmentKeyUpdatePayload>,
) -> Result<HttpResponse, Error> {
    let environment_key = service
        .update(id.into_inner(), payload.into_inner())
        .await
        .map_err(actix_web::error::ErrorBadRequest)?;
    Ok(HttpResponse::Ok().json(EnvironmentKeyResponse::from(environment_key)))
}

#[utoipa::path(
    delete,
    path = "/environment-keys/{id}",
    tag = "EnvironmentKeys",
    responses(
        (status = 200, description = "Environment key deleted", body = String),
        (status = 400, description = "Invalid request", body = String),
    ),
)]
pub async fn delete(
    service: web::Data<EnvironmentKeyService>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let deleted = service
        .delete(id.into_inner())
        .await
        .map_err(actix_web::error::ErrorBadRequest)?;
    Ok(HttpResponse::Ok().json(deleted))
}

#[utoipa::path(
    get,
    path = "/environment-keys",
    tag = "EnvironmentKeys",
    responses(
        (status = 200, description = "List environment keys", body = [EnvironmentKeyResponse]),
        (status = 400, description = "Invalid request", body = String),
    ),
)]
pub async fn list(
    service: web::Data<EnvironmentKeyService>,
    filter: web::Query<EnvironmentKeyFilter>,
    pagination: web::Query<Pagination>,
) -> Result<HttpResponse, Error> {
    let sort = vec![EnvironmentKeySortOrder {
        field: EnvironmentKeySortableFields::Id,
        order: SortOrder::Desc,
    }];

    let environment_keys = service
        .find(
            filter.into_inner(),
            Some(sort),
            Some(pagination.into_inner()),
        )
        .await
        .map_err(actix_web::error::ErrorBadRequest)?;
    let responses: Vec<EnvironmentKeyResponse> = environment_keys
        .into_iter()
        .map(EnvironmentKeyResponse::from)
        .collect();
    Ok(HttpResponse::Ok().json(responses))
}

pub fn configure_routes(config: &mut actix_web::web::ServiceConfig) {
    config.service(
        web::scope("/environment-keys")
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
            )
            .service(
                actix_web::web::resource("/{id}/rotate")
                    .route(actix_web::web::post().to(rotate_key)),
            ),
    );
}
