use std::sync::Arc;

use anyhow::Result;
use uuid::Uuid;

use crate::domain::{
    entities::appointments::RescheduleAppointmentEntity, repositories::appointment_ops::AppointmentOpsRepository, value_objects::appointment_model::{AddAppointmentDto, EditAppointmentDto}
};

pub struct AppointmentOpsUseCase<T>
where
    T: AppointmentOpsRepository,
{
    appointment_ops_repository: Arc<T>,
}

impl<T> AppointmentOpsUseCase<T>
where
    T: AppointmentOpsRepository + Send + Sync,
{
    pub fn new(appointment_ops_repository: Arc<T>) -> Self {
        Self {
            appointment_ops_repository,
        }
    }

    pub async fn add(&self, add_appointment_dto: AddAppointmentDto, patient_id: i32) -> Result<Uuid> {
        let current_time = chrono::Utc::now().naive_utc();
        let add_appointment_entity = add_appointment_dto.to_entity(patient_id, current_time);

        let appointment_id = self.appointment_ops_repository.add(add_appointment_entity).await?;
        Ok(appointment_id)
    }

    pub async fn edit(&self, appointment_id: Uuid, patient_id :i32, edit_appointment_dto: EditAppointmentDto) -> Result<()> {
        let current_time = chrono::Utc::now().naive_utc();
        if let Some(new_slot_id) = edit_appointment_dto.slot_id {
            let reschedule_appointment_entity = RescheduleAppointmentEntity {
                slot_id: new_slot_id,
                updated_at: current_time
            };
            self.appointment_ops_repository.reschedule(appointment_id, patient_id, reschedule_appointment_entity).await?;
        }

        let edit_appointment_entity = edit_appointment_dto.to_entity(current_time);
        self.appointment_ops_repository.edit(appointment_id, patient_id, edit_appointment_entity).await?;

        Ok(())
    }

    pub async fn remove(&self, appointment_id: Uuid, patient_id: i32) -> Result<()> {
        self.appointment_ops_repository.remove(appointment_id, patient_id).await?;

        Ok(())
    }
}
