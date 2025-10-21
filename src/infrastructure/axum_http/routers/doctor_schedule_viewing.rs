use std::sync::Arc;

use axum::{
    Extension, Json, Router, extract::State, http::StatusCode, middleware, response::IntoResponse,
    routing::get,
};
use utoipa_axum::router::OpenApiRouter;

use crate::{
    application::usecases::schedule_viewing::ScheduleViewingUseCase,
    domain::{
        repositories::schedule_viewing::ScheduleViewingRepository,
        value_objects::schedule_model::GetDoctorScheduleResponseModel,
    },
    infrastructure::{
        axum_http::{api_response::ApiResponse, middleware::doctors_authorization},
        postgres::{
            postgres_connection::PgPoolSquad,
            repositories::schedule_viewing::ScheduleViewingPostgres,
        },
    },
};

#[deprecated]
pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let schedule_viewing_repository = ScheduleViewingPostgres::new(db_pool);
    let schedule_viewing_use_case =
        ScheduleViewingUseCase::new(Arc::new(schedule_viewing_repository));

    Router::new()
        .route("/", get(get_doctor_schedules))
        .route_layer(middleware::from_fn(doctors_authorization))
        .with_state(Arc::new(schedule_viewing_use_case))
}

/// Defines routes with OpenAPI specs. Should be used over `routes()` where possible.
pub fn routes_with_openapi(db_pool: Arc<PgPoolSquad>) -> OpenApiRouter {
    let schedule_viewing_repository = ScheduleViewingPostgres::new(db_pool);
    let schedule_viewing_use_case =
        ScheduleViewingUseCase::new(Arc::new(schedule_viewing_repository));

    OpenApiRouter::new().nest(
        "/schedule-view/doctor",
        OpenApiRouter::new()
            .routes(utoipa_axum::routes!(get_doctor_schedules))
            .route_layer(middleware::from_fn(doctors_authorization))
            .with_state(Arc::new(schedule_viewing_use_case)),
    )
}

/// Retrieves all schedules belonging to the authenticated doctor.
#[utoipa::path(
    get,
    path = "/",
    tags = ["Schedule Viewing"],
    responses(
        (status = 200, description = "Fetched doctor schedules successfully", body = ApiResponse<GetDoctorScheduleResponseModel>)
    )
)]
async fn get_doctor_schedules<T>(
    State(schedule_viewing_use_case): State<Arc<ScheduleViewingUseCase<T>>>,
    Extension(doctor_id): Extension<i32>,
) -> impl IntoResponse
where
    T: ScheduleViewingRepository + Send + Sync,
{
    match schedule_viewing_use_case
        .get_doctor_schedules(doctor_id)
        .await
    {
        Ok(schedules) => (
            StatusCode::OK,
            Json(ApiResponse::<GetDoctorScheduleResponseModel> {
                data: Some(GetDoctorScheduleResponseModel { schedules }),
                message: None,
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<GetDoctorScheduleResponseModel> {
                data: None,
                message: Some(e.to_string()),
            }),
        )
            .into_response(),
    }
}
