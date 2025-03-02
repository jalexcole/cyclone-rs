use std::{
    marker::PhantomData,
    ptr::{null, null_mut},
};

use crate::{
    core::ReturnCodes,
    domain::DomainParticipant,
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
    /// subscriber that currently have new data available. Any
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

    pub fn read(&self) {
        todo!("not implemented")
    }
}

pub struct AnyDataReader {
    reader: cyclonedds_sys::dds_entity_t,
    _marker: PhantomData<*mut ()>,
}
