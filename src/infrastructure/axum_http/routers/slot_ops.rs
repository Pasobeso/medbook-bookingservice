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
        value_objects::slot_model::{AddSlotDto, AddSlotResponseModel, RemoveSlotResponseModel},
    },
    infrastructure::{
        axum_http::{api_response::ApiResponse, middleware::doctors_authorization},
        postgres::{postgres_connection::PgPoolSquad, repositories::slot_ops::SlotOpsPostgres},
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let slot_ops_repository = SlotOpsPostgres::new(db_pool);
    let slot_ops_use_case = SlotOpsUseCase::new(Arc::new(slot_ops_repository));

    Router::new()
        .route("/", post(add))
        // .route("/:quest_id", patch(edit))
        .route("/:quest_id", delete(remove))
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
    match slot_ops_use_case.add(add_slot_dto, doctor_id).await {
        Ok(slot_id) => {
            let response = format!("Add slot success with id: {}", slot_id);
            (
                StatusCode::OK,
                Json(ApiResponse::<AddSlotResponseModel> {
                    data: None,
                    message: Some(response),
                }),
            )
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<AddSlotResponseModel> {
                data: None,
                message: Some(e.to_string()),
            }),
        ),
    }
}

// pub async fn edit<T1, T2>(
//     State(quest_ops_use_case): State<Arc<QuestOpsUseCase<T1, T2>>>,
//     Extension(guild_commander_id): Extension<i32>,
//     Path(quest_id): Path<i32>,
//     Json(edit_quest_model): Json<EditQuestModel>,
// ) -> impl IntoResponse
// where
//     T1: QuestOpsRepository + Send + Sync,
//     T2: QuestViewingRepository + Send + Sync,
// {
//     match quest_ops_use_case
//         .edit(quest_id, guild_commander_id, edit_quest_model)
//         .await
//     {
//         Ok(quest_id) => {
//             let response = format!("Edit quest success with id: {}", quest_id);
//             (StatusCode::OK, response)
//         }
//         Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
//     }
// }

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
            let response = format!("Edit slot success with id: {}", slot_id);
            (
                StatusCode::OK,
                Json(ApiResponse::<RemoveSlotResponseModel> {
                    data: None,
                    message: Some(response),
                }),
            )
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<RemoveSlotResponseModel> {
                data: None,
                message: Some(e.to_string()),
            }),
        ),
    }
}
