use std::sync::Arc;

use anyhow::Result;
use diesel_async::{AsyncConnection, scoped_futures::ScopedFutureExt};
use uuid::Uuid;

use crate::{
    domain::{
        entities::appointments::{
            AddAppointmentEntity, EditAppointmentEntity, RescheduleAppointmentEntity,
        },
        repositories::appointment_ops::AppointmentOpsRepository,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad,
        repositories::data_access_objects::{
            appointment_ops::AppointmentOpsDao, appointment_viewing::AppointmentViewingDao,
            slot_ops::SlotOpsDao, slot_viewing::SlotViewingDao,
        },
    },
};

pub struct AppointmentOpsPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl AppointmentOpsPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

impl AppointmentOpsRepository for AppointmentOpsPostgres {
    async fn add(&self, add_appointment_entity: AddAppointmentEntity) -> Result<Uuid> {
        let mut conn = self.db_pool.get().await?;

        let appointment_id = conn
            .transaction(|conn| {
                async move {
                    let slot_id = add_appointment_entity.slot_id;

                    let end_time = SlotViewingDao::get_end_time_by_slot_id(conn, slot_id).await?;
                    let now = chrono::Utc::now().naive_utc();

                    if now > end_time {
                        return Err(anyhow::anyhow!("Slot is already ended!!!"));
                    }

                    SlotOpsDao::lock(conn, slot_id).await?;

                    let slot_is_not_full =
                        SlotOpsDao::try_add_slot_appointment_count(conn, slot_id).await?;

                    if !slot_is_not_full {
                        return Err(anyhow::anyhow!("Slot is full!!!"));
                    }

                    let appointment_id =
                        AppointmentOpsDao::add(conn, add_appointment_entity).await?;
                    anyhow::Ok(appointment_id)
                }
                .scope_boxed()
            })
            .await?;

        Ok(appointment_id)
    }

    async fn edit(
        &self,
        appointment_id: Uuid,
        patient_id: i32,
        edit_appointment_entity: EditAppointmentEntity,
    ) -> Result<Uuid> {
        let mut conn = self.db_pool.get().await?;

        let result = AppointmentOpsDao::edit(
            &mut conn,
            appointment_id,
            patient_id,
            edit_appointment_entity,
        )
        .await?;

        Ok(result)
    }

    async fn reschedule(
        &self,
        appointment_id: Uuid,
        patient_id: i32,
        reschedule_appointment_entity: RescheduleAppointmentEntity,
    ) -> Result<Uuid> {
        let mut conn = self.db_pool.get().await?;

        let appointment_effected_id = conn
            .transaction(|conn| {
                async move {
                    let new_slot_id = reschedule_appointment_entity.slot_id;
                    let end_time =
                        SlotViewingDao::get_end_time_by_slot_id(conn, new_slot_id).await?;
                    let now = chrono::Utc::now().naive_utc();

                    if now > end_time {
                        return Err(anyhow::anyhow!("Slot is already ended!!!"));
                    }

                    let old_slot_id =
                        AppointmentViewingDao::get_slot_id_by_appointment_id(conn, appointment_id)
                            .await?;

                    SlotOpsDao::lock(conn, new_slot_id).await?;
                    SlotOpsDao::lock(conn, old_slot_id).await?;

                    let slot_is_not_full =
                        SlotOpsDao::try_add_slot_appointment_count(conn, new_slot_id).await?;

                    if !slot_is_not_full {
                        return Err(anyhow::anyhow!("Slot is full!!!"));
                    }

                    SlotOpsDao::dec_slot_appointment_count(conn, old_slot_id).await?;
                    let appointment_effected_id = AppointmentOpsDao::reschedule(
                        conn,
                        appointment_id,
                        patient_id,
                        reschedule_appointment_entity,
                    )
                    .await?;

                    Ok(appointment_effected_id)
                }
                .scope_boxed()
            })
            .await?;

        Ok(appointment_effected_id)
    }

    async fn remove(&self, appointment_id: Uuid, patient_id: i32) -> Result<()> {
        let mut conn = self.db_pool.get().await?;
        conn.transaction(|conn| {
            async move {
                let slot_id =
                    AppointmentViewingDao::get_slot_id_by_appointment_id(conn, appointment_id)
                        .await?;
                SlotOpsDao::lock(conn, slot_id).await?;
                SlotOpsDao::dec_slot_appointment_count(conn, slot_id).await?;

                AppointmentOpsDao::remove(conn, appointment_id, patient_id).await?;
                anyhow::Ok(())
            }
            .scope_boxed()
        })
        .await?;

        Ok(())
    }
}
