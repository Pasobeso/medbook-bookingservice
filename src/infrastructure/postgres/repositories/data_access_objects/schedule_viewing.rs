use anyhow::Result;
use diesel::prelude::*;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;

use crate::{
    domain::entities::schedule_view::ScheduleViewEntity,
    infrastructure::postgres::schema::{appointments, slots},
};

pub struct ScheduleViewingDao;

impl ScheduleViewingDao {
    pub async fn get_patient_schedules(
        conn: &mut AsyncPgConnection,
        patient_id: i32,
    ) -> Result<Vec<ScheduleViewEntity>> {
        let rows = appointments::table
            .inner_join(slots::table.on(slots::id.eq(appointments::slot_id)))
            .filter(appointments::deleted_at.is_null())
            .filter(slots::deleted_at.is_null())
            .filter(appointments::patient_id.eq(patient_id))
            .select((
                appointments::id,
                appointments::slot_id,
                appointments::patient_id,
                appointments::patient_abnormal_symptom,
                appointments::patient_is_missed_medication,
                appointments::patient_blood_test_status,
                appointments::patient_is_overdue_medication,
                appointments::patient_is_partner_hiv_positive,
                appointments::status,
                slots::doctor_id,
                slots::start_time,
                slots::end_time,
            ))
            .order((slots::start_time.asc(), appointments::created_at.asc()))
            .load::<ScheduleViewEntity>(conn)
            .await?;

        Ok(rows.into_iter().map(ScheduleViewEntity::from).collect())
    }

    pub async fn get_doctor_schedules(
        conn: &mut AsyncPgConnection,
        doctor_id: i32,
    ) -> Result<Vec<ScheduleViewEntity>> {
        let rows = appointments::table
            .inner_join(slots::table.on(slots::id.eq(appointments::slot_id)))
            .filter(appointments::deleted_at.is_null())
            .filter(slots::deleted_at.is_null())
            .filter(slots::doctor_id.eq(doctor_id))
            .select((
                appointments::id,
                appointments::slot_id,
                appointments::patient_id,
                appointments::patient_abnormal_symptom,
                appointments::patient_is_missed_medication,
                appointments::patient_blood_test_status,
                appointments::patient_is_overdue_medication,
                appointments::patient_is_partner_hiv_positive,
                appointments::status,
                slots::doctor_id,
                slots::start_time,
                slots::end_time,
            ))
            .order((slots::start_time.asc(), appointments::created_at.asc()))
            .load::<ScheduleViewEntity>(conn)
            .await?;

        Ok(rows.into_iter().map(ScheduleViewEntity::from).collect())
    }
}
