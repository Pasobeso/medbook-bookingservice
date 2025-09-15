use serde::{Deserialize, Serialize};

use crate::domain::entities::schedule_view::ScheduleViewEntity;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct GetDoctorScheduleResponseModel {
    pub schedules: Vec<ScheduleViewEntity>,
}


#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct GetPatientScheduleResponseModel {
    pub schedules: Vec<ScheduleViewEntity>,
}
