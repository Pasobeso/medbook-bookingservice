use anyhow::Result;
use chrono::NaiveDateTime;
use diesel::{dsl::exists, prelude::*, select};

use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{domain::entities::slots::SlotEntity, infrastructure::postgres::schema::slots};

pub struct SlotViewingDao;

impl SlotViewingDao {
    pub async fn is_overlapping_slots_for_doctor_id(
        conn: &mut AsyncPgConnection,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
        doctor_id: i32,
    ) -> Result<bool> {
        let overlap_exists = select(exists(
            slots::table
                .filter(slots::doctor_id.eq(doctor_id))
                .filter(slots::deleted_at.is_null())
                // overlap rule: [start, end)
                .filter(slots::start_time.lt(end_time))
                .filter(slots::end_time.gt(start_time)),
        ))
        .get_result::<bool>(conn)
        .await?;

        Ok(overlap_exists)
    }

    pub async fn get_current_appointment_count_by_slot_id(
        conn: &mut AsyncPgConnection,
        slot_id: Uuid,
    ) -> Result<i32> {
        let result = slots::table
            .filter(slots::deleted_at.is_null())
            .filter(slots::id.eq(slot_id))
            .select(slots::current_appointment_count)
            .first::<i32>(conn)
            .await?;

        Ok(result)
    }

    pub async fn get_end_time_by_slot_id(
        conn: &mut AsyncPgConnection,
        slot_id: Uuid,
    ) -> Result<NaiveDateTime> {
        let result = slots::table
            .filter(slots::deleted_at.is_null())
            .filter(slots::id.eq(slot_id))
            .select(slots::end_time)
            .first::<NaiveDateTime>(conn)
            .await?;

        Ok(result)
    }

    pub async fn get_slots(conn: &mut AsyncPgConnection) -> Result<Vec<SlotEntity>> {
        let result = slots::table
            .filter(slots::deleted_at.is_null())
            .load::<SlotEntity>(conn)
            .await?;

        Ok(result)
    }

    pub async fn get_doctor_slots(
        conn: &mut AsyncPgConnection,
        doctor_id: i32,
    ) -> Result<Vec<SlotEntity>> {
        let result = slots::table
            .filter(slots::deleted_at.is_null())
            .filter(slots::doctor_id.eq(doctor_id))
            .load::<SlotEntity>(conn)
            .await?;

        Ok(result)
    }
}
