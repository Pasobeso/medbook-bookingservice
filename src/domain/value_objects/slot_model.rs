use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::domain::entities::slots::AddSlotEntity;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct AddSlotDto {
    pub max_appointment_count: i32,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}

impl AddSlotDto {
    pub fn to_entity(&self, doctor_id: i32, current_time: NaiveDateTime) -> AddSlotEntity {
        AddSlotEntity {
            doctor_id,
            current_appointment_count: 0,
            max_appointment_count: self.max_appointment_count,
            start_time: self.start_time,
            end_time: self.end_time,
            created_at: current_time,
            updated_at: current_time,
            deleted_at: None,
        }
    }
}
