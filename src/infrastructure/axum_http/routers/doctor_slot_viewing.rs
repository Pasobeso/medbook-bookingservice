use std::sync::Arc;

use axum::{
    extract::State, http::StatusCode, middleware, response::IntoResponse, routing::get, Extension, Json, Router
};

use crate::{application::usecases::slot_viewing::SlotViewingUseCase, domain::{repositories::slot_viewing::SlotViewingRepository, value_objects::slot_model::GetSlotsResponseModel}, infrastructure::{axum_http::{api_response::ApiResponse, middleware::doctors_authorization}, postgres::{
    postgres_connection::PgPoolSquad, repositories::slot_viewing::SlotViewingPostgres,
}}};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let slot_viewing_repository = SlotViewingPostgres::new(db_pool);
    let slot_viewing_use_case = SlotViewingUseCase::new(Arc::new(slot_viewing_repository));

    Router::new()
        .route("/view-my-slots", get(get_doctor_slots))
        .route_layer(middleware::from_fn(doctors_authorization))
        .with_state(Arc::new(slot_viewing_use_case))
}

async fn get_doctor_slots<T>(
    State(slot_viewing_use_case): State<Arc<SlotViewingUseCase<T>>>,
    Extension(doctor_id): Extension<i32>,
) -> impl IntoResponse
where
    T: SlotViewingRepository + Send + Sync,
{
    match slot_viewing_use_case
        .get_doctor_slots(doctor_id)
        .await
    {
        Ok(slots) => (
            StatusCode::OK,
            Json(ApiResponse::<GetSlotsResponseModel> {
                data: Some(GetSlotsResponseModel { slots }),
                message: None,
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<GetSlotsResponseModel> {
                data: None,
                message: Some(e.to_string()),
            }),
        )
            .into_response(),
    }
}
