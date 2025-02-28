


// safe.rs

use cyclonedds_sys::*;
use core::panic;
use std::ffi::c_uint;
use std::os::raw::c_int;
use std::ptr;

use crate::{publisher::Publisher, subscriber::{self, Subscriber}};

/// A safe wrapper around a Cyclone DDS Participant.
///
/// In Cyclone DDS, participants are represented by the dds_entity_t type.
pub struct DomainParticipant {
    pub(super) participant: dds_entity_t,
}

impl DomainParticipant {
    /// Creates a new Participant for the given domain.
    ///
    /// Returns a `Result` with a safe Participant on success,
    /// or the negative error code on failure.
    pub fn new(domain_id: u32) -> Result<Self, c_int> {
        unsafe {
            // Create a participant. The `dds_create_participant` function returns the participant handle
            // or a negative error code if creation fails.
            let participant = dds_create_participant(domain_id as c_uint, ptr::null(), ptr::null());

            if participant < 0 {
                Err(participant as c_int)
            } else {
                Ok(DomainParticipant { participant })
            }
        }
    }

    /// Example method that could wrap an operation on the participant.
    ///
    /// Replace this with actual functionality as needed.
    pub fn do_something(&self) {
        unsafe {
            // For example, if there's a function to enable discovery on the participant,
            // you could call it here:
            // dds_enable_discovery(self.inner);
        }
    }

    pub fn subscriber(&mut self) -> Subscriber {
        Subscriber::new(&self)
    }

    pub fn publisher(&mut self) -> Publisher {
        todo!("not implemented")
    }
}

impl Drop for DomainParticipant {
    fn drop(&mut self) {
        unsafe {
            if self.participant >= 0 {
                dds_delete(self.participant);
            } else {
                panic!("Failed to delete participant");
            }
        }
    }
}


pub trait DomainParticipantListener: Drop {

}


pub mod qos {
    pub struct DomainParticipantQos {}
}

#[cfg(test)]
mod test {

    #[test]
    fn test_participant() {
        let participant = super::DomainParticipant::new(0).unwrap();
        drop(participant);
    }
}