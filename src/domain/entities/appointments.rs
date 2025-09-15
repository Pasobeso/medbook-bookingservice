use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::infrastructure::postgres::schema::appointments;

#[derive(Debug, Clone, Insertable, Queryable)]
#[diesel(table_name = appointments)]
pub struct AddAppointmentEntity {
    pub slot_id: Uuid,
    pub patient_id: i32,
    pub patient_abnormal_symptom: String,
    pub patient_is_missed_medication: String,
    pub patient_blood_test_status: String,
    pub patient_is_overdue_medication: String,
    pub patient_is_partner_hiv_positive: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Queryable, AsChangeset)]
#[diesel(table_name = appointments)]
pub struct EditAppointmentEntity {
    pub patient_abnormal_symptom: Option<String>,
    pub patient_is_missed_medication: Option<String>,
    pub patient_blood_test_status: Option<String>,
    pub patient_is_overdue_medication: Option<String>,
    pub patient_is_partner_hiv_positive: Option<String>,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Queryable, AsChangeset)]
#[diesel(table_name = appointments)]
pub struct RescheduleAppointmentEntity {
    pub slot_id: Uuid,
    pub updated_at: NaiveDateTime,
}