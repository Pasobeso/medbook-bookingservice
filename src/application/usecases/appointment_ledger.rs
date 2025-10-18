use std::sync::Arc;

use anyhow::Result;
use uuid::Uuid;

use crate::domain::repositories::appointment_ledger::AppointmentLedgerRepository;

pub struct AppointmentLedgerUseCase<T>
where
    T: AppointmentLedgerRepository,
{
    appointment_ledger_repository: Arc<T>,
}

impl<T> AppointmentLedgerUseCase<T>
where
    T: AppointmentLedgerRepository + Send + Sync,
{
    pub fn new(appointment_ledger_repository: Arc<T>) -> Self {
        Self {
            appointment_ledger_repository,
        }
    }

    pub async fn to_ready(&self, appointment_id: Uuid) -> Result<Uuid> {
        let result = self
            .appointment_ledger_repository
            .to_ready(appointment_id)
            .await?;
        Ok(result)
    }

    pub async fn to_waiting_for_prescription(&self, appointment_id: Uuid) -> Result<Uuid> {
        let result = self
            .appointment_ledger_repository
            .to_waiting_for_prescription(appointment_id)
            .await?;
        Ok(result)
    }

    pub async fn to_completed(&self, appointment_id: Uuid) -> Result<Uuid> {
        let result = self
            .appointment_ledger_repository
            .to_completed(appointment_id)
            .await?;
        Ok(result)
    }
}
