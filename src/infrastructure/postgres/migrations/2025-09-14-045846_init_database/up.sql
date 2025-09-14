-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE
    slots (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        doctor_id INTEGER NOT NULL,
        current_appointment_count INTEGER NOT NULL,
        max_appointment_count INTEGER NOT NULL,
        start_time TIMESTAMP NOT NULL,
        end_time TIMESTAMP NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT now (),
        updated_at TIMESTAMP NOT NULL DEFAULT now (),
        deleted_at TIMESTAMP
    );

CREATE TABLE
    appointments (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        slot_id UUID NOT NULL,
        patient_id INTEGER NOT NULL,
        patient_abnormal_symptom VARCHAR(255) NOT NULL,
        patient_is_missed_medication VARCHAR(255) NOT NULL,
        patient_blood_test_status VARCHAR(255) NOT NULL,
        patient_is_overdue_medication VARCHAR(255) NOT NULL,
        patient_is_partner_hiv_positive VARCHAR(255) NOT NULL,
        status VARCHAR(50) NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT now (),
        updated_at TIMESTAMP NOT NULL DEFAULT now (),
        deleted_at TIMESTAMP,
        CONSTRAINT fk_appointments_slot FOREIGN KEY (slot_id) REFERENCES slots (id) ON DELETE CASCADE
    );

CREATE INDEX idx_slots_doctor_id ON slots (doctor_id);

CREATE INDEX idx_appointments_slot_id ON appointments (slot_id);