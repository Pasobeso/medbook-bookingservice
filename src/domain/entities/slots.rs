use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::infrastructure::postgres::schema::slots;

#[derive(Debug, Clone, Identifiable, Selectable, Serialize, Deserialize, Queryable, ToSchema)]
#[diesel(table_name = slots)]
pub struct SlotEntity {
    pub id: Uuid,
    pub doctor_id: i32,
    pub current_appointment_count: i32,
    pub max_appointment_count: i32,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, Queryable)]
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

#[derive(Debug, Clone, Queryable, AsChangeset)]
#[diesel(table_name = slots)]
pub struct EditSlotEntity {
    pub max_appointment_count: Option<i32>,
    pub end_time: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}
