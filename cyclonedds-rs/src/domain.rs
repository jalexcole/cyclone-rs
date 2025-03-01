


// safe.rs
use core::panic;
use std::ffi::c_uint;
use std::os::raw::c_int;
use std::ptr;

use crate::{core::{qos::Qos, ReturnCodes}, publisher::Publisher, subscriber::Subscriber, topic::{AnyTopic, Topic, TopicType}};

/// A safe wrapper around a Cyclone DDS Participant.
///
/// In Cyclone DDS, participants are represented by the dds_entity_t type.
pub struct DomainParticipant {
    pub(super) participant: cyclonedds_sys::dds_entity_t,
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
            let participant = cyclonedds_sys::dds_create_participant(domain_id as c_uint, ptr::null(), ptr::null());

            if participant < 0 {
                Err(participant as c_int)
            } else {
                Ok(DomainParticipant { participant })
            }
        }
    }

    pub fn subscriber(&mut self) -> Subscriber {
        Subscriber::new(&self)
    }

    pub fn publisher(&mut self) -> Result<Publisher, ReturnCodes> {
        let entity_handle = unsafe {
            cyclonedds_sys::dds_create_publisher(
                self.participant,
                ptr::null(),
                ptr::null(),
            )
        };

        if entity_handle < 0 {
            Err(ReturnCodes::from(entity_handle))
        } else {
            Ok(Publisher { publisher: entity_handle })
        }

    }

    pub fn topic<T: TopicType>(&mut self) -> Result<Topic<T>, ReturnCodes> {
        todo!()
    }

    pub fn any_tpic(&mut self) -> Result<AnyTopic, ReturnCodes> {
        todo!()
    }

    pub fn qos(&self) -> Result<Qos, ReturnCodes> {
        todo!()
    }
}

impl Drop for DomainParticipant {
    fn drop(&mut self) {
        unsafe {
            if self.participant >= 0 {
                cyclonedds_sys::dds_delete(self.participant);
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
    use std::time::Duration;

    use crate::{domain::DomainParticipant, ReliabilityKind};


    #[test]
    fn test_participant() {
        let participant = super::DomainParticipant::new(0).unwrap();
        drop(participant);
    }
   

    

    // Test that a DomainParticipant cleans up without error.
    #[test]
    fn test_participant_lifecycle() {
        {
            let participant = DomainParticipant::new(0).expect("Failed to create participant");
            // Optionally, do something with the participant here.
        }
        // After the scope ends, the participant is dropped.
        // Pause briefly to allow asynchronous cleanup if necessary.
        std::thread::sleep(Duration::from_millis(50));
        // If no panic occurs, the test passes.
    }

   
}