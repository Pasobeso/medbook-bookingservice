use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::entities::slots::{AddSlotEntity, EditSlotEntity, SlotEntity};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EditSlotDto {
    pub max_appointment_count: Option<i32>,
    pub end_time: Option<NaiveDateTime>,
}

impl EditSlotDto {
    pub fn to_entity(&self, current_time: NaiveDateTime) -> EditSlotEntity {
        EditSlotEntity {
            max_appointment_count: self.max_appointment_count,
            end_time: self.end_time,
            updated_at: current_time,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GetSlotsResponseModel {
    pub slots: Vec<SlotEntity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDoctorSlotsResponseModel {
    pub slots: Vec<SlotEntity>,
}
