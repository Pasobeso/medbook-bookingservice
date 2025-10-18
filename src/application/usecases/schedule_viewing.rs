use std::sync::Arc;

use anyhow::Result;
use tracing::info;

use crate::domain::{
    entities::schedule_view::ScheduleViewEntity,
    repositories::schedule_viewing::ScheduleViewingRepository,
};

pub struct ScheduleViewingUseCase<T>
where
    T: ScheduleViewingRepository,
{
    schedule_viewing_repository: Arc<T>,
}

impl<T> ScheduleViewingUseCase<T>
where
    T: ScheduleViewingRepository + Send + Sync,
{
    pub fn new(schedule_viewing_repository: Arc<T>) -> Self {
        Self {
            schedule_viewing_repository,
        }
    }

    pub async fn get_patient_schedules(&self, patient_id: i32) -> Result<Vec<ScheduleViewEntity>> {
        let schedules = self
            .schedule_viewing_repository
            .get_patient_schedules(patient_id)
            .await?;
        Ok(schedules)
    }

    pub async fn get_doctor_schedules(&self, doctor_id: i32) -> Result<Vec<ScheduleViewEntity>> {
        let schedules = self
            .schedule_viewing_repository
            .get_doctor_schedules(doctor_id)
            .await?;
        Ok(schedules)
    }
}
