use anyhow::Result;
use diesel::ExpressionMethods;
use diesel::dsl::insert_into;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::domain::entities::appointments::{EditAppointmentEntity, RescheduleAppointmentEntity};
use crate::domain::value_objects::appointment_status::AppointmentStatus;
use crate::{
    domain::entities::appointments::AddAppointmentEntity,
    infrastructure::postgres::schema::appointments,
};

pub struct AppointmentOpsDao;

impl AppointmentOpsDao {
    // pub async fn lock(conn: &mut AsyncPgConnection, appointment_id: Uuid) -> Result<()> {
    //     let n = diesel::sql_query(
    //         r#"
    //         SELECT 1
    //           FROM appointments
    //          WHERE id = $1
    //            AND deleted_at IS NULL
    //          FOR UPDATE
    //     "#,
    //     )
    //     .bind::<SqlUuid, _>(appointment_id)
    //     .execute(conn)
    //     .await?;

    //     if n == 0 {
    //         return Err(anyhow!("Appointment not found"));
    //     }
    //     Ok(())
    // }

    pub async fn add(
        conn: &mut AsyncPgConnection,
        add_appointment_entity: AddAppointmentEntity,
    ) -> Result<Uuid> {
        let result = insert_into(appointments::table)
            .values(add_appointment_entity)
            .returning(appointments::id)
            .get_result::<Uuid>(conn)
            .await?;

        Ok(result)
    }

    pub async fn edit(
        conn: &mut AsyncPgConnection,
        appointment_id: Uuid,
        patient_id: i32,
        edit_appointment_entity: EditAppointmentEntity,
    ) -> Result<Uuid> {

        let result = diesel::update(appointments::table)
            .filter(appointments::id.eq(appointment_id))
            .filter(appointments::patient_id.eq(patient_id))
            .filter(appointments::deleted_at.is_null())
            .filter(appointments::status.eq(AppointmentStatus::Waiting.to_string()))
            .set(edit_appointment_entity)
            .returning(appointments::id)
            .get_result::<Uuid>(conn)
            .await?;

        Ok(result)
    }

    pub async fn reschedule(
        conn: &mut AsyncPgConnection,
        appointment_id: Uuid,
        patient_id: i32,
        reschedule_appointment_entity: RescheduleAppointmentEntity,
    ) -> Result<Uuid> {

        let result = diesel::update(appointments::table)
            .filter(appointments::id.eq(appointment_id))
            .filter(appointments::patient_id.eq(patient_id))
            .filter(appointments::deleted_at.is_null())
            .filter(appointments::status.eq(AppointmentStatus::Waiting.to_string()))
            .set(reschedule_appointment_entity)
            .returning(appointments::id)
            .get_result::<Uuid>(conn)
            .await?;

        Ok(result)
    }

    pub async fn remove(
        conn: &mut AsyncPgConnection,
        appointment_id: Uuid,
        patient_id: i32,
    ) -> Result<()> {
        diesel::update(appointments::table)
            .filter(appointments::id.eq(appointment_id))
            .filter(appointments::patient_id.eq(patient_id))
            .filter(appointments::deleted_at.is_null())
            .filter(appointments::status.eq(AppointmentStatus::Waiting.to_string()))
            .set((appointments::deleted_at.eq(chrono::Utc::now().naive_utc()),))
            .execute(conn)
            .await?;

        Ok(())
    }
}
