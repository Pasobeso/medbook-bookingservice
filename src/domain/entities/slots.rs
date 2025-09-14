use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::infrastructure::postgres::schema::slots;

#[derive(Debug,Clone,Insertable,Queryable)]
#[diesel(table_name = slots)]
pub struct AddSlotEntity {
    pub doctor_id: i32,
    pub current_appointment_count: i32,
    pub max_appointment_count: i32,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

