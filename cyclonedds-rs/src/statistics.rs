use std::{
    ffi::{c_char, CString},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use cyclonedds_sys::dds_delete_statistics;

use crate::core::{Entity, FetchableEntity, ReturnCodes};

/// Kind of statistical value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatKind {
    /// value is a 32-bit unsigned integer
    UInt32(u32),
    /// value is a 64-bit unsigned integer
    UInt64(u64),
    /// value is integral(length(t) dt)
    LengthTime(u64),
}

pub struct Statistics {
    statistics: *mut cyclonedds_sys::dds_statistics,
}

impl Statistics {
    /// Allocate a new statistics object for [Entity]
    ///
    /// This allocates and populates a newly allocated struct [Statistics] for
    /// the specified entity.
    /// * `entity` the handle of the entity
    pub(crate) fn create(entity: &impl FetchableEntity) -> Result<Statistics, ReturnCodes> {
        let result = unsafe { cyclonedds_sys::dds_create_statistics(entity.fetch()) };

        if result.is_null() {
            Err(ReturnCodes::Error)
        } else {
            Ok(Statistics { statistics: result })
        }
    }
    /// Update a previously created statistics structure with current values
    ///
    /// Only the time stamp and the values (and “opaque”) may change. The set
    /// of keys and the types of the values do not change.
    pub fn refresh(&mut self) -> Result<(), ReturnCodes> {
        match unsafe { cyclonedds_sys::dds_refresh_statistics(self.statistics) } {
            0 => Ok(()),
            _ => todo!("Implement Errors"),
        }
    }

    /// Lookup a specific value by name
    ///
    /// This looks up the specified name in the list of keys in stat and
    /// returns the address of the key-value pair if present, a null pointer if
    /// not. If stat is a null pointer, it returns a null pointer.
    pub fn lookup(&self, name: &str) -> Result<KeyValue, ReturnCodes> {
        let key_value: *const cyclonedds_sys::dds_stat_keyvalue = unsafe {
            cyclonedds_sys::dds_lookup_statistic(
                self.statistics,
                CString::new(name).unwrap().as_ptr(),
            )
        };

        if key_value.is_null() {
            Err(ReturnCodes::Error)
        } else {
            // Dereference the pointer to get a value and convert it.
            Ok(unsafe { KeyValue::from(*key_value) })
        }
    }

    pub fn time(&self) -> SystemTime {
        let time = unsafe { self.statistics.as_ref().unwrap().time };
        let duration = Duration::from_nanos(time as u64);

        UNIX_EPOCH + duration
    }

    pub fn count(&self) -> usize {
        unsafe { self.statistics.as_ref().unwrap().count }
    }

    pub fn kv(&self) -> Vec<KeyValue> {
        let keys = unsafe { self.statistics.as_ref().unwrap().kv.as_slice(self.count()) };

        keys.iter().map(|k| (*k).into()).collect()
    }
}

impl Drop for Statistics {
    fn drop(&mut self) {
        unsafe { dds_delete_statistics(self.statistics) };
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct KeyValue {
    name: String,
    kind: StatKind,
}

impl KeyValue {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn kind(&self) -> &StatKind {
        &self.kind
    }
}

impl From<cyclonedds_sys::dds_stat_keyvalue> for KeyValue {
    fn from(value: cyclonedds_sys::dds_stat_keyvalue) -> Self {
        let name = unsafe { CString::from_raw(value.name as *mut c_char) };

        let stat_kind = match value.kind {
            cyclonedds_sys::dds_stat_kind::DDS_STAT_KIND_UINT32 => todo!(),
            cyclonedds_sys::dds_stat_kind::DDS_STAT_KIND_UINT64 => todo!(),
            cyclonedds_sys::dds_stat_kind::DDS_STAT_KIND_LENGTHTIME => todo!(),
                    };

        KeyValue {
            name: name.to_str().unwrap().to_string(),
            kind: stat_kind,
        }
    }
}
