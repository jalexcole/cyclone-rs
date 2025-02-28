use cyclonedds_sys::*;
use domain::DomainParticipant;
use internal::InstanceHandle;
use publisher::Publisher;
use thiserror::Error;

pub(crate) mod internal;

pub mod core;
pub mod dynamic;
pub mod domain;
pub mod psmx;
pub mod statistics;
pub mod publisher;
pub mod subscriber;
pub mod topic;

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
    fn guid(&self) -> Result<dds_guid_t, ReturnCodes>;
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







pub struct PublicationMatchedStatus {
    status: dds_publication_matched_status_t,
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

    pub fn qset_durability(&mut self, kind: dds_durability_kind_t) {
        unsafe {
            cyclonedds_sys::dds_qset_durability(self.qos, kind);
        }
    }
    /// Set the history policy of a qos structure.
    ///
    /// Note that depth is only relevant for keep last. If you want limited history for keep all, use [Qos::qset_resource_limits()].
    /// * `kind` - History kind value
    /// * `depth` - History depth value
    pub fn qset_history(&mut self, kind: dds_history_kind_t, depth: i32) {
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
        access_scope: dds_presentation_access_scope_kind_t,
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

    pub fn qset_ownership(&mut self, kind: dds_ownership_kind_t) {
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

    pub fn set_liveliness(&mut self, kind: dds_liveliness_kind_t, lease_duration: Duration) {
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

    pub fn set_reliability(&mut self, kind: dds_reliability_kind_t, max_blocking_time: Duration) {
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

    pub fn set_destination_order(&mut self, kind: dds_destination_order_kind_t) {
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
        history_kind: dds_history_kind_t,
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

    pub fn set_ignorelocal(&mut self, ignore: dds_ignorelocal_kind_t) {
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
        kind: dds_type_consistency_kind_t,
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

struct InconsistentTopic;

struct PublicationMatched;
struct LivelinessLost;
struct OfferedDeadlineMissed;
struct OfferedIncompatibleQos;
struct SubscriptionMatched;
struct LivelinessChanged;
struct SampleRejected;
struct SampleLost;
struct RequestedDeadlineMissed;
struct RequestedIncompatibleQos;

use std::ffi::c_void;
use std::ptr::null_mut;
use std::time::Duration;
use std::ffi::CString;

use cyclonedds_sys::{dds_entity_t, dds_listener_t};

struct Domain {
    domain: *mut dds_entity_t,
}

impl Domain {
    pub fn new(domain_id: u32, config: &str) -> Domain {
        // FIXME: Domain {
        //     domain: unsafe { cyclonedds_sys::dds_create_domain(domain_id , config.to_string().to_bytes().as_ptr()) }
        // }
        todo!("not implemented")
    }

    pub fn lookup_participant(&self) -> Result<&[Participant], ReturnCodes> {
        todo!("not implemented")
    }

    pub fn create_participant(
        &mut self,
        domain_id: u32,
        config: &str,
    ) -> Result<Participant, ReturnCodes> {
        todo!("not implemented")
    }
}

impl Drop for Domain {
    fn drop(&mut self) {
        todo!("not implemented")
    }
}

pub struct Participant {
    participant: dds_entity_t,
}

impl Participant {
    pub fn new(domain_id: u32) -> Result<Participant, ReturnCodes> {
        match unsafe { cyclonedds_sys::dds_create_participant(domain_id, null_mut(), null_mut()) } {
            0 => Ok(Participant { participant: 0 }),
            _ => todo!("not implemented"),
        }
    }

    pub fn create_publisher(
        &mut self,
        qos: &mut Qos,
        listener: *mut dds_listener_t,
    ) -> Result<Publisher, ReturnCodes> {
        todo!("not implemented")
    }
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



