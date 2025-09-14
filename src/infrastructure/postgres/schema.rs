// @generated automatically by Diesel CLI.

diesel::table! {
    appointments (id) {
        id -> Uuid,
        slot_id -> Uuid,
        patient_id -> Int4,
        #[max_length = 255]
        patient_abnormal_symptom -> Varchar,
        #[max_length = 255]
        patient_is_missed_medication -> Varchar,
        #[max_length = 255]
        patient_blood_test_status -> Varchar,
        #[max_length = 255]
        patient_is_overdue_medication -> Varchar,
        #[max_length = 255]
        patient_is_partner_hiv_positive -> Varchar,
        #[max_length = 50]
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    slots (id) {
        id -> Uuid,
        doctor_id -> Int4,
        current_appointment_count -> Int4,
        max_appointment_count -> Int4,
        start_time -> Timestamp,
        end_time -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(appointments -> slots (slot_id));

diesel::allow_tables_to_appear_in_same_query!(
    appointments,
    slots,
);
