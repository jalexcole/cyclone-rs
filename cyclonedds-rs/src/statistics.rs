

use cyclonedds_sys::dds_delete_statistics;

use crate::{Entity, ReturnCodes};


pub enum StatKind {
    UInt32, UInt64, LengthTime
}


pub struct Statistics {
    statistics: *mut cyclonedds_sys::dds_statistics
}

impl Statistics {
    pub fn create(entity: &impl Entity) -> Result<Statistics, ReturnCodes> {
        todo!("not implemented")
    }

    pub fn refresh(&mut self) -> Result<(), ReturnCodes> {
        match unsafe { cyclonedds_sys::dds_refresh_statistics(self.statistics) } {
            0 => Ok(()),
            _ => todo!("Implement Errors"),
        }
    }

    
}

impl Drop for Statistics {
    fn drop(&mut self) {
        unsafe { dds_delete_statistics(self.statistics) };
    }
}