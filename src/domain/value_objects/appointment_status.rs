use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AppointmentStatus {
    #[default]
    Waiting,
    Ready,
    WaitingForPrescription,
    Completed,
}

//finding a way to derive string from this enum

impl fmt::Display for AppointmentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppointmentStatus::Waiting => write!(f, "Waiting"),
            AppointmentStatus::Ready => write!(f, "Ready"),
            AppointmentStatus::WaitingForPrescription => write!(f, "WaitingForPrescription"),
            AppointmentStatus::Completed => write!(f, "Completed"),
        }
    }
}
