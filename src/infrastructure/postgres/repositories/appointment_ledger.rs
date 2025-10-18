use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncConnection};
use uuid::Uuid;

use crate::{
    domain::{
        repositories::appointment_ledger::AppointmentLedgerRepository,
        value_objects::appointment_status::AppointmentStatus,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad,
        repositories::data_access_objects::{
            appointment_ledger::AppointmentLedgerDao, appointment_viewing::AppointmentViewingDao,
        },
    },
};

pub struct AppointmentLedgerPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl AppointmentLedgerPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl AppointmentLedgerRepository for AppointmentLedgerPostgres {
    async fn to_ready(&self, appointment_id: Uuid) -> Result<Uuid> {
        let mut conn = self.db_pool.get().await?;

        let result = conn
            .transaction(|conn| {
                async move {
                    let current_appointment_status =
                        AppointmentViewingDao::get_appointment_status_by_appointment_id(
                            conn,
                            appointment_id,
                        )
                        .await?;

                    let condition_to_update =
                        current_appointment_status == AppointmentStatus::Waiting.to_string();

                    if !condition_to_update {
                        return Err(anyhow::anyhow!("Invalid condition to change status"));
                    }

                    let appointment_status = AppointmentStatus::Ready;
                    let appointment = AppointmentLedgerDao::change_appointment_status(
                        conn,
                        appointment_id,
                        appointment_status,
                    )
                    .await?;

                    anyhow::Ok(appointment)
                }
                .scope_boxed()
            })
            .await?;

        Ok(result)
    }

    async fn to_waiting_for_prescription(&self, appointment_id: Uuid) -> Result<Uuid> {
        let mut conn = self.db_pool.get().await?;

        let result = conn
            .transaction(|conn| {
                async move {
                    let current_appointment_status =
                        AppointmentViewingDao::get_appointment_status_by_appointment_id(
                            conn,
                            appointment_id,
                        )
                        .await?;

                    let condition_to_update =
                        current_appointment_status == AppointmentStatus::Ready.to_string();

                    if !condition_to_update {
                        return Err(anyhow::anyhow!("Invalid condition to change status"));
                    }

                    let appointment_status = AppointmentStatus::WaitingForPrescription;
                    let appointment = AppointmentLedgerDao::change_appointment_status(
                        conn,
                        appointment_id,
                        appointment_status,
                    )
                    .await?;

                    anyhow::Ok(appointment)
                }
                .scope_boxed()
            })
            .await?;

        Ok(result)
    }

    async fn to_completed(&self, appointment_id: Uuid) -> Result<Uuid> {
        let mut conn = self.db_pool.get().await?;

        let result = conn
            .transaction(|conn| {
                async move {
                    let current_appointment_status =
                        AppointmentViewingDao::get_appointment_status_by_appointment_id(
                            conn,
                            appointment_id,
                        )
                        .await?;

                    let condition_to_update = current_appointment_status
                        == AppointmentStatus::WaitingForPrescription.to_string();

                    if !condition_to_update {
                        return Err(anyhow::anyhow!("Invalid condition to change status"));
                    }

                    let appointment_status = AppointmentStatus::Completed;
                    let appointment = AppointmentLedgerDao::change_appointment_status(
                        conn,
                        appointment_id,
                        appointment_status,
                    )
                    .await?;

                    anyhow::Ok(appointment)
                }
                .scope_boxed()
            })
            .await?;

        Ok(result)
    }
}
