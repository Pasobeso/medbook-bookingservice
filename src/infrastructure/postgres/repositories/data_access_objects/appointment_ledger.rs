use anyhow::Result;
use diesel::ExpressionMethods;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    domain::value_objects::appointment_status::AppointmentStatus,
    infrastructure::postgres::schema::appointments,
};

pub struct AppointmentLedgerDao;

impl AppointmentLedgerDao {
    pub async fn change_appointment_status(
        conn: &mut AsyncPgConnection,
        appointment_id: Uuid,
        appointment_status: AppointmentStatus,
    ) -> Result<Uuid> {
        let result = diesel::update(appointments::table)
            .filter(appointments::id.eq(appointment_id))
            .filter(appointments::deleted_at.is_null())
            .set((appointments::status.eq(appointment_status.to_string()),))
            .returning(appointments::id)
            .get_result::<Uuid>(conn)
            .await?;

        Ok(result)
    }
}
