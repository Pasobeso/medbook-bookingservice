use anyhow::Result;
use axum::async_trait;

use crate::domain::entities::schedule_view::ScheduleViewEntity;

#[async_trait]
pub trait ScheduleViewingRepository {
    async fn get_patient_schedules(&self, patient_id: i32) -> Result<Vec<ScheduleViewEntity>>;
    async fn get_doctor_schedules(&self, doctor_id: i32) -> Result<Vec<ScheduleViewEntity>>;
}