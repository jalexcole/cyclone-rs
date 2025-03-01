use std::{
    any::Any,
    marker::PhantomData,
    os::raw::c_void,
    time::{Duration, Instant},
};

use cyclonedds_sys::dds_delete;

use crate::{
    core::ReturnCodes, domain::DomainParticipant, internal::InstanceHandle, topic::TopicType,
};

pub struct Publisher {
    pub(crate) publisher: cyclonedds_sys::dds_entity_t,
}

impl Publisher {
    pub fn suspend(&mut self) -> Result<(), ReturnCodes> {
        match unsafe { cyclonedds_sys::dds_suspend(self.publisher) } {
            0 => Ok(()),
            result => Err(ReturnCodes::from(result)),
        }
    }

    pub fn resume(&mut self) -> Result<(), ReturnCodes> {
        match unsafe { cyclonedds_sys::dds_resume(self.publisher) } {
            0 => Ok(()),
            result => Err(ReturnCodes::from(result)),
        }
    }

    pub fn create_datawriter<T: TopicType>(&mut self) -> Result<DataWriter<T>, ReturnCodes> {
        todo!("not implemented")
    }

    pub fn create_anydatawriter(&mut self) -> Result<AnyDataWriter, ReturnCodes> {
        todo!("not implemented")
    }
    /// Waits at most for the duration timeout for acks for data in the
    /// publisher or writer.
    ///
    /// This operation blocks the calling thread until either all data written
    /// by the publisher or writer is acknowledged by all matched reliable
    /// reader entities, or else the duration specified by the timeout parameter
    /// elapses, whichever happens first.
    pub fn wait_for_acks(&mut self, timeout: Duration) -> Result<(), ReturnCodes> {
        match unsafe {
            cyclonedds_sys::dds_wait_for_acks(self.publisher, timeout.as_nanos() as i64)
        } {
            0 => Ok(()),
            result => Err(ReturnCodes::from(result)),
        }
    }
}

impl TryFrom<DomainParticipant> for Publisher {
    type Error = ReturnCodes;
    fn try_from(value: DomainParticipant) -> Result<Self, Self::Error> {
        let entity_handle = unsafe {
            cyclonedds_sys::dds_create_publisher(
                value.participant,
                std::ptr::null(),
                std::ptr::null(),
            )
        };

        if entity_handle < 0 {
            Err(ReturnCodes::from(entity_handle))
        } else {
            Ok(Publisher {
                publisher: entity_handle,
            })
        }
    }
}

pub struct DataWriter<T: TopicType> {
    writer: cyclonedds_sys::dds_entity_t,
    _marker: PhantomData<T>,
}

impl<T: TopicType> DataWriter<T> {
    /// Get PUBLICATION_MATCHED status.
    ///
    /// This operation gets the status value corresponding to
    /// PUBLICATION_MATCHED and reset the status. The value can be obtained,
    /// only if the status is enabled for an entity. NULL value for status is
    /// allowed and it will reset the trigger value when status is enabled.
    pub fn publication_matched_status(&self) -> Result<PublicationMatchedStatus, ReturnCodes> {
        unsafe {
            let mut status = cyclonedds_sys::dds_publication_matched_status_t {
                total_count: 0,
                total_count_change: 0,
                current_count: 0,
                current_count_change: 0,
                last_subscription_handle: 0,
            };

            let result =
                cyclonedds_sys::dds_get_publication_matched_status(self.writer, &mut status);

            if result != 0 {
                return Err(ReturnCodes::from(result));
            }

            Ok(PublicationMatchedStatus { status })
        }
    }
    /// This operation disposes an instance with a specific timestamp,
    /// identified by the instance handle.
    ///
    /// This operation performs the same functions as [dds_dispose_ih()]
    /// except that the application provides the value for the source_timestamp
    /// that is made available to connected reader objects. This timestamp is
    /// important for the interpretation of the destination_order QoS policy.
    pub fn dispose_ih_ts(
        &self,
        handle: cyclonedds_sys::dds_instance_handle_t,
        timestamp: &Instant,
    ) -> Result<(), ReturnCodes> {
        match unsafe {
            cyclonedds_sys::dds_dispose_ih_ts(
                self.writer,
                handle,
                timestamp.elapsed().as_nanos() as i64,
            )
        } {
            0 => Ok(()),
            result => Err(ReturnCodes::from(result)),
        }
    }

    /// Write the value of a data instance
    ///
    /// With this API, the value of the source timestamp is automatically made
    /// available to the data reader by the service.
    pub fn write(&mut self, data: &T) -> Result<(), ReturnCodes>
    where
        T: serde::Serialize,
    {
        // Serialize the data to a byte vector.
        let serialized = bincode::serialize(data).map_err(|e| ReturnCodes::Unsupported)?;

        // Write the serialized data to DDS.
        // Note: Ensure that `dds_write` expects the serialized layout.
        match unsafe { cyclonedds_sys::dds_write(self.writer, serialized.as_ptr() as *mut c_void) }
        {
            0 => Ok(()),
            result => Err(ReturnCodes::from(result)),
        }
    }
    /// Flush a writers batched writes
    ///
    /// When using write batching you can manually batch small writes into
    /// larger datapackets for network efficiency. The normal
    /// [DataWriter::write()] no longer guarantee that data is sent on the
    /// network automatically.
    pub fn write_flush(&mut self) -> Result<(), ReturnCodes> {
        todo!("not implemented")
    }
    /// Write a serialized value of a data instance
    ///
    /// This call causes the writer to write the serialized value that is
    /// provided in the serdata argument. Timestamp and statusinfo fields are
    /// set to the current time and 0 (indicating a regular write),
    /// respectively.
    pub fn write_cdr(&mut self, data: &[u8]) -> Result<(), ReturnCodes> {
        match unsafe {
            cyclonedds_sys::dds_writecdr(
                self.writer,
                data.as_ptr() as *mut cyclonedds_sys::ddsi_serdata,
            )
        } {
            0 => Ok(()),
            result => Err(ReturnCodes::from(result)),
        }
    }

    /// Write a serialized value of a data instance
    ///
    /// This call causes the writer to write the serialized value that is
    /// provided in the serdata argument. Timestamp and statusinfo are used as
    /// is.
    pub fn forwardcdr(
        &mut self,
        data: *mut cyclonedds_sys::ddsi_serdata,
    ) -> Result<(), ReturnCodes> {
        todo!("not implemented")
    }

    /// Write the value of a data instance along with the source timestamp passed.
    pub fn write_ts(&self, data: &T, timestamp: &Instant) -> Result<(), ReturnCodes> {
        todo!("not implemented")
    }

    /// Waits at most for the duration timeout for acks for data in the
    /// publisher or writer.
    ///
    /// This operation blocks the calling thread until either all data written
    /// by the publisher or writer is acknowledged by all matched reliable
    /// reader entities, or else the duration specified by the timeout parameter
    /// elapses, whichever happens first.
    pub fn wait_for_acks(&mut self, timeout: Duration) -> Result<(), ReturnCodes> {
        match unsafe { cyclonedds_sys::dds_wait_for_acks(self.writer, timeout.as_nanos() as i64) } {
            0 => Ok(()),
            result => Err(ReturnCodes::from(result)),
        }
    }
}

impl<T: TopicType> Drop for DataWriter<T> {
    fn drop(&mut self) {
        match unsafe { dds_delete(self.writer) } {
            0 => (),
            _ => panic!("Failed to delete writer, "),
        }
    }
}

pub struct AnyDataWriter {
    writer: cyclonedds_sys::dds_entity_t,
}

impl AnyDataWriter {
    /// Waits at most for the duration timeout for acks for data in the
    /// publisher or writer.
    ///
    /// This operation blocks the calling thread until either all data written
    /// by the publisher or writer is acknowledged by all matched reliable
    /// reader entities, or else the duration specified by the timeout parameter
    /// elapses, whichever happens first.
    pub fn wait_for_acks(&mut self, timeout: Duration) -> Result<(), ReturnCodes> {
        match unsafe { cyclonedds_sys::dds_wait_for_acks(self.writer, timeout.as_nanos() as i64) } {
            0 => Ok(()),
            result => Err(ReturnCodes::from(result)),
        }
    }

    /// Registers an instance
    ///
    /// This operation registers an instance with a key value to the data
    /// writer and returns an instance handle that could be used for successive
    /// write & dispose operations. When the handle is not allocated, the
    /// function will return an error and the handle will be un-touched.
    pub fn register_instance(&mut self, data: &impl Any) -> Result<InstanceHandle, ReturnCodes> {
        todo!("not implemented")
    }

    pub fn unregister_instance(&mut self, data: &impl Any) -> Result<(), ReturnCodes> {
        todo!("not implemented")
    }
}

impl<T: TopicType> From<DataWriter<T>> for AnyDataWriter {
    fn from(value: DataWriter<T>) -> Self {
        todo!()
    }
}

impl Drop for AnyDataWriter {
    fn drop(&mut self) {
        match unsafe { dds_delete(self.writer) } {
            0 => (),
            _ => panic!("Failed to delete writer, "),
        }
    }
}

pub struct PublicationMatchedStatus {
    status: cyclonedds_sys::dds_publication_matched_status_t,
}

impl PublicationMatchedStatus {
    pub fn current_count(&self) -> u32 {
        self.status.current_count
    }
}
