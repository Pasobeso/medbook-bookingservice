use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::patch,
};
use utoipa_axum::router::OpenApiRouter;
use uuid::Uuid;

use crate::{
    application::usecases::appointment_ledger::AppointmentLedgerUseCase,
    domain::{
        repositories::appointment_ledger::AppointmentLedgerRepository,
        value_objects::appointment_status::AppointmentStatus,
    },
    infrastructure::{
        axum_http::{
            api_response::{ApiResponse, EmptyResponseModel},
            middleware::doctors_authorization,
        },
        postgres::{
            postgres_connection::PgPoolSquad,
            repositories::appointment_ledger::AppointmentLedgerPostgres,
        },
    },
};

#[deprecated]
pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let appointment_ledger_repository = AppointmentLedgerPostgres::new(db_pool);
    let appointment_ledger_use_case =
        AppointmentLedgerUseCase::new(Arc::new(appointment_ledger_repository));

    Router::new()
        .route("/to-ready/:appointment_id", patch(to_ready))
        .route(
            "/to-waiting-for-prescription/:appointment_id",
            patch(to_waiting_for_prescription),
        )
        .route("/to-completed/:appointment_id", patch(to_completed))
        .route_layer(middleware::from_fn(doctors_authorization))
        .with_state(Arc::new(appointment_ledger_use_case))
}

/// Defines routes with OpenAPI specs. Should be used over `routes()` where possible.
pub fn routes_with_openapi(db_pool: Arc<PgPoolSquad>) -> OpenApiRouter {
    let appointment_ledger_repository = AppointmentLedgerPostgres::new(db_pool);
    let appointment_ledger_use_case =
        AppointmentLedgerUseCase::new(Arc::new(appointment_ledger_repository));

    OpenApiRouter::new().nest(
        "/appointment-ledger",
        OpenApiRouter::new()
            .routes(utoipa_axum::routes!(to_ready))
            .routes(utoipa_axum::routes!(to_waiting_for_prescription))
            .routes(utoipa_axum::routes!(to_completed))
            .route_layer(middleware::from_fn(doctors_authorization))
            .with_state(Arc::new(appointment_ledger_use_case)),
    )
}

/// Marks an appointment as **Ready**.
#[utoipa::path(
    patch,
    path = "/to-ready/{appointment_id}",
    tags = ["Appointment Ledger"],
    params(
        ("appointment_id" = Uuid, Path, description = "Appointment ID to update to Ready")
    ),
    responses(
        (status = 200, description = "Appointment status updated to Ready successfully", body = ApiResponse<EmptyResponseModel>)
    )
)]
pub async fn to_ready<T>(
    State(appointment_ledger_use_case): State<Arc<AppointmentLedgerUseCase<T>>>,
    Path(appointment_id): Path<Uuid>,
) -> impl IntoResponse
where
    T: AppointmentLedgerRepository + Send + Sync,
{
    match appointment_ledger_use_case
        .to_completed(appointment_id)
        .await
    {
        Ok(appointment_id) => {
            let response = format!(
                "Appointment id: {} is now {:?}",
                appointment_id,
                AppointmentStatus::Ready
            );
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

/// Marks an appointment as **Waiting for Prescription**.
#[utoipa::path(
    patch,
    path = "/to-waiting-for-prescription/{appointment_id}",
    tags = ["Appointment Ledger"],
    params(
        ("appointment_id" = Uuid, Path, description = "Appointment ID to update to WaitingForPrescription")
    ),
    responses(
        (status = 200, description = "Appointment status updated to WaitingForPrescription successfully", body = ApiResponse<EmptyResponseModel>)
    )
)]

pub async fn to_waiting_for_prescription<T>(
    State(appointment_ledger_use_case): State<Arc<AppointmentLedgerUseCase<T>>>,
    Path(appointment_id): Path<Uuid>,
) -> impl IntoResponse
where
    T: AppointmentLedgerRepository + Send + Sync,
{
    match appointment_ledger_use_case
        .to_waiting_for_prescription(appointment_id)
        .await
    {
        Ok(appointment_id) => {
            let response = format!(
                "Appointment id: {} is now {:?}",
                appointment_id,
                AppointmentStatus::WaitingForPrescription
            );
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

/// Marks an appointment as **Completed**.
#[utoipa::path(
    patch,
    path = "/to-completed/{appointment_id}",
    tags = ["Appointment Ledger"],
    params(
        ("appointment_id" = Uuid, Path, description = "Appointment ID to update to Completed")
    ),
    responses(
        (status = 200, description = "Appointment status updated to Completed successfully", body = ApiResponse<EmptyResponseModel>)
    )
)]
pub async fn to_completed<T>(
    State(appointment_ledger_use_case): State<Arc<AppointmentLedgerUseCase<T>>>,
    Path(appointment_id): Path<Uuid>,
) -> impl IntoResponse
where
    T: AppointmentLedgerRepository + Send + Sync,
{
    match appointment_ledger_use_case
        .to_completed(appointment_id)
        .await
    {
        Ok(appointment_id) => {
            let response = format!(
                "Appointment id: {} is now {:?}",
                appointment_id,
                AppointmentStatus::Completed
            );
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
