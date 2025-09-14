use anyhow::Result;
use axum::async_trait;
use uuid::Uuid;

use crate::domain::entities::appointments::{AddAppointmentEntity};

#[async_trait]
pub trait AppointmentOpsRepository {
    async fn add(&self, add_appointment_entity: AddAppointmentEntity) -> Result<Uuid>;
    async fn remove(&self, appointment_id: Uuid, patient_id: i32) -> Result<()>;
}