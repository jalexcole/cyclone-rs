


// safe.rs

use cyclonedds_sys::*;
use std::os::raw::c_int;
use std::ptr;

/// A safe wrapper around a Cyclone DDS Participant.
///
/// In Cyclone DDS, participants are represented by the dds_entity_t type.
pub struct Participant {
    inner: dds_entity_t,
}

impl Participant {
    /// Creates a new Participant for the given domain.
    ///
    /// Returns a `Result` with a safe Participant on success,
    /// or the negative error code on failure.
    pub fn new(domain_id: i32) -> Result<Self, c_int> {
        let mut participant: dds_entity_t = 0;
        // This call is unsafe because it works with raw pointers.
        let ret = unsafe {
            // Create a participant. Here, we pass null pointers for QoS and listener.
            // Adjust parameters as required by your cyclonedds-sys bindings.
            // dds_create_participant(&mut participant as *mut dds_entity_t, domain_id, ptr::null())
        };

        // if ret < 0 {
        //     Err(ret)
        // } else {
        //     Ok(Participant { inner: participant })
        // }
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
}

impl Drop for Participant {
    fn drop(&mut self) {
        unsafe {
            // Clean up the participant resource when Participant goes out of scope.
            // Note: Some DDS implementations require a different or additional cleanup.
            dds_delete(self.inner);
        }
    }
}