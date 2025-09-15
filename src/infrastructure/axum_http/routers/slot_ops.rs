use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, patch, post},
};
use uuid::Uuid;

use crate::{
    application::usecases::slot_ops::SlotOpsUseCase,
    domain::{
        repositories::slot_ops::SlotOpsRepository,
        value_objects::slot_model::{AddSlotDto, EditSlotDto},
    },
    infrastructure::{
        axum_http::{
            api_response::{ApiResponse, EmptyResponseModel},
            middleware::doctors_authorization,
        },
        postgres::{postgres_connection::PgPoolSquad, repositories::slot_ops::SlotOpsPostgres},
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let slot_ops_repository = SlotOpsPostgres::new(db_pool);
    let slot_ops_use_case = SlotOpsUseCase::new(Arc::new(slot_ops_repository));

    Router::new()
        .route("/", post(add))
        .route("/:slot_id", patch(edit))
        .route("/:slot_id", delete(remove))
        .route_layer(middleware::from_fn(doctors_authorization))
        .with_state(Arc::new(slot_ops_use_case))
}

pub async fn add<T>(
    State(slot_ops_use_case): State<Arc<SlotOpsUseCase<T>>>,
    Extension(doctor_id): Extension<i32>,
    Json(add_slot_dto): Json<AddSlotDto>,
) -> impl IntoResponse
where
    T: SlotOpsRepository + Send + Sync,
{
    match slot_ops_use_case.add(doctor_id, add_slot_dto).await {
        Ok(slot_id) => {
            let response = format!("Add slot success with id: {}", slot_id);
            (
                StatusCode::OK,
                Json(ApiResponse::<EmptyResponseModel> {
                    data: None,
                    message: Some(response),
                }),
            )
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<EmptyResponseModel> {
                data: None,
                message: Some(e.to_string()),
            }),
        ),
    }
}

pub async fn edit<T>(
    State(slot_ops_use_case): State<Arc<SlotOpsUseCase<T>>>,
    Extension(doctor_id): Extension<i32>,
    Path(slot_id): Path<Uuid>,
    Json(edit_slot_dto): Json<EditSlotDto>,
) -> impl IntoResponse
where
    T: SlotOpsRepository + Send + Sync,
{
    match slot_ops_use_case
        .edit(slot_id, doctor_id, edit_slot_dto)
        .await
    {
        Ok(quest_id) => {
            let response = format!("Edit slot success with id: {}", quest_id);
            (
                StatusCode::OK,
                Json(ApiResponse::<EmptyResponseModel> {
                    data: None,
                    message: Some(response),
                }),
            )
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<EmptyResponseModel> {
                data: None,
                message: Some(e.to_string()),
            }),
        ),
    }
}

pub async fn remove<T>(
    State(slot_ops_use_case): State<Arc<SlotOpsUseCase<T>>>,
    Extension(doctor_id): Extension<i32>,
    Path(slot_id): Path<Uuid>,
) -> impl IntoResponse
where
    T: SlotOpsRepository + Send + Sync,
{
    match slot_ops_use_case.remove(slot_id, doctor_id).await {
        Ok(_) => {
            let response = format!("Remove slot success with id: {}", slot_id);
            (
                StatusCode::OK,
                Json(ApiResponse::<EmptyResponseModel> {
                    data: None,
                    message: Some(response),
                }),
            )
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<EmptyResponseModel> {
                data: None,
                message: Some(e.to_string()),
            }),
        ),
    }
}
