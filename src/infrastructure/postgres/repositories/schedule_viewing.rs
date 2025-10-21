use std::sync::Arc;

use anyhow::Result;

use crate::{
    domain::{
        entities::schedule_view::ScheduleViewEntity,
        repositories::schedule_viewing::ScheduleViewingRepository,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad,
        repositories::data_access_objects::schedule_viewing::ScheduleViewingDao,
    },
};

pub struct ScheduleViewingPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl ScheduleViewingPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

impl ScheduleViewingRepository for ScheduleViewingPostgres {
    async fn get_patient_schedules(&self, patient_id: i32) -> Result<Vec<ScheduleViewEntity>> {
        let mut conn = self.db_pool.get().await?;
        let schedules = ScheduleViewingDao::get_patient_schedules(&mut conn, patient_id).await?;

        Ok(schedules)
    }

    async fn get_doctor_schedules(&self, doctor_id: i32) -> Result<Vec<ScheduleViewEntity>> {
        let mut conn = self.db_pool.get().await?;
        let schedules = ScheduleViewingDao::get_doctor_schedules(&mut conn, doctor_id).await?;

        Ok(schedules)
    }
}
