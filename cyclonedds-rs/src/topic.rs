use std::{ffi::CString, fmt::Debug, ptr::null};

use cyclonedds_sys::dds_topic_descriptor;
use serde::Serialize;

use crate::{
    core::{Entity, EntityParticipantError, Guid, ReturnCodes},
    domain::DomainParticipant,
    internal::InstanceHandle,
    InconsistentTopicStatus,
};

pub enum FindScope {
    Global,
    LocalDomain,
    Participant,
}

pub enum FilterMode {
    None,
    Sample,
    SampleArg,
    SampleinfoArg,
    SampleSampleinfoArg,
}

pub struct Topic<T: TopicType> {
    pub(super) topic: cyclonedds_sys::dds_entity_t,
    _marker: std::marker::PhantomData<T>,
}

impl<T: TopicType> Topic<T> {
    pub fn new(participant: &DomainParticipant) -> Result<Topic<T>, ReturnCodes> {
        todo!("not implemented")
    }
}

impl<T: TopicType> Drop for Topic<T> {
    fn drop(&mut self) {
        unsafe {
            cyclonedds_sys::dds_delete(self.topic);
        }
    }
}

impl<T: TopicType> TryFrom<AnyTopic> for Topic<T> {
    type Error = ReturnCodes;

    fn try_from(value: AnyTopic) -> Result<Self, Self::Error> {
        todo!()
    }
}

pub struct AnyTopic {
    pub(crate) topic: cyclonedds_sys::dds_entity_t,
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
    /// This operation gets the status value corresponding to
    /// INCONSISTENT_TOPIC and reset the status. The value can be obtained,
    /// only if the status is enabled for an entity. NULL value for status is
    /// allowed and it will reset the trigger value when status is enabled.
    pub fn inconsistent_topic_status(&self) -> Result<InconsistentTopicStatus, ReturnCodes> {
        todo!("not implemented")
    }

    /// Sets a filter and filter argument on a topic.
    ///
    /// Not thread-safe with respect to data being read/written using
    /// readers/writers using this topic. Be sure to create a topic entity
    /// specific to the reader you want to filter, then set the filter
    /// function, and only then create the reader. And don’t change it unless
    /// you know there are no concurrent writes.
    pub fn set_topic_filter_and_arg(
        &mut self,
        filter: cyclonedds_sys::dds_topic_filter_arg_fn,
    ) -> Result<(), ReturnCodes> {
        todo!()
    }

    pub fn write(&self, data: &impl Serialize) -> Result<(), ReturnCodes> {
        todo!()
    }

    pub(crate) fn new(arg: &mut DomainParticipant) -> Result<Self, ReturnCodes> {
        todo!()
    }
}

impl Entity for AnyTopic {
    fn instance_handle(&self) -> Result<InstanceHandle, ReturnCodes> {
        let mut handle = 0;
        let ret = unsafe { cyclonedds_sys::dds_get_instance_handle(self.topic, &mut handle) };
        if ret >= 0 {
            Ok(InstanceHandle { handle })
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

    fn participant(&self) -> Result<DomainParticipant, EntityParticipantError> {
        let parent_entity = unsafe { cyclonedds_sys::dds_get_parent(self.topic) };
        if parent_entity < 0 {
            todo!("Error handling")
        } else {
            // Assuming Participant can be constructed with an entity ID
            Ok(DomainParticipant {
                participant: parent_entity,
            })
        }
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
        let return_code;
        unsafe {
            return_code = cyclonedds_sys::dds_triggered(self.topic);
        }

        if return_code != 0 {
            Err(ReturnCodes::from(return_code))
        } else {
            Ok(())
        }
    }

    fn get_topic(&self) -> Result<impl Entity, ReturnCodes> {
        Ok(Self { topic: self.topic })
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

impl<T: TopicType> From<Topic<T>> for AnyTopic {
    fn from(topic: Topic<T>) -> Self {
        todo!("Need to implement the cyclone dds copy function")
    }
}

pub struct TopicListener {
    listener: cyclonedds_sys::dds_entity_t,
}

pub trait TopicSupport {}

pub enum TopicIdKind {
    Minimal,
    Complete,
}

pub mod detail {}

pub mod qos {
    /// This struct provides the basic mechanism for an application to specify
    /// Quality of Service attributes for a Topic.
    ///
    /// A QosPolicy can be set when the Topic is created or modified with the
    /// set qos operation. Both operations take the TopicQos object as a
    /// parameter. There may be cases where several policies are in conflict.
    /// Consistency checking is performed each time the policies are modified
    /// when they are being created and, in case they are already enabled, via
    /// the set qos operation.
    pub struct TopicQos {}
}

pub struct Filter {}

pub trait MessageType {
    fn name() -> String;
    fn type_name() -> String;
}

pub trait XType: TopicType {
    type Extends;

    fn extends() -> Self::Extends;
    fn extends_name() -> String;
}

pub struct ParticipantBuiltinTopicData {}

pub struct TopicBuiltinTopicData {}

pub struct PublicationBuiltinTopicData {}

pub struct SubscriptionBuiltinTopicData {}

pub struct TopicDescription {}

/// Trait for Topic types
/// All messages must be of [TopicType] to be used by a data reader or writer.
pub trait TopicType: Clone + Debug + PartialEq {
    fn name() -> &'static str {
        std::any::type_name::<Self>().rsplit("::").next().unwrap()
    }

    /// By default this is the module path of the type `T` excluding
    /// the crate name.
    fn type_name() -> &'static str {
        // Get the fully qualified name and remove the crate name.
        let full = std::any::type_name::<Self>();
        if let Some((_, rest)) = full.split_once("::") {
            rest
        } else {
            full
        }
    }
}

pub struct TopicGuid {
    guid: cyclonedds_sys::dds_guid_t,
}

impl Guid for TopicGuid {
    fn guid(&self) -> [u8; 16] {
        self.guid.v
    }
}

/// Simple sized byte container to hold serialized type info Holds XTypes 
/// information (TypeInformation, TypeMapping) for a type.
pub struct MetaSer {
    data: String,
}
/// Type identifier kind for getting endpoint type identifier.
pub enum TypeidKind {
    /// XTypes Minimal Type ID
    Minimal,
    /// XTypes Complete Type ID
    Complete
}

pub struct BuiltinTopic {}

pub struct BuiltinTopicEndpoint {}

pub struct BuiltinTopicGuid {}


