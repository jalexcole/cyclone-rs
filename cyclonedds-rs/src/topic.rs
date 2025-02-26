use crate::{domain::DomainParticipant, internal::InstanceHandle, Entity, EntityParticipantError, InconsistentTopicStatus, Participant, ReturnCodes};

pub enum FindScope {
    Global,
    LocalDomain,
    Participant,
}

pub struct Topic<T> {
    pub(super) topic: cyclonedds_sys::dds_entity_t,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Topic<T> {}

impl<T> Drop for Topic<T> {
    fn drop(&mut self) {
        unsafe {
            cyclonedds_sys::dds_delete(self.topic);
        }
    }
}

pub struct AnyTopic {
    topic: cyclonedds_sys::dds_entity_t,
}

impl AnyTopic {
    pub fn name(&self) -> Result<String, ReturnCodes> {
        todo!()
    }

    pub fn type_name(&self) -> Result<String, ReturnCodes> {
        todo!()
    }

    /// Get INCONSISTENT_TOPIC status.
    ///
    /// This operation gets the status value corresponding to INCONSISTENT_TOPIC and reset the status. The value can be obtained, only if the status is enabled for an entity. NULL value for status is allowed and it will reset the trigger value when status is enabled.
    pub fn inconsistent_topic_status(&self) -> Result<InconsistentTopicStatus, ReturnCodes> {
        todo!("not implemented")
    }
}

impl Entity for AnyTopic {
    fn instance_handle(&self) -> Result<InstanceHandle, ReturnCodes> {
        let mut handle = 0;
        let ret = unsafe { cyclonedds_sys::dds_get_instance_handle(self.topic, &mut handle) };
        if ret >= 0 {
            Ok(InstanceHandle{handle})
        } else {
            Err(ReturnCodes::from(ret))
        }
    }

    fn guid(&self) -> Result<cyclonedds_sys::dds_guid_t, ReturnCodes> {
        let mut guid = cyclonedds_sys::dds_guid_t { v: [0; 16] };
        let ret = unsafe { cyclonedds_sys::dds_get_guid(self.topic, &mut guid) };
        if ret >= 0 {
            Ok(guid)
        } else {
            Err(ReturnCodes::from(ret))
        }
    }

    fn parent(&self) -> Result<impl Entity, crate::ParentError> {
        let parent = unsafe { cyclonedds_sys::dds_get_parent(self.topic) };
    }

    fn participant(&self) -> Result<DomainParticipant, crate::EntityParticipantError> {
         let parent_entity = unsafe { cyclonedds_sys::dds_get_parent(self.topic) };
        if parent_entity < 0 {
            todo!("Error handling")
        } else {
            // Assuming Participant can be constructed with an entity ID
            Ok(DomainParticipant { participant: parent_entity })
        }
    }

    fn children(&self) -> Result<Vec<impl Entity>, ReturnCodes> {
        return Err(ReturnCodes::Ok)
    }

    fn domain_id(&self) -> Result<u32, ReturnCodes> {
        let mut domain_id = 0;
        let result = unsafe { cyclonedds_sys::dds_get_domainid(self.topic, &mut domain_id) };
        if result >= 0 {
            Ok(domain_id as u32)
        } else {
            Err(ReturnCodes::from(result))
        }
    }

    fn triggered(&self) -> Result<(), ReturnCodes> {
        todo!()
    }

    fn get_topic(&self) -> Result<impl Entity, ReturnCodes> {
        todo!()
    }

    fn assert_liveliness(&self) -> Result<(), ReturnCodes> {
        let ret = unsafe { cyclonedds_sys::dds_assert_liveliness(self.topic) };
        if ret >= 0 {
            Ok(())
        } else {
            Err(ReturnCodes::from(ret))
        }
    }
}

impl Drop for AnyTopic {
    fn drop(&mut self) {
        unsafe {
            cyclonedds_sys::dds_delete(self.topic);
        }
    }
}

pub trait TopicSupport {}

pub enum TopicIdKind {
    Minimal,
    Complete,
}

pub mod detail {}

pub mod qos {
    pub struct TopicQos {}
}


pub struct Filter {}

