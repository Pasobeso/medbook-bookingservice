use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::entities::schedule_view::ScheduleViewEntity;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GetDoctorScheduleResponseModel {
    pub schedules: Vec<ScheduleViewEntity>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GetPatientScheduleResponseModel {
    pub schedules: Vec<ScheduleViewEntity>,
}
