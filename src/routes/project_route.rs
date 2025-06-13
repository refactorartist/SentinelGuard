use crate::services::project_service::ProjectService;
use crate::{
    models::{
        pagination::Pagination,
        project::{
            ProjectCreatePayload, ProjectFilter, ProjectSortOrder, ProjectSortableFields,
            ProjectUpdatePayload,
        },
        sort::SortOrder,
    },
    services::base::Service,
};
use actix_web::{Error, HttpResponse, web};

pub async fn post(
    service: web::Data<ProjectService>,
    payload: web::Json<ProjectCreatePayload>,
) -> Result<HttpResponse, Error> {
    let project = service
        .create(payload.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Created().json(project))
}

pub async fn get(
    service: web::Data<ProjectService>,
    id: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, Error> {
    let project = service
        .read(id.into_inner())
        .await
        .map_err(actix_web::error::ErrorNotFound)?;
    Ok(HttpResponse::Ok().json(project))
}

pub async fn patch(
    service: web::Data<ProjectService>,
    id: web::Path<uuid::Uuid>,
    payload: web::Json<ProjectUpdatePayload>,
) -> Result<HttpResponse, Error> {
    let project = service
        .update(id.into_inner(), payload.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(project))
}

pub async fn delete(
    service: web::Data<ProjectService>,
    id: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, Error> {
    let deleted = service
        .delete(id.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if !deleted {
        return Err(actix_web::error::ErrorNotFound("Project not found"));
    }

    Ok(HttpResponse::NoContent().finish())
}

pub async fn list(
    service: web::Data<ProjectService>,
    filter: web::Query<ProjectFilter>,
    pagination: web::Query<Pagination>,
) -> Result<HttpResponse, Error> {
    let sort = vec![ProjectSortOrder::new(
        ProjectSortableFields::Id,
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
        web::scope("/projects")
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
