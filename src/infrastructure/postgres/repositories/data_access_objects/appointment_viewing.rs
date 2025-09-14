use anyhow::Result;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::infrastructure::postgres::schema::appointments;

pub struct AppointmentViewingDao;

impl AppointmentViewingDao {

    pub async fn get_slot_id_by_appointment_id(
        conn: &mut AsyncPgConnection,
        appointment_id: Uuid,
    ) -> Result<Uuid> {
        let result = appointments::table
            .filter(appointments::deleted_at.is_null())
            .filter(appointments::id.eq(appointment_id))
            .select(appointments::slot_id)   
            .first::<Uuid>(conn)                        
            .await?;

        Ok(result)
    }
}
