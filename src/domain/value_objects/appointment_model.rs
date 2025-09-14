use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{entities::appointments::{AddAppointmentEntity, EditAppointmentEntity}, value_objects::appointment_status::AppointmentStatus};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct AddAppointmentDto {
    pub slot_id: Uuid,
    pub patient_abnormal_symptom: String,
    pub patient_is_missed_medication: String,
    pub patient_blood_test_status: String,
    pub patient_is_overdue_medication: String,
    pub patient_is_partner_hiv_positive: String,

}

impl AddAppointmentDto {
    pub fn to_entity(&self, patient_id: i32, current_time: NaiveDateTime) -> AddAppointmentEntity {
        AddAppointmentEntity {
            slot_id: self.slot_id,
            patient_id,
            patient_abnormal_symptom: self.patient_abnormal_symptom.clone(),
            patient_is_missed_medication: self.patient_is_missed_medication.clone(),
            patient_blood_test_status: self.patient_blood_test_status.clone(),
            patient_is_overdue_medication: self.patient_is_overdue_medication.clone(),
            patient_is_partner_hiv_positive: self.patient_is_partner_hiv_positive.clone(),
            status: AppointmentStatus::Waiting.to_string(),
            created_at: current_time,
            updated_at: current_time,
            deleted_at: None,
        }
    }
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct EditAppointmentDto {
    pub slot_id: Option<Uuid>,
    pub patient_abnormal_symptom: Option<String>,
    pub patient_is_missed_medication: Option<String>,
    pub patient_blood_test_status: Option<String>,
    pub patient_is_overdue_medication: Option<String>,
    pub patient_is_partner_hiv_positive: Option<String>,
}

impl EditAppointmentDto {
    pub fn to_entity(&self, current_time: NaiveDateTime) -> EditAppointmentEntity {
        EditAppointmentEntity {
            patient_abnormal_symptom: self.patient_abnormal_symptom.clone(),
            patient_is_missed_medication: self.patient_is_missed_medication.clone(),
            patient_blood_test_status: self.patient_blood_test_status.clone(),
            patient_is_overdue_medication: self.patient_is_overdue_medication.clone(),
            patient_is_partner_hiv_positive: self.patient_is_partner_hiv_positive.clone(),
            updated_at: current_time,
        }
    }
}
