use anyhow::Result;
use axum::async_trait;
use uuid::Uuid;

use crate::domain::entities::appointments::{AddAppointmentEntity, EditAppointmentEntity, RescheduleAppointmentEntity};

#[async_trait]
pub trait AppointmentOpsRepository {
    async fn add(&self, add_appointment_entity: AddAppointmentEntity) -> Result<Uuid>;
    async fn edit(&self, appointment_id: Uuid, patient_id: i32, edit_appointment_entity: EditAppointmentEntity) -> Result<Uuid>;
    async fn reschedule(&self, appointment_id: Uuid, patient_id: i32, reschedule_appointment_entity: RescheduleAppointmentEntity) -> Result<Uuid>;
    async fn remove(&self, appointment_id: Uuid, patient_id: i32) -> Result<()>;
}