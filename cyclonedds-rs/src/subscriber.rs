use std::{
    marker::PhantomData,
    ptr::{null, null_mut},
};

use serde::Serialize;

use crate::{core::ReturnCodes, domain::DomainParticipant, topic::{self, Topic, TopicType}};



pub struct Subscriber {
    subscriber: cyclonedds_sys::dds_entity_t,
}

impl Subscriber {
    pub(crate) fn new(participant: &DomainParticipant) -> Subscriber {
        Subscriber {
            subscriber: unsafe {
                cyclonedds_sys::dds_create_subscriber(participant.participant, null_mut(), null())
            },
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

    pub fn create_reader<T: TopicType>(&self, topic: &Topic<T>) -> Result<DataReader<T>, ReturnCodes> {
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

impl From<DomainParticipant> for Subscriber {
    fn from(mut participant: DomainParticipant) -> Subscriber {
        participant.subscriber()
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

pub struct AnyDataReader {
    reader: cyclonedds_sys::dds_entity_t,
    _marker: PhantomData<*mut ()>,
}
