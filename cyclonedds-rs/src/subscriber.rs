use std::{
    marker::PhantomData,
    ptr::{null, null_mut},
    time::{Duration, SystemTime},
};

use crate::{
    core::{FetchableEntity, ReturnCodes},
    domain::DomainParticipant,
    internal::InstanceHandle,
    topic::{Topic, TopicType},
};

pub struct Subscriber {
    subscriber: cyclonedds_sys::dds_entity_t,
}

impl Subscriber {
    /// Creates a new instance of a DDS subscriber.
    pub(crate) fn new(participant: &DomainParticipant) -> Result<Subscriber, ReturnCodes> {
        let subscriber = unsafe {
            cyclonedds_sys::dds_create_subscriber(participant.participant, null_mut(), null())
        };

        if subscriber < 0 {
            Err(ReturnCodes::from(subscriber))
        } else {
            Ok(Subscriber { subscriber })
        }
    }

    /// Trigger DATA_AVAILABLE event on contained readers
    ///
    /// The DATA_AVAILABLE event is broadcast to all readers owned by this
    /// [Subscriber] that currently have new data available. Any
    /// on_data_available listener callbacks attached to respective readers
    /// are invoked.
    pub fn notify_readers(&self) -> Result<(), ReturnCodes> {
        match unsafe { cyclonedds_sys::dds_notify_readers(self.subscriber) } {
            0 => Ok(()),
            result => Err(ReturnCodes::from(result)),
        }
    }

    pub fn create_reader<T: TopicType>(
        &self,
        topic: &Topic<T>,
    ) -> Result<DataReader<T>, ReturnCodes> {
        unsafe {
            let reader = cyclonedds_sys::dds_create_reader(
                self.subscriber,
                topic.topic,
                null(),
                std::ptr::null_mut(),
            );
            if reader < 0 {
                Err(ReturnCodes::from(reader))
            } else {
                Ok(DataReader {
                    reader,
                    _marker: PhantomData,
                })
            }
        }
    }
}

impl TryFrom<DomainParticipant> for Subscriber {
    type Error = ReturnCodes;

    fn try_from(mut value: DomainParticipant) -> Result<Self, ReturnCodes> {
        value.subscriber()
    }
}

impl Drop for Subscriber {
    fn drop(&mut self) {
        unsafe {
            cyclonedds_sys::dds_delete(self.subscriber);
        }
    }
}

impl FetchableEntity for Subscriber {
    fn fetch(&self) -> cyclonedds_sys::dds_entity_t {
        self.subscriber
    }
}

/// [DataReader] allows the application to access published sample data.
///
/// A DataReader allows the application:
///
/// to declare the data it wishes to receive (i.e., make a subscription)
/// to access the data received by the attached [Subscriber]
///
/// A DataReader refers to exactly one TopicDescription (either a [Topic], a
/// [ContentFilteredTopic] or a [MultiTopic]) that identifies the samples to be
/// read. The Topic must exist prior to the DataReader creation.
///
/// A DataReader is attached to exactly one Subscriber which acts as a factory
/// for it.
///
/// The DataReader may give access to several instances of the data type, which
/// are distinguished from each other by their key.
///
/// The pre-processor generates from IDL type descriptions the application
/// DataReader<type> classes. For each application data type that is used as
/// Topic data type, a typed class DataReader<type> is derived from the
/// [AnyDataReader] class.
pub struct DataReader<T: TopicType> {
    reader: cyclonedds_sys::dds_entity_t,
    _marker: PhantomData<T>,
}

impl<T: TopicType> DataReader<T> {
    pub(crate) fn new(
        subscriber: &mut Subscriber,
        topic: &Topic<T>,
    ) -> Result<DataReader<T>, ReturnCodes> {
        let data_reader = unsafe {
            cyclonedds_sys::dds_create_reader(
                subscriber.subscriber,
                topic.topic,
                null(),
                std::ptr::null_mut(),
            )
        };

        if data_reader < 0 {
            Err(ReturnCodes::from(data_reader))
        } else {
            Ok(DataReader {
                reader: data_reader,
                _marker: PhantomData::default(),
            })
        }
    }

    /// Wait until reader receives all historic data
    ///
    /// The operation blocks the calling thread until either all “historical”
    /// data is received, or else the duration specified by the max_wait
    /// parameter elapses, whichever happens first. A return value of 0
    /// indicates that all the “historical” data was received; a return value
    /// of TIMEOUT indicates that max_wait elapsed before all the data was received.
    pub fn wait_for_historical_data(&self, duration: Duration) -> Result<(), ReturnCodes> {
        let return_code = unsafe {
            cyclonedds_sys::dds_reader_wait_for_historical_data(
                self.reader,
                duration.as_nanos() as i64,
            )
        };

        if return_code != 0 {
            Err(ReturnCodes::from(return_code))
        } else {
            Ok(())
        }
    }

    /// Read data from the data reader, read or query condition without
    /// updating state
    ///
    /// Reads samples from the reader history cache without marking these
    /// samples as “read”. It starts with an arbitrary (matching) instance,
    /// reading (matching) samples from the oldest to the most recent, then
    /// continues with another arbitrarily selected (matching) instance, etc.
    /// This continues until it has traversed the entire history cache or has
    /// gathered maxs samples.
    ///
    /// The dds_read operation can be used to mark the returned samples as
    /// “read”; the dds_take operation can be used to also remove the returned
    /// samples from the history cache.
    ///
    /// For the plain dds_peek operation, all instances and samples match. This
    /// is different for the more selective variants, where the documentation
    /// refers to this function and only gives detailed information where it
    /// differs.
    pub fn peek(&self, max_samples: usize) -> Result<(), ReturnCodes> {
        todo!("not implemented")
    }

    /// Read data for a specific instance from the data reader, read or query
    /// condition without updating state
    ///
    /// See [DataReader::peek]. The matching criterion referred to there is
    /// that the instance handle must equal the handle parameter.
    pub fn peek_instance(
        &self,
        max_samples: usize,
        instance_handle: InstanceHandle,
    ) -> Result<(), ReturnCodes> {
        todo!("not implemented")
    }

    /// Read data for a specific instance matching sample/view/instance states
    /// from the data reader, read or query condition without updating state
    ///
    /// See [DataReader::peek]. The matching criterion referred to there is that:
    ///
    /// - the instance handle must equal the handle parameter; and
    /// - the sample/view/instance states must match the specification in the
    /// mask parameter.
    ///
    /// If the sample/view/instance state component in the mask is 0 and
    /// reader_or_condition references a data reader (as opposed to a read or
    /// query condition), it is treated as equivalent to any
    /// sample/view/instance state. If reader_or_condition references a read
    /// or query condition, the matching states are the union of mask and the
    /// condition’s mask.
    pub fn peek_instance_mask(&self) {
        todo!("not implemented")
    }

    /// Read the first unread sample without updating state
    pub fn peek_next(&self) {
        todo!("not implemented");


    }
    /// Read data from the data reader, read or query condition
    ///
    /// Reads samples from the reader history cache, marking these samples as
    /// “read”. It starts with an arbitrary (matching) instance, reading
    /// (matching) samples from the oldest to the most recent, then continues
    /// with another arbitrarily selected (matching) instance, etc. This
    /// continues until it has traversed the entire history cache or has
    /// gathered maxs samples.
    ///
    /// The dds_peek operation can be read samples without marking them as
    /// “read”; the dds_take operation can be used to also remove the returned
    /// samples from the history cache.
    ///
    /// For the plain dds_read operation, all instances and samples match.
    /// This is different for the more selective variants, where the
    /// documentation refers to this function and only gives detailed
    /// information where it differs.
    pub fn read(&self) -> Result<T, ReturnCodes>{
        let mut buffer = Vec::new();
        let mut sample_infos = Vec::new();
        let return_code = unsafe {
            cyclonedds_sys::dds_read(
                self.reader,
                buffer.as_mut_ptr(),
                sample_infos.as_mut_ptr(),
                2 << 16,
                1,
            )
        };

        if return_code < 0 {
            Err(ReturnCodes::from(return_code))
        } else {
            todo!("not implemented")
        }
        }
    

    /// Read data from the data reader, read or query condition.
    pub fn read_wl(&self) {
        todo!("not implemented")
    }

    /// Read data matching sample/view/instance states from the data reader,
    /// read or query condition
    ///
    /// See dds_read. The matching criterion referred to there is that the
    /// sample/view/instance states must match the specification in the mask
    /// parameter.
    ///
    /// If the sample/view/instance state component in the mask is 0 and
    /// reader_or_condition references a data reader (as opposed to a read or
    /// query condition), it is treated as equivalent to any
    /// sample/view/instance state. If reader_or_condition references a read or
    /// query condition, the matching states are the union of mask and the
    /// condition’s mask.
    pub fn read_mask(&self, mask: ReaderMask) {
        todo!("not implemented")
    }

    /// Read data matching sample/view/instance states from the data reader,
    /// read or query condition.
    pub fn read_mask_wl(&self) {
        todo!("not implemented")
    }

    /// Read data for a specific instance from the data reader, read or query
    /// condition
    pub fn read_instance(&self) {
        todo!("not implemented")
    }

    /// Read data for a specific instance from the data reader, read or query condition.
    pub fn read_instance_wl(&self) {
        todo!("not implemented")
    }

    /// Read data for a specific instance matching sample/view/instance states
    /// from the data reader, read or query condition
    ///
    /// See dds_read. The matching criterion referred to there is that:
    ///
    /// the instance handle must equal the handle parameter; and
    ///
    /// the sample/view/instance states must match the specification in the
    /// mask parameter.
    ///
    /// If the sample/view/instance state component in the mask is 0 and
    /// reader_or_condition references a data reader (as opposed to a read or
    /// query condition), it is treated as equivalent to any
    /// sample/view/instance state. If reader_or_condition references a read or
    /// query condition, the matching states are the union of mask and the
    /// condition’s mask.
    pub fn read_instance_mask(&self) {
        todo!("not implemented")
    }

    /// Read data for a specific instance matching sample/view/instance states
    /// from the data reader, read or query condition.
    pub fn read_instance_mask_wl(&self) {
        todo!("not implemented")
    }

    /// Read the first unread sample
    pub fn read_next(&self) {
        // Equivalent to dds_read_mask(reader, buf, si, 1, 1, DDS_NOT_READ_SAMPLE_STATE).
        todo!("not implemented")
    }

    /// Read the first unread sample.
    pub fn read_next_wl(&self) {}
    /// Take data from the data reader, read or query condition
    ///
    /// Reads and removes samples from the reader history cache. It starts with
    /// an arbitrary (matching) instance, reading (matching) samples from the
    /// oldest to the most recent, then continues with another arbitrarily
    /// selected (matching) instance, etc. This continues until it has
    /// traversed the entire history cache or has gathered maxs samples.
    ///
    /// The dds_read operation can be used to read samples without removing them
    /// from the history cache but marking them as “read”; the dds_peek
    /// operation can be used to read samples from the cache without changing
    /// any internal state.
    ///
    /// For the plain dds_take operation, all instances and samples match. This
    /// is different for the more selective variants, where the documentation
    /// refers to this function and only gives detailed information where it
    /// differs.
    pub fn take(&self) {
        todo!("not implemented")
    }

    /// Take data from the data reader, read or query condition.
    pub fn take_wl(&self) {
        todo!("not implemented")
    }

    /// Take data matching sample/view/instance states from the data reader,
    /// read or query condition
    ///
    /// See dds_take. The matching criterion referred to there is that the
    /// sample/view/instance states must match the specification in the mask
    /// parameter.
    ///
    /// If the sample/view/instance state component in the mask is 0 and
    /// reader_or_condition references a data reader (as opposed to a read or
    /// query condition), it is treated as equivalent to any
    /// sample/view/instance state. If reader_or_condition references a read or
    /// query condition, the matching states are the union of mask and the
    /// condition’s mask.
    pub fn take_mask(&self) {
        todo!("not implemented")
    }
    /// Take data matching sample/view/instance states from the data reader, read or query condition.
    pub fn take_mask_wl(&self) {
        todo!("not implemented")
    }

    /// Take data for a specific instance from the data reader, read or query condition
    pub fn take_instance(&self) {
        todo!("not implemented")
    }

    /// Take the first unread sample
    pub fn take_next(&self) {}
    /// Read samples while collecting result in an application-defined way
    /// without updating state
    ///
    /// When using a readcondition or querycondition, their masks are or’d with
    /// the given mask.
    ///
    /// If the sample/view/instance state component in the mask is 0 and there
    /// is no read or query condition, to combine it with, it is treated as
    /// equivalent to any sample/view/instance state.
    ///
    ///Collected samples are not marked as read.
    pub fn peek_with_collector(&self) {
        todo!("not implemented")
    }

    /// Read samples while collecting result in an application-defined way
    ///
    /// When using a readcondition or querycondition, their masks are or’d with
    /// the given mask.
    ///
    /// If the sample/view/instance state component in the mask is 0 and there
    /// is no read or query condition, to combine it with, it is treated as
    /// equivalent to any sample/view/instance state.
    ///
    /// Collected samples are marked as read.
    pub fn read__with_collector(&self) {
        todo!("not implemented")
    }

    /// Take samples while collecting result in an application-defined way
    ///
    /// When using a readcondition or querycondition, their masks are or’d with
    /// the given mask.
    ///
    /// If the sample/view/instance state component in the mask is 0 and there
    /// is no read or query condition, to combine it with, it is treated as
    /// equivalent to any sample/view/instance state.
    ///
    /// Collected samples are removed from the history cache.
    pub fn take_with_collector(&self) {
        todo!("not implemented")
    }

    /// Get references to a representation of the samples in a reader history
    /// cache and their accompanying sample infodata values (of same type)
    /// without updating state
    ///
    /// This operation returns references to the internal representation of
    /// samples (struct ddsi_serdata), which can then be used in a variety of
    /// ways. Examples are converting it to application representation and
    /// obtaining a copy or a reference of the serialized representation. If
    /// the underlying implementation (struct ddsi_sertype) is known to the
    /// application, other options may exist as well.
    ///
    /// The data is left in the reader history cache and the sample state and
    /// view state of the returned samples and their instances are not
    /// updated; [DataReader::readcdr] updates these states; dds_takecdr
    /// removes the data from the history cache.
    ///
    /// The returned references must eventually be released by calling
    /// ddsi_serdata_unref. There is no guarantee the type pointer survives
    /// beyond the existence of the reader from which the references were read.
    ///
    /// When using a readcondition or querycondition, their masks are or’d with
    /// the given mask.
    ///
    /// If the sample/view/instance state component in the mask is 0 and there
    /// is no read or query condition, to combine it with, it is treated as
    /// equivalent to any sample/view/instance state.
    ///
    /// Note that this is a simple wrapper around dds_peek_with_collector.
    pub fn peekcdr(&self) {
        todo!("not implemented")
    }

    /// Get references to a representation of the samples of a specific
    /// instance in a reader history cache and their accompanying sample
    /// infodata values (of same type) without updating state
    ///
    /// This operation returns references to the internal representation of
    /// samples (struct ddsi_serdata), which can then be used in a variety of
    /// ways. Examples are converting it to application representation and
    /// obtaining a copy or a reference of the serialized representation. If
    /// the underlying implementation (struct ddsi_sertype) is known to the
    /// application, other options may exist as well.
    ///
    /// The data is left in the reader history cache and the sample state and
    /// view state of the returned samples and their instances are not updated;
    /// dds_readcdr updates these states; dds_takecdr removes the data from the
    /// history cache.
    ///
    /// The returned references must eventually be released by calling
    /// ddsi_serdata_unref. There is no guarantee the type pointer survives
    /// beyond the existence of the reader from which the references were read.
    ///
    /// When using a readcondition or querycondition, their masks are or’d with
    /// the given mask.
    ///
    /// If the sample/view/instance state component in the mask is 0 and there
    /// is no read or query condition, to combine it with, it is treated as
    /// equivalent to any sample/view/instance state.
    ///
    /// Note that this is a simple wrapper around dds_peek_with_collector.
    pub fn peekcdr_instance(&self) {
        todo!("not implemented")
    }

    /// Get references to a representation of the samples in a reader history
    /// cache and their accompanying sample infodata values (of same type) and
    /// marking them as read
    ///
    /// This operation returns references to the internal representation of
    /// samples (struct ddsi_serdata), which can then be used in a variety of
    /// ways. Examples are converting it to application representation and
    /// obtaining a copy or a reference of the serialized representation. If
    /// the underlying implementation (struct ddsi_sertype) is known to the
    /// application, other options may exist as well.
    ///
    /// The data is left in the reader history cache and the sample state and
    /// view state of the returned samples and their instances are updated;
    /// dds_peekcdr returns the data without updating these states; dds_takecdr
    /// removes the data from the history cache.
    ///
    ///The returned references must eventually be released by calling
    /// ddsi_serdata_unref. There is no guarantee the type pointer survives
    /// beyond the existence of the reader from which the references were read.
    ///
    ///When using a readcondition or querycondition, their masks are or’d with
    /// the given mask.
    ///
    /// If the sample/view/instance state component in the mask is 0 and there
    /// is no read or query condition, to combine it with, it is treated as
    /// equivalent to any sample/view/instance state.
    ///
    ///Note that this is a simple wrapper around dds_read_with_collector.
    pub fn readcdr(&self) {
        todo!("not implemented")
    }

    /// Get references to a representation of the samples of a specific
    /// instance in a reader history cache and their accompanying sample
    /// infodata values (of same type) and marking them as read
    ///
    ///This operation returns references to the internal representation of
    /// samples (struct ddsi_serdata), which can then be used in a variety of
    /// ways. Examples are converting it to application representation and
    /// obtaining a copy or a reference of the serialized representation. If
    /// the underlying implementation (struct ddsi_sertype) is known to the
    /// application, other options may exist as well.
    ///
    ///The data is left in the reader history cache and the sample state and
    /// view state of the returned samples and their instances are updated;
    /// dds_peekcdr returns the data without updating these states; dds_takecdr
    /// removes the data from the history cache.
    ///
    ///The returned references must eventually be released by calling
    /// ddsi_serdata_unref. There is no guarantee the type pointer survives
    /// beyond the existence of the reader from which the references were read.
    ///
    /// When using a readcondition or querycondition, their masks are or’d with
    /// the given mask.
    ///
    /// If the sample/view/instance state component in the mask is 0 and there
    /// is no read or query condition, to combine it with, it is treated as
    /// equivalent to any sample/view/instance state.
    ///
    /// * Note that this is a simple wrapper around dds_read_with_collector.
    pub fn readcdr_instance(&self) {
        todo!("not implemented")
    }

    /// Get references to a representation of the samples in a reader history
    /// cache and their accompanying sample infodata values (of same type) and
    /// remove them from the cache
    ///
    /// This operation returns references to the internal representation of
    /// samples (struct ddsi_serdata), which can then be used in a variety of
    /// ways. Examples are converting it to application representation and
    /// obtaining a copy or a reference of the serialized representation. If
    /// the underlying implementation (struct ddsi_sertype) is known to the
    /// application, other options may exist as well.
    ///
    /// The data is removed from the reader history cache; dds_peekcdr leaves
    /// them in and leaves the sample and view states unchanged; dds_readcdr
    /// leaves the data in the cache but does update the sample and view states.
    ///
    ///The returned references must eventually be released by calling
    /// ddsi_serdata_unref. There is no guarantee the type pointer survives
    /// beyond the existence of the reader from which the references were read.
    ///
    /// When using a readcondition or querycondition, their masks are or’d with
    /// the given mask.
    ///
    /// If the sample/view/instance state component in the mask is 0 and there
    /// is no read or query condition, to combine it with, it is treated as
    /// equivalent to any sample/view/instance state.
    ///
    /// Note that this is a simple wrapper around dds_take_with_collector.
    pub fn takecdr(&self) {
        todo!("not implemented")
    }

    /// Get references to a representation of the samples of a specific
    /// instance in a reader history cache and their accompanying sample
    /// infodata values (of same type) and remove them from the cache
    ///
    /// This operation returns references to the internal representation of
    /// samples (struct ddsi_serdata), which can then be used in a variety of
    /// ways. Examples are converting it to application representation and
    /// obtaining a copy or a reference of the serialized representation. If
    /// the underlying implementation (struct ddsi_sertype) is known to the
    /// application, other options may exist as well.
    ///
    /// The data is removed from the reader history cache; dds_peekcdr leaves them in and leaves the sample and view states unchanged; dds_readcdr leaves the data in the cache but does update the sample and view states.
    ///
    /// The returned references must eventually be released by calling
    /// ddsi_serdata_unref. There is no guarantee the type pointer survives
    /// beyond the existence of the reader from which the references were read.
    ///
    /// When using a readcondition or querycondition, their masks are or’d with
    /// the given mask.
    ///
    /// If the sample/view/instance state component in the mask is 0 and there
    /// is no read or query condition, to combine it with, it is treated as
    /// equivalent to any sample/view/instance state.
    ///
    /// Note that this is a simple wrapper around dds_take_with_collector.
    pub fn takecdr_instance(&self) {
        todo!("not implemented")
    }
}

impl<T: TopicType> FetchableEntity for DataReader<T> {
    fn fetch(&self) -> cyclonedds_sys::dds_entity_t {
        self.reader
    }
}

pub struct AnyDataReader {
    reader: cyclonedds_sys::dds_entity_t,
}

impl FetchableEntity for AnyDataReader {
    fn fetch(&self) -> cyclonedds_sys::dds_entity_t {
        self.reader
    }
}

pub enum ReaderMask {
    SampleState(cyclonedds_sys::dds_sample_state),
    ViewState(cyclonedds_sys::dds_view_state),
    InstanceState(cyclonedds_sys::dds_instance_state),
}

/// Read state for a data value
pub enum SampleState {
    /// [DataReader] has already accessed the sample by read
    Read,
    /// [DataReader] has not accessed the sample before
    NotRead,
}
/// View state of an instance relative to the samples
pub enum ViewState {
    /// [DataReader] is accessing the sample for the first time when the instance
    /// is alive
    New,
    /// [DataReader] accessed the sample before
    Old,
}

/// Defines the state of the instance
pub enum InstanceState {
    /// Samples received for the instance from the live data writers
    Alive,
    /// Instance was explicitly disposed by the data writer
    NotAliveDisposed,
    /// Instance has been declared as not alive by data reader as there are no
    /// live data writers writing that instance
    NotAliveNoWriters,
}

/// Contains information about the associated data value
pub struct SampleInfo {
    sample_state: SampleState,
    view_state: ViewState,
    instance_state: InstanceState,
    /// Indicates whether there is a data associated with a sample
    /// * `true`, indicates the data is valid
    /// * `false`, indicates the data is invalid, no data to read
    valid_data: bool,
    /// timestamp of a data instance when it is written
    source_timestamp: SystemTime,
    /// handle to the data instance
    instance_handle: InstanceHandle,
    /// handle to the publisher
    publication_handle: cyclonedds_sys::dds_instance_handle_t,
    /// count of instance state change from NOT_ALIVE_DISPOSED to ALIVE
    disposed_generation_count: u32,
    /// count of instance state change from NOT_ALIVE_NO_WRITERS to ALIVE
    no_writers_generation_count: u32,
    /// indicates the number of samples of the same instance that follow the
    /// current one in the collection
    sample_rank: u32,
    /// difference in generations between the sample and most recent sample of
    /// the same instance that appears in the returned collection
    generation_rank: u32,
    /// difference in generations between the sample and most recent sample of
    /// the same instance when read/take was called
    absolute_generation_rank: u32,
}
