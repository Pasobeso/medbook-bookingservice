use crate::domain::entities::schedule_view::ScheduleViewEntity;
use anyhow::Result;

pub trait ScheduleViewingRepository {
    async fn get_patient_schedules(&self, patient_id: i32) -> Result<Vec<ScheduleViewEntity>>;
    async fn get_doctor_schedules(&self, doctor_id: i32) -> Result<Vec<ScheduleViewEntity>>;
}
