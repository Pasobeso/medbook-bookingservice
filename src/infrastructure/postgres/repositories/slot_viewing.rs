use std::sync::Arc;

use anyhow::Result;

use crate::{
    domain::{entities::slots::SlotEntity, repositories::slot_viewing::SlotViewingRepository},
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad,
        repositories::data_access_objects::slot_viewing::SlotViewingDao,
    },
};

pub struct SlotViewingPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl SlotViewingPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

impl SlotViewingRepository for SlotViewingPostgres {
    async fn get_slots(&self) -> Result<Vec<SlotEntity>> {
        let mut conn = self.db_pool.get().await?;
        let slots = SlotViewingDao::get_slots(&mut conn).await?;

        Ok(slots)
    }

    async fn get_doctor_slots(&self, doctor_id: i32) -> Result<Vec<SlotEntity>> {
        let mut conn = self.db_pool.get().await?;
        let slots = SlotViewingDao::get_doctor_slots(&mut conn, doctor_id).await?;

        Ok(slots)
    }
}
