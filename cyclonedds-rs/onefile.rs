// Project: cyclonedds-rs (v0.1.0)

// ./examples/HelloWorldPublisher.rs
use cyclonedds_rs::domain::DomainParticipant;

fn main() {
    let mut participant: DomainParticipant = DomainParticipant::new(0).unwrap();
    let mut publisher = participant.publisher().unwrap();
    println!("=== [Publisher]  Waiting for a reader to be discovered ...\n");
    let mut writer = publisher.create_datawriter().unwrap();
    let mut msg = HelloWorldData::Msg {
        userID: 1,
        message: "Hello World".to_string(),
    };

    println!("=== [Publisher]  Writing : ");

    writer.write(&msg).unwrap();

    drop(participant)
   
}

pub mod HelloWorldData {
    use cyclonedds_rs::topic::TopicType;
    use serde::Serialize;

    /// ```idl
    /// module HelloWorldData {
    ///   struct Msg {
    ///     @key
    ///    long userID;
    ///     string message;
    ///   };
    /// };
    /// ```
    #[derive(Serialize, Debug, Clone)]
    pub struct Msg {
        pub userID: i64,
        pub message: String,
    }

    impl TopicType for Msg {
    }
}

// ./examples/hello_world/HelloWorldSubscriber.rs




fn main() {
    
}
// ./src/core.rs
use thiserror::Error;

use crate::{domain::DomainParticipant, internal::InstanceHandle};



#[derive(Debug)]
pub enum ReturnCodes {
    Ok,
    Error,
    Unsupported,
    BadParameter,
    PreconditionNotMet,
    OutOfResources,
    NotEnabled,
    ImmutablePolicy,
    AlreadyDeleted,
    Timeout,
    NoData,
    IllegalOperation,
    NotAllowedBySecurity,
    XRetCodeBase,
    XRetCode,
    InProgress,
    TryAgaian,
    Interrupted,
    NotAllowed,
    HostNotFound,
    NoNetwork,
    NoConnection,
    NotEnoughSpace,
    OutOfRange,
    ResultTooLarge,
}

impl From<i32> for ReturnCodes {
    fn from(value: i32) -> Self {
        match value {
            0 => ReturnCodes::Ok,
            1 => ReturnCodes::Error,
            2 => ReturnCodes::Unsupported,
            3 => ReturnCodes::BadParameter,
            4 => ReturnCodes::PreconditionNotMet,
            5 => ReturnCodes::OutOfResources,
            6 => ReturnCodes::NotEnabled,
            7 => ReturnCodes::ImmutablePolicy,
            8 => ReturnCodes::AlreadyDeleted,
            9 => ReturnCodes::Timeout,
            10 => ReturnCodes::NoData,
            11 => ReturnCodes::IllegalOperation,
            12 => ReturnCodes::NotAllowedBySecurity,
            13 => ReturnCodes::XRetCodeBase,
            14 => ReturnCodes::XRetCode,
            15 => ReturnCodes::InProgress,
            16 => ReturnCodes::TryAgaian,
            17 => ReturnCodes::Interrupted,
            18 => ReturnCodes::NotAllowed,
            19 => ReturnCodes::HostNotFound,
            20 => ReturnCodes::NoNetwork,
            21 => ReturnCodes::NoConnection,
            22 => ReturnCodes::NotEnoughSpace,
            23 => ReturnCodes::OutOfRange,
            24 => ReturnCodes::ResultTooLarge,
            _ => panic!("Unknown return code: {}", value),
        }
    }
}

pub trait Entity: Drop {
    /// Returns the instance handle that represents the entity.
    fn instance_handle(&self) -> Result<InstanceHandle, ReturnCodes>;
    /// Returns the GUID that represents the entity in the network, and therefore only supports participants, readers and writers.
    fn guid(&self) -> Result<cyclonedds_sys::dds_guid_t, ReturnCodes>;
    /// Get entity parent.
    ///
    /// This operation returns the parent to which the given entity belongs. For instance, it will return the Participant that was used when creating a Publisher (when that Publisher was provided here).
    ///
    /// When a reader or a writer are created with a participant, then a subscriber or publisher are created implicitly. This function will return the implicit parent and not the used participant.
    // fn parent(&self) -> Result<impl Entity, ParentError>;
    
    /// Get entity participant.
    ///
    /// This operation returns the participant to which the given entity belongs. For instance, it will return the Participant that was used when creating a Publisher that was used to create a DataWriter (when that DataWriter was provided here).
    ///
    /// `DOC_TODO`: Link to generic dds entity relations documentation.
    fn participant(&self) -> Result<DomainParticipant, EntityParticipantError>;
    /// Get entity children.
    ///
    /// This operation returns the children that the entity contains. For instance, it will return all the Topics, Publishers and Subscribers of the Participant that was used to create those entities (when that Participant is provided here).
    ///
    /// This functions takes a pre-allocated list to put the children in and will return the number of found children. It is possible that the given size of the list is not the same as the number of found children. If less children are found, then the last few entries in the list are untouched. When more children are found, then only ‘size’ number of entries are inserted into the list, but still complete count of the found children is returned. Which children are returned in the latter case is undefined.
    ///
    /// When supplying NULL as list and 0 as size, you can use this to acquire the number of children without having to pre-allocate a list.
    ///
    /// When a reader or a writer are created with a participant, then a subscriber or publisher are created implicitly. When used on the participant, this function will return the implicit subscriber and/or publisher and not the related reader/writer.
    // fn children(&self) -> Result<Vec<impl Entity>, ReturnCodes>;

    /// Get the domain id to which this entity is attached.
    ///
    /// When creating a participant entity, it is attached to a certain domain.
    /// All the children (like Publishers) and childrens’ children
    /// (like DataReaders), etc are also attached to that domain.
    ///
    /// This function will return the original domain ID when called on any of
    /// the entities within that hierarchy. For entities not associated with a
    /// domain, the id is set to DDS_DOMAIN_DEFAULT.
    fn domain_id(&self) -> Result<u32, ReturnCodes>;
    /// Checks whether the entity has one of its enabled statuses triggered.
    fn triggered(&self) -> Result<(), ReturnCodes>;
    /// Get the topic
    ///
    /// This operation returns a topic (handle) when the function call is done
    /// with reader, writer, read condition or query condition. For instance,
    /// it will return the topic when it is used for creating the reader or
    /// writer. For the conditions, it returns the topic that is used for
    /// creating the reader which was used to create the condition.
    fn get_topic(&self) -> Result<impl Entity, ReturnCodes>;
    /// This operation manually asserts the liveliness of a writer or domain
    /// participant.
    ///
    /// This operation manually asserts the liveliness of a writer or domain
    /// participant. This is used in combination with the Liveliness QoS policy
    /// to indicate that the entity remains active. This operation need only be
    /// used if the liveliness kind in the QoS is either
    /// DDS_LIVELINESS_MANUAL_BY_PARTICIPANT or DDS_LIVELINESS_MANUAL_BY_TOPIC.
    fn assert_liveliness(&self) -> Result<(), ReturnCodes>;
}

#[derive(Error, Debug)]
pub enum ParentError {
    #[error("Called with a participant")]
    NIL,
    #[error("An internal error has occurred")]
    InternalError,
    #[error("The operation is invoked on an inappropriate object")]
    IllegalOperation,
    #[error("The entity has already be deleted. DOC_TODO: Link to generic dds entity relations documentation")]
    AlreadyDeleted,
}
#[derive(Error, Debug)]
pub enum EntityParticipantError {
    #[error("An internal error has occurred")]
    InternalError,
    #[error("The operation is invoked on an inappropriate object")]
    IllegalOperation,
    #[error("The entity has already be deleted. DOC_TODO: Link to generic dds entity relations documentation")]
    AlreadyDeleted,
}

pub mod qos {
    use std::{ffi::{c_void, CString}, time::Duration};

    pub struct Qos {
    qos: *mut cyclonedds_sys::dds_qos_t,
}

impl Qos {
    pub fn new() -> Qos {
        Qos {
            qos: unsafe { cyclonedds_sys::dds_create_qos() },
        }
    }

    pub fn reset(&mut self) {
        unsafe {
            cyclonedds_sys::dds_reset_qos(self.qos);
        }
    }

    pub fn merge(&mut self, other: &Qos) {
        unsafe {
            cyclonedds_sys::dds_merge_qos(self.qos, other.qos);
        }
    }
    /// Set the userdata of a [Qos] structure.
    /// * `value` - Pointer to the userdata
    /// * `sz` - Size of the userdata stored in value
    pub fn qset_userdata(&mut self, value: *const c_void, sz: usize) {
        unsafe {
            cyclonedds_sys::dds_qset_userdata(self.qos, value, sz);
        }
    }
    /// Set the topicdata of a qos structure.
    /// @param value - Pointer to the topicdata
    /// @param sz - Size of the topicdata stored in value
    pub fn qset_topicdata(&mut self, value: *const c_void, sz: usize) {
        unsafe {
            cyclonedds_sys::dds_qset_topicdata(self.qos, value, sz);
        }
    }

    pub fn qset_groupdata(&mut self, value: *const c_void, sz: usize) {
        unsafe {
            cyclonedds_sys::dds_qset_groupdata(self.qos, value, sz);
        }
    }

    pub fn qset_durability(&mut self, kind: cyclonedds_sys::dds_durability_kind) {
        unsafe {
            cyclonedds_sys::dds_qset_durability(self.qos, kind);
        }
    }
    /// Set the history policy of a qos structure.
    ///
    /// Note that depth is only relevant for keep last. If you want limited history for keep all, use [Qos::qset_resource_limits()].
    /// * `kind` - History kind value
    /// * `depth` - History depth value
    pub fn qset_history(&mut self, kind: cyclonedds_sys::dds_history_kind, depth: i32) {
        unsafe {
            cyclonedds_sys::dds_qset_history(self.qos, kind, depth);
        }
    }

    pub fn qset_resource_limits(
        &mut self,
        max_instances: i32,
        max_samples: i32,
        max_samples_per_read: i32,
    ) {
        unsafe {
            cyclonedds_sys::dds_qset_resource_limits(
                self.qos,
                max_instances,
                max_samples,
                max_samples_per_read,
            );
        }
    }

    pub fn qset_presentation(
        &mut self,
        access_scope: cyclonedds_sys::dds_presentation_access_scope_kind,
        coherent_access: bool,
        ordered_access: bool,
    ) {
        unsafe {
            cyclonedds_sys::dds_qset_presentation(
                self.qos,
                access_scope,
                coherent_access,
                ordered_access,
            );
        }
    }

    pub fn qset_lifespan(&mut self, duration: Duration) {
        unsafe {
            cyclonedds_sys::dds_qset_lifespan(self.qos, duration.as_nanos() as i64);
        }
    }
    /// Set the deadline policy of a [Qos] structure.
    pub fn qset_deadline(&mut self, duration: Duration) {
        unsafe {
            cyclonedds_sys::dds_qset_deadline(self.qos, duration.as_nanos() as i64);
        }
    }

    pub fn dds_qset_latency_budget(&mut self, duration: Duration) {
        unsafe {
            cyclonedds_sys::dds_qset_latency_budget(self.qos, duration.as_nanos() as i64);
        }
    }

    pub fn qset_ownership(&mut self, kind: cyclonedds_sys::dds_ownership_kind) {
        unsafe {
            cyclonedds_sys::dds_qset_ownership(self.qos, kind);
        }
    }
    /// Set the ownership strength of a [Qos] structure.
    /// * `value` - Ownership strength
    pub fn qset_ownership_strength(&mut self, value: i32) {
        unsafe {
            cyclonedds_sys::dds_qset_ownership_strength(self.qos, value);
        }
    }

    pub fn set_liveliness(&mut self, kind: cyclonedds_sys::dds_liveliness_kind, lease_duration: Duration) {
        unsafe {
            cyclonedds_sys::dds_qset_liveliness(self.qos, kind, lease_duration.as_nanos() as i64);
        }
    }

    pub fn set_time_based_filter(&mut self, minimum_separation: Duration) {
        unsafe {
            cyclonedds_sys::dds_qset_time_based_filter(
                self.qos,
                minimum_separation.as_nanos() as i64,
            );
        }
    }

    pub fn set_partition(&mut self, n: usize, ps: &mut [&str]) {
        unsafe {
            // cyclonedds_sys::dds_qset_partition(self.qos, n as u32, ps.as_ptr());
        }
    }

    pub fn set_partition1(&mut self, name: &str) {
        unsafe {
            cyclonedds_sys::dds_qset_partition1(self.qos, CString::new(name).unwrap().as_ptr());
        }
    }

    pub fn set_reliability(&mut self, kind: cyclonedds_sys::dds_reliability_kind, max_blocking_time: Duration) {
        unsafe {
            cyclonedds_sys::dds_qset_reliability(
                self.qos,
                kind,
                max_blocking_time.as_nanos() as i64,
            );
        }
    }

    pub fn set_transport_priority(&mut self, value: i32) {
        unsafe {
            cyclonedds_sys::dds_qset_transport_priority(self.qos, value);
        }
    }

    pub fn set_destination_order(&mut self, kind: cyclonedds_sys::dds_destination_order_kind) {
        unsafe {
            cyclonedds_sys::dds_qset_destination_order(self.qos, kind);
        }
    }

    pub fn set_writer_data_lifecycle(&mut self, autodispose: bool) {
        unsafe {
            cyclonedds_sys::dds_qset_writer_data_lifecycle(self.qos, autodispose);
        }
    }

    pub fn set_reader_data_lifecycle(
        &mut self,
        autopurge_nowriter_samples_delay: Duration,
        autopurge_disposed_samples_delay: Duration,
    ) {
        unsafe {
            cyclonedds_sys::dds_qset_reader_data_lifecycle(
                self.qos,
                autopurge_nowriter_samples_delay.as_nanos() as i64,
                autopurge_disposed_samples_delay.as_nanos() as i64,
            );
        }
    }

    pub fn set_writer_batching(&mut self, batch_updates: bool) {
        unsafe {
            cyclonedds_sys::dds_qset_writer_batching(self.qos, batch_updates);
        }
    }

    pub fn set_durability_service(
        &mut self,
        service_cleanup_delay: Duration,
        history_kind: cyclonedds_sys::dds_history_kind,
        history_depth: i32,
        max_samples: i32,
        max_instances: i32,
        max_samples_per_read: i32,
    ) {
        unsafe {
            cyclonedds_sys::dds_qset_durability_service(
                self.qos,
                service_cleanup_delay.as_nanos() as i64,
                history_kind,
                history_depth,
                max_samples,
                max_instances,
                max_samples_per_read,
            );
        }
    }

    pub fn set_ignorelocal(&mut self, ignore: cyclonedds_sys::dds_ignorelocal_kind) {
        unsafe {
            cyclonedds_sys::dds_qset_ignorelocal(self.qos, ignore);
        }
    }

    pub fn set_prop(&mut self, name: &str, value: &str) {
        unsafe {
            cyclonedds_sys::dds_qset_prop(
                self.qos,
                CString::new(name).unwrap().as_ptr(),
                CString::new(value).unwrap().as_ptr(),
            );
        }
    }

    pub fn unset_prop(&mut self, name: &str) {
        unsafe {
            cyclonedds_sys::dds_qunset_prop(self.qos, CString::new(name).unwrap().as_ptr());
        }
    }

    pub unsafe fn set_bprop(&mut self, name: &str, value: *const c_void, sz: usize) {
        unsafe {
            cyclonedds_sys::dds_qset_bprop(
                self.qos,
                CString::new(name).unwrap().as_ptr(),
                value,
                sz,
            );
        }
    }

    pub fn unset_bprop(&mut self, name: &str) {
        unsafe {
            cyclonedds_sys::dds_qunset_bprop(self.qos, CString::new(name).unwrap().as_ptr());
        }
    }

    pub fn set_type_consistency(
        &mut self,
        kind: cyclonedds_sys::dds_type_consistency_kind,
        ignore_sequence_bounds: bool,
        ignore_string_bounds: bool,
        ignore_member_names: bool,
        prevent_type_widening: bool,
        force_type_validation: bool,
    ) {
        unsafe {
            cyclonedds_sys::dds_qset_type_consistency(
                self.qos,
                kind,
                ignore_sequence_bounds,
                ignore_string_bounds,
                ignore_member_names,
                prevent_type_widening,
                force_type_validation,
            );
        }
    }

    pub fn set_entity_name(&mut self, name: &str) {
        unsafe {
            cyclonedds_sys::dds_qset_entity_name(self.qos, CString::new(name).unwrap().as_ptr());
        }
    }

    pub fn set_psmx_instances(&mut self, instances: usize, values: &[&str]) {
        unsafe {
            let c_strings: Vec<CString> =
                values.iter().map(|&s| CString::new(s).unwrap()).collect();
            let c_ptrs: Vec<*const i8> = c_strings.iter().map(|s| s.as_ptr()).collect();
            cyclonedds_sys::dds_qset_psmx_instances(
                self.qos,
                instances as u32,
                c_ptrs.as_ptr() as *mut *const i8,
            );
        }
    }

    pub fn get_userdata(&mut self) -> Result<Vec<c_void>, &'static str> {
        todo!()
    }
}

impl Clone for Qos {
    fn clone(&self) -> Qos {
        unsafe {
            let mut qos_output = std::ptr::null_mut();
            cyclonedds_sys::dds_copy_qos(self.qos, qos_output);

            Qos { qos: qos_output }
        }
    }
}

impl Drop for Qos {
    fn drop(&mut self) {
        unsafe {
            cyclonedds_sys::dds_delete_qos(self.qos);
        }
    }
}

impl PartialEq for Qos {
    fn eq(&self, other: &Qos) -> bool {
        unsafe { cyclonedds_sys::dds_qos_equal(self.qos, other.qos) }
    }
}
}
// ./src/domain.rs



// safe.rs
use core::panic;
use std::ffi::c_uint;
use std::os::raw::c_int;
use std::ptr;

use crate::{core::ReturnCodes, publisher::Publisher, subscriber::Subscriber, topic::{AnyTopic, Topic, TopicType}};

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

    #[test]
    fn test_participant() {
        let participant = super::DomainParticipant::new(0).unwrap();
        drop(participant);
    }
}
// ./src/dynamic.rs
//! The Dynamic Type API to construct and manipulate data types.

use cyclonedds_sys::{dds_dynamic_type, dds_dynamic_type_t};

use crate::core::{Entity, ReturnCodes};

/// Enumeration with the type kind values that can be used to create a dynamic type.
pub enum DynamicTypeKind {
    None,
    Boolean,
    Byte,
    Int16,
    Int32,
    Int64,
    Uint16,
    Uint32,
    Uint64,
    Float32,
    Float64,
    Int8,
    UINT8,
    CHAR8,
    String8,
    Enumeration,
    Bitmask,
    Alias,
    Array,
    Sequence,
    Map,
    Structure,
    Union,
    Bitset,
}

pub enum TypeSpecKind {
    KindUpset,
    KindDefinition,
    KindPrimitive,
}

pub enum TypeExtensibility {
    Final,
    Appendable,
    Mutable,
}

pub enum TypeAutoid {
    Sequential,
    Hash,
}

/// Representation of a dynamically created type. This struct has an opaque pointer to the type in the type system. During construction of the type (setting properties and adding members), the internal type has the state ‘CONSTRUCTION’. Once the type is registered, the state is updated to ‘RESOLVED’ and the type cannot be modified.
///
/// The ‘ret’ member of this struct holds the return code of operations performed on this type. In case this value is not DDS_RETCODE_OK, the type cannot be used for further processing (e.g. adding members, registering the type, etc.).
pub struct DynamicType {
    dynamic_type: *mut dds_dynamic_type_t,
}

impl DynamicType {
    pub(crate) fn create(
        entity: & impl Entity,
        descripter: cyclonedds_sys::dds_dynamic_type_descriptor_t,
    ) -> Result<DynamicType, ReturnCodes> {
        todo!("not implemented")
    }
    /// Set the extensibility of a Dynamic Type.
    /// -
    pub fn set_extensibility(
        &mut self,
        extensibility: &TypeExtensibility,
    ) -> Result<(), ReturnCodes> {
        todo!("not implemented")
    }

    pub fn set_bit_bound(&mut self, bit_bound: u16) -> Result<(), ReturnCodes> {
        todo!("not implemented")
    }

    pub fn set_nested(&mut self, nested: bool) -> Result<(), ReturnCodes> {
        todo!("not implemented")
    }

    pub fn set_autoid(&mut self, autoid: &TypeAutoid) -> Result<(), ReturnCodes> {
        todo!("not implemented")
    }

    pub fn add_member(
        &mut self,
        member_descripter: cyclonedds_sys::dds_dynamic_member_descriptor_t,
    ) -> Result<(), ReturnCodes> {
        todo!("not implemented")
    }

    pub fn add_bitmask_field(&mut self, name: &str, position: u16) -> Result<(), ReturnCodes> {
        todo!("not implemented")
    }

    pub fn member_set_key(&mut self, member_id: u32, is_key: bool) -> Result<(), ReturnCodes> {
        todo!("not implemented")
    }

    pub fn member_set_optional(
        &mut self,
        member_id: u32,
        is_optional: bool,
    ) -> Result<(), ReturnCodes> {
        todo!("not implemented")
    }

    pub fn member_set_external(
        &mut self,
        member_id: u32,
        is_external: bool,
    ) -> Result<(), ReturnCodes> {
        todo!("not implemented")
    }

    pub fn member_set_hashid(
        &mut self,
        member_id: u32,
        hash_member_name: &str,
    ) -> Result<(), ReturnCodes> {
        todo!("not implemented")
    }

    pub fn member_set_must_understand(
        &mut self,
        member_id: u32,
        is_must_understand: bool,
    ) -> Result<(), ReturnCodes> {
        todo!("not implemented")
    }
}


struct DynamicTypeSpec;

struct DynamicTypeDescriptor;

struct DynamicMemberDescriptor;
// ./src/internal.rs
pub struct InstanceHandle {
    pub(super) handle: cyclonedds_sys::dds_instance_handle_t,
}



pub enum EntityKind {
    DontCare,
    Topic,
    Particpant,
    Reader,
    Writer,
    Subscriber,
    Publisher,
    CondRead,
    CondQuarry,
    CondGuard,
    Waitset,
    Domain,
    CycloneDds,
}

impl EntityKind {
    pub(crate) fn from_c(entity_kind: cyclonedds_sys::dds_entity_kind) -> EntityKind {
        match entity_kind {
            0 => EntityKind::DontCare,
            1 => EntityKind::Topic,
            2 => EntityKind::Particpant,
            3 => EntityKind::Reader,
            4 => EntityKind::Writer,
            5 => EntityKind::Subscriber,
            6 => EntityKind::Publisher,
            7 => EntityKind::CondRead,
            8 => EntityKind::CondQuarry,
            9 => EntityKind::CondGuard,
            10 => EntityKind::Waitset,
            11 => EntityKind::Domain,
            12 => EntityKind::CycloneDds,
            _ => panic!("Unknown entity kind"),
        }
    }
}

// ./src/lib.rs
use cyclonedds_sys::*;
use publisher::PublicationMatchedStatus;

pub(crate) mod internal;

pub mod core;
pub mod domain;
pub mod dynamic;
pub mod psmx;
pub mod publisher;
pub mod statistics;
pub mod subscriber;
pub mod topic;

pub mod prelude {
    use crate::domain::DomainParticipant;
    use crate::publisher::DataWriter;
    use crate::publisher::Publisher;
    use crate::subscriber::DataReader;
    use crate::subscriber::Subscriber;
    use crate::topic::Topic;

    pub use crate::core::ReturnCodes;
}

pub struct LivelinessLostStatus {
    status: dds_liveliness_lost_status_t,
}

pub struct OfferedDeadlineMissedStatus {
    status: dds_offered_deadline_missed_status_t,
}

pub struct OfferedIncompatibleQosStatus {
    status: dds_offered_incompatible_qos_status_t,
}

pub struct SubscriptionMatchedStatus {
    status: dds_subscription_matched_status_t,
}

pub struct LivelinessChangedStatus {
    status: dds_liveliness_changed_status_t,
}

pub struct SampleRejectedStatus {
    status: dds_sample_rejected_status_t,
}

pub struct SampleLostStatus {
    status: dds_sample_lost_status_t,
}
pub struct RequestDeadlineMissedStatus {
    status: dds_requested_deadline_missed_status_t,
}

pub struct RequestIncompatibleQosStatus {
    status: dds_requested_incompatible_qos_status_t,
}

struct Children;
struct DomainId;
struct Triggered;

pub struct InconsistentTopicStatus {
    status: dds_inconsistent_topic_status_t,
}

trait Liveliness {}

/// Durability QoS: Applies to Topic, DataReader, DataWriter.
pub enum DurabilityKind {
    /// Volatile durability
    Volatile,
    /// Transient Local durability
    TransientLocal,
    /// Transient durability
    Transient,
    /// Persistent durability
    Persistent,
}

/// History QoS: Applies to Topic, DataReader, DataWriter.
pub enum HistoryKind {
    /// Keep Last history
    KeepLast,
    /// Keep All history
    KeepAll,
}

/// Ownership QoS: Applies to Topic, DataReader, DataWriter.
pub enum OwnershipKind {
    /// Shared Ownership
    Shared,
    /// Exclusive Ownership
    Exclusive,
}
/// Liveliness QoS: Applies to Topic, DataReader, DataWriter.
pub enum LivelinessKind {
    Automatic,
    ManualByParticipant,
    ManualByTopic,
}
/// Reliability QoS: Applies to Topic, DataReader, DataWriter.
pub enum ReliabilityKind {
    BestEffort,
    Reliable,
}
/// DestinationOrder QoS: Applies to Topic, DataReader, DataWriter.
pub enum DestinationOrderKind {
    ByReceptionTimestamp,
    BySourceTimestamp,
}
/// Presentation QoS: Applies to Publisher, Subscriber.
pub enum PresentationAccessScopeKind {
    Instance,
    Topic,
}
/// Ignore-local QoS: Applies to DataReader, DataWriter.
pub enum IgnorelocalKind {
    None,
    Participant,
    Process,
}
/// Type-consistency QoS: Applies to DataReader, DataWriter.
pub enum ConsistencyKind {
    DisallowTypeCoercion,
    AllowTypeCoercion,
}

pub enum Statuses {
    InconsistentTopicStatus(InconsistentTopicStatus),
    OfferedDeadlineMissedStatus(OfferedDeadlineMissedStatus),
    OfferedIncompatibleQosStatus(OfferedIncompatibleQosStatus),
    LivelinessLostStatus(LivelinessLostStatus),
    PublicationMatchedStatus(PublicationMatchedStatus),
    RequestDeadlineMissedStatus,
    RequestIncompatibleQosStatus,
    SampleRejectedStatus,
    LivelinessChangedStatus,
    SubscriptionMatchedStatus,
    SampleLostStatus,
}

// ./src/psmx.rs
//! # Publish Subscribe Message Exchange
//! ## Overview
//! 
//! The Publish Subscribe Message Exchange (PSMX) interface provides support for off-loading data communication from the network stack of Cyclone to arbitrary pub-sub transport implementations.[^1] This section provides an overview of the structure and introduces some terminology, details on specific operations are provided at the definitions of those operations.
//!
//! A Cyclone DDS Domain consists of a plurality of DDS Domain Entities, which are the representations of the DDS Domain in a specific process.
//!
//! A PSMX Plugin provides an implementation of the PSMX interface, allowing the instantiation of a PSMX Instance to establish a connection between a DDS Domain Entity and a PSMX Domain. The PSMX Plugin is specified as a library and a PSMX Instance Name. The library is loaded in the process and a constructor function provided by the library is invoked to create and initialize a PSMX Instance given the PSMX Instance Name (and a configuration string). In principle a specific library may be configured multiple times in a single DDS Domain.
//!
//! The PSMX Instance Name is assumed to uniquely identify the PSMX Domain in the DDS Domain. From the PSMX Instance Name, a numeric PSMX Instance ID is derived that uniquely identifies the PSMX Domain within the DDS Domain Entity and is assumed to uniquely identify the PSMX Domain in the DDS Domain.[^2]
//!
//! Each PSMX Instance chooses a 16-byte PSMX Locator[^3] such that any pair of instances with the same PSMX Locator communicate, and any pair with different locators do not communicate.[^4]
//!
//! DDS Topics, DDS Readers and DDS Writers are mapped to corresponding objects in PSMX Instances. For DDS Readers and DDS Writers, the application can restrict the set of PSMX Instances for which the mapping is created using the “PSMX Instances” QoS setting, and the PSMX Instances can refuse mapping based on type and QoS information.
//!
//! DDS Topic Entities are representations of the topics in the DDS Domain, such that two identical definitions of a topic in a DDS Domain Entity give rise to two application-level DDS Topic Entities, but only to a single topic in the DDS Domain Entity and thus also only one PSMX Topic object per PSMX Instance.
//!
//! Each DDS Reader/Writer is mapped to a set of PSMX Reader/Writer Endpoints, one for each PSMX Instance in the “PSMX Instances” QoS that accepts the type and reader/writer QoS. An associated set of PSMX Domains consisting of the PSMX Domains for which PSMX Reader/Writer Endpoints have been created is assumed to exist.
//!
//! The PSMX Domain is assumed to deliver data published by the PSMX Writer associated with DDS Writer X to all PSMX Readers associated with the DDS Readers Ys that match X[^5], optionally excluding DDS Readers in the same Domain Entity as X. It is assumed to not deliver data to other DDS Readers in the DDS Domain. It is assumed to do this with a quality of service compatible with the DDS QoS.
//!
//! Readers associated with DDS Readers in the same DDS Domain Entity.
//!
//! If the intersection of the sets of PSMX Domains of a DDS Reader and a DDS Writer in a DDS Domain:
//!
//! - is empty, off-loading data transfer to PSMX (for this pair) is not possible;
//!
//! - contains one instance, that PSMX Domain is eligible for off-loading data transfer;
//!
//! - contains multiple instances, the configuration is invalid.
//!
//! If an eligible PSMX Domain exists and the PSMX Locators for the corresponding two PSMX Instances are the same, then PSMX is used to transfer data.
//!
//! The PSMX objects are represented in the interface as pointers to “dds_psmx”, “dds_psmx_topic”, “dds_psmx_endpoint”. The PSMX Plugin is responsible for allocating and freeing these. It is expected that the PSMX Plugin internally uses an extended version of these types to store any additional data it needs. E.g., a hypothetical “weed” PSMX Plugin could do:
//!```c
//! struct psmx_weed {
//!   struct dds_psmx c;
//!   weed_root *x;
//! };
//! ```
//! The creator function mentioned above is required to be called NAME_create_psmx, where NAME is the value of the “name” attribute of the PubSubMessageExchange interface configuration element. It must have the following signature:
//!```c 
//! dds_return_t NAME_create_psmx (
//!   struct dds_psmx **psmx_instance,
//!   dds_psmx_instance_id_t identifier,
//!   const char *config)
//! ```
//! Where
//! *psmx_instance must be set point to a new PSMX Instance on success and may be left undefined on error identifier contains the numeric PSMX Instance ID config the PSMX configuration from the “config” attribute of the PubSubMessageExchange interface configuration element.
//!
//! The “config” argument is a contiguous sequence of characters terminated by the first double-\0. Each \0-terminated character sequence is a string that consists of KEY=VALUE pairs, where each K-V pair is terminated by a semicolon.
//!
//! If the configuration string as set in Cyclone DDS configuration contains a “INSTANCE_NAME” key, its value is used as the PSMX Instance Name. If the key is not included, the value of the “name” attribute of the corresponding PubSubMessageExchange element in configuration is used as the PSMX Instance Name. In all cases, looking up the “INSTANCE_NAME” key in the configuration string using dds_psmx_get_config_option_value will return the PSMX Instance Name as its value.
//!
//! The behaviour of the constructor function is dependent on the interface version it implements:
//!
//! - For version 0, it is responsible for setting:
//!
//!     - ops to the addresses of the various functions implementing the operations
//!
//!     - instance_name to a “dds_alloc” allocated string
//!
//!     - instance_id to the “identifier” argument
//!
//! and for zero-initializing the other fields. At some point after this initialization, and once it is prepared to handle the “get_node_id” operation, it must invoke the “dds_psmx_init_generic” to complete the initialization.
//!
//! - For version 1, it is responsible for setting:
//!
//!     - ops
//!
//! All other fields will be initialized by the Cyclone DDS after succesful return and the “get_node_id” operation also will be invoked after the constructor returned.
//!
//! Whether the plugin implements version 0 or version 1 of the interface is controlled by the function pointers in “dds_psmx_ops_t”. If “create_topic” and “deinit” are non-null, it is version 0; if both are null it is version 1. Neither “create_topic_type” nor “delete_psmx” is touched by Cyclone DDS if the interface is version 0, allowing for binary backwards compatibility.
//!
//! &#8212; Footnotes: &#8212;
//!
//! [^1]: In particular including shared-memory based mechanisms.
//!
//! [^2]: Internally, the name is not used for anything other than the generation of the numeric id.
//!
//! [^3]: Confusingly named “node identifier” in the interface, even though it has nothing to do with the numeric PSMX Domain identifier.
//!
//! [^4]: This typically matches a machine when the transport is shared memory.
//!
//! [^5]: That is, the matching rules between Readers and Writers defined in the DDS specification.
// ./src/publisher.rs
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

// ./src/statistics.rs


use cyclonedds_sys::dds_delete_statistics;

use crate::core::{Entity, ReturnCodes};




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
// ./src/subscriber.rs
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

// ./src/topic.rs
use std::fmt::Debug;

use serde::Serialize;

use crate::{
    core::{Entity, EntityParticipantError, ReturnCodes},
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

impl<T: TopicType> Topic<T> {}

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
        todo!()
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

