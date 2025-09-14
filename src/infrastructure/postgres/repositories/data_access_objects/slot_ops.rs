use anyhow::{Result, anyhow};
use diesel::query_dsl::methods::FilterDsl;
use diesel::sql_types::Uuid as SqlUuid;
use diesel::{ExpressionMethods, dsl::insert_into};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{domain::entities::slots::AddSlotEntity, infrastructure::postgres::schema::slots};

pub struct SlotOpsDao;

impl SlotOpsDao {
    pub async fn lock(conn: &mut AsyncPgConnection, slot_id: uuid::Uuid) -> Result<()> {
        let n = diesel::sql_query(
            r#"
            SELECT 1
              FROM slots
             WHERE id = $1
               AND deleted_at IS NULL
             FOR UPDATE
        "#,
        )
        .bind::<SqlUuid, _>(slot_id)
        .execute(conn)
        .await?;

        if n == 0 {
            return Err(anyhow!("slot not found"));
        }
        Ok(())
    }

    pub async fn add(conn: &mut AsyncPgConnection, add_slot_entity: AddSlotEntity) -> Result<Uuid> {
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

    pub async fn try_add_slot_appointment_count(
        conn: &mut AsyncPgConnection,
        id: Uuid,
    ) -> Result<bool> {
        let result = diesel::update(
            slots::table
                .filter(slots::id.eq(id))
                .filter(slots::deleted_at.is_null())
                .filter(slots::current_appointment_count.lt(slots::max_appointment_count)),
        )
        .set((
            slots::current_appointment_count.eq(slots::current_appointment_count + 1),
            slots::updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .returning(slots::id)
        .get_result::<Uuid>(conn)
        .await;

        match result {
            Ok(_) => Ok(true),
            Err(diesel::result::Error::NotFound) => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn dec_slot_appointment_count(conn: &mut AsyncPgConnection, id: Uuid) -> Result<()> {
        diesel::update(
            slots::table
                .filter(slots::id.eq(id))
                .filter(slots::deleted_at.is_null()),
        )
        .set((
            slots::current_appointment_count.eq(slots::current_appointment_count - 1),
            slots::updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(conn)
        .await?;

        Ok(())
    }
}
