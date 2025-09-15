use std::sync::Arc;

use axum::{
    Extension, Json, Router, extract::State, http::StatusCode, middleware, response::IntoResponse,
    routing::get,
};

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

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let schedule_viewing_repository = ScheduleViewingPostgres::new(db_pool);
    let schedule_viewing_use_case =
        ScheduleViewingUseCase::new(Arc::new(schedule_viewing_repository));

    Router::new()
        .route("/", get(get_doctor_schedules))
        .route_layer(middleware::from_fn(doctors_authorization))
        .with_state(Arc::new(schedule_viewing_use_case))
}

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
