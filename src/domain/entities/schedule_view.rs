use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Queryable)]
#[diesel(table_name = appointments)]
pub struct ScheduleViewEntity {
    pub id: Uuid,
    pub slot_id: Uuid,
    pub patient_id: i32,
    pub patient_abnormal_symptom: String,
    pub patient_is_missed_medication: String,
    pub patient_blood_test_status: String,
    pub patient_is_overdue_medication: String,
    pub patient_is_partner_hiv_positive: String,
    pub status: String,
    pub doctor_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}
