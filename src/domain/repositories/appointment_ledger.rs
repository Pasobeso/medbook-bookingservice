use anyhow::Result;
use uuid::Uuid;
pub trait AppointmentLedgerRepository {
    async fn to_ready(&self, appointment_id: Uuid) -> Result<Uuid>;
    async fn to_waiting_for_prescription(&self, appointment_id: Uuid) -> Result<Uuid>;
    async fn to_completed(&self, appointment_id: Uuid) -> Result<Uuid>;
}
