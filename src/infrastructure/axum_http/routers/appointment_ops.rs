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

use crate::{application::usecases::appointment_ops::AppointmentOpsUseCase, domain::{repositories::appointment_ops::AppointmentOpsRepository, value_objects::appointment_model::AddAppointmentDto}, infrastructure::{axum_http::{api_response::{ApiResponse, EmptyResponseModel}, middleware::patients_authorization}, postgres::{postgres_connection::PgPoolSquad, repositories::appointment_ops::AppointmentOpsPostgres}}};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let appointment_ops_repository = AppointmentOpsPostgres::new(db_pool);
    let appointment_ops_use_case = AppointmentOpsUseCase::new(Arc::new(appointment_ops_repository));

    Router::new()
        .route("/", post(add))
        // .route("/:quest_id", patch(edit))
        .route("/:appointment_id", delete(remove))
        .route_layer(middleware::from_fn(patients_authorization))
        .with_state(Arc::new(appointment_ops_use_case))
}

pub async fn add<T>(
    State(appointment_ops_use_case): State<Arc<AppointmentOpsUseCase<T>>>,
    Extension(patient_id): Extension<i32>,
    Json(add_appointment_dto): Json<AddAppointmentDto>,
) -> impl IntoResponse
where
    T: AppointmentOpsRepository + Send + Sync,
{
    match appointment_ops_use_case.add(add_appointment_dto, patient_id).await {
        Ok(appointment_id) => {
            let response = format!("Add appointment success with id: {}", appointment_id);
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
    State(appointment_ops_use_case): State<Arc<AppointmentOpsUseCase<T>>>,
    Extension(patient_id): Extension<i32>,
    Path(appointment_id): Path<Uuid>,
) -> impl IntoResponse
where
    T: AppointmentOpsRepository + Send + Sync,
{
    match appointment_ops_use_case.remove(appointment_id, patient_id).await {
        Ok(_) => {
            let response = format!("Edit appointment success with id: {}", appointment_id);
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
