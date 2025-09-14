use anyhow::{Result, anyhow};
use diesel::sql_types::{Int4, Uuid as SqlUuid};
use diesel::{ExpressionMethods, dsl::insert_into};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{domain::entities::slots::AddSlotEntity, infrastructure::postgres::schema::slots};

pub struct SlotOpsDao;

impl SlotOpsDao {
    pub async fn lock(
        conn: &mut AsyncPgConnection,
        slot_id: uuid::Uuid,
        doctor_id: i32,
    ) -> Result<()> {
        let n = diesel::sql_query(
            r#"
            SELECT 1
              FROM slots
             WHERE id = $1
               AND doctor_id = $2
               AND deleted_at IS NULL
             FOR UPDATE
        "#,
        )
        .bind::<SqlUuid, _>(slot_id)
        .bind::<Int4, _>(doctor_id)
        .execute(conn)
        .await?;

        if n == 0 {
            return Err(anyhow!("slot not found or not owned"));
        }
        Ok(())
    }

    pub async fn add(
        conn: &mut AsyncPgConnection,
        add_slot_entity: AddSlotEntity,
    ) -> Result<Uuid> {
        let result = insert_into(slots::table)
            .values(add_slot_entity)
            .returning(slots::id)
            .get_result::<Uuid>(conn)
            .await?;

        Ok(result)
    }

    pub async fn remove(conn: &mut AsyncPgConnection, slot_id: Uuid, doctor_id: i32) -> Result<()> {
        diesel::update(slots::table)
            .filter(slots::id.eq(slot_id))
            .filter(slots::doctor_id.eq(doctor_id))
            .filter(slots::deleted_at.is_null())
            .filter(slots::start_time.gt(chrono::Utc::now().naive_utc()))
            .set((slots::deleted_at.eq(chrono::Utc::now().naive_utc()),))
            .execute(conn)
            .await?;

        Ok(())
    }
}
