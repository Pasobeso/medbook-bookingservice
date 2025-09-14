use std::sync::Arc;

use anyhow::{Result, anyhow};
use axum::async_trait;
use diesel_async::{AsyncConnection, scoped_futures::ScopedFutureExt};
use uuid::Uuid;

use crate::{
    domain::{entities::slots::AddSlotEntity, repositories::slot_ops::SlotOpsRepository},
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad,
        repositories::data_access_objects::{slot_ops::SlotOpsDao, slot_viewing::SlotViewingDao},
    },
};

pub struct SlotOpsPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl SlotOpsPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl SlotOpsRepository for SlotOpsPostgres {
    async fn add(&self, add_slot_entity: AddSlotEntity) -> Result<Uuid> {
        let mut conn = self.db_pool.get().await?;

        let slot_id = conn
            .transaction(|conn| {
                async move {
                    let is_overlapping_slot = SlotViewingDao::is_overlapping_slots_for_doctor_id(
                        conn,
                        add_slot_entity.start_time,
                        add_slot_entity.end_time,
                        add_slot_entity.doctor_id,
                    )
                    .await?;

                    if is_overlapping_slot {
                        return Err(anyhow::anyhow!("Slot time is overlapping!!!"));
                    }

                    let slot_id = SlotOpsDao::add(conn, add_slot_entity).await?;
                    Ok(slot_id)
                }
                .scope_boxed()
            })
            .await?;

        Ok(slot_id)
    }

    async fn remove(&self, slot_id: Uuid, doctor_id: i32) -> Result<()> {
        let mut conn = self.db_pool.get().await?;
        conn.transaction(|conn| {
            async move {
                SlotOpsDao::lock(conn, slot_id).await?;
                let appointment_count =
                    SlotViewingDao::get_current_appointment_count_by_slot_id(conn, slot_id).await?;
                if appointment_count > 0 {
                    return Err(anyhow!("Patient already booked this slot!"));
                }

                SlotOpsDao::remove(conn, slot_id, doctor_id).await?;
                Ok(())
            }
            .scope_boxed()
        })
        .await?;

        Ok(())
    }
}
