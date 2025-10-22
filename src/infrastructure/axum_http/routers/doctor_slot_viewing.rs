use std::sync::Arc;

use axum::{
    Extension, Json, Router, extract::State, http::StatusCode, middleware, response::IntoResponse,
    routing::get,
};
use utoipa_axum::router::OpenApiRouter;

use crate::{
    application::usecases::slot_viewing::SlotViewingUseCase,
    domain::{
        repositories::slot_viewing::SlotViewingRepository,
        value_objects::slot_model::GetSlotsResponseModel,
    },
    infrastructure::{
        axum_http::{api_response::ApiResponse, middleware::doctors_authorization},
        postgres::{
            postgres_connection::PgPoolSquad, repositories::slot_viewing::SlotViewingPostgres,
        },
    },
};

#[deprecated]
pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let slot_viewing_repository = SlotViewingPostgres::new(db_pool);
    let slot_viewing_use_case = SlotViewingUseCase::new(Arc::new(slot_viewing_repository));

    Router::new()
        .route("/view-my-slots", get(get_doctor_slots))
        .route_layer(middleware::from_fn(doctors_authorization))
        .with_state(Arc::new(slot_viewing_use_case))
}

/// Defines routes with OpenAPI specs. Should be used over `routes()` where possible.
pub fn routes_with_openapi(db_pool: Arc<PgPoolSquad>) -> OpenApiRouter {
    let slot_viewing_repository = SlotViewingPostgres::new(db_pool);
    let slot_viewing_use_case = SlotViewingUseCase::new(Arc::new(slot_viewing_repository));

    OpenApiRouter::new().nest(
        "/slot-view",
        OpenApiRouter::new()
            .routes(utoipa_axum::routes!(get_doctor_slots))
            .route_layer(middleware::from_fn(doctors_authorization))
            .with_state(Arc::new(slot_viewing_use_case)),
    )
}

/// Retrieves all available slots belonging to the authenticated doctor.
#[utoipa::path(
    get,
    path = "/view-my-slots",
    tags = ["Slot Viewing"],
    responses(
        (status = 200, description = "Fetched doctor slots successfully", body = ApiResponse<GetSlotsResponseModel>)
    )
)]
async fn get_doctor_slots<T>(
    State(slot_viewing_use_case): State<Arc<SlotViewingUseCase<T>>>,
    Extension(doctor_id): Extension<i32>,
) -> impl IntoResponse
where
    T: SlotViewingRepository + Send + Sync,
{
    match slot_viewing_use_case.get_doctor_slots(doctor_id).await {
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
