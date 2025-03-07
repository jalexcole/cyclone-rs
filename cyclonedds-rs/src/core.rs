use thiserror::Error;

use crate::{domain::DomainParticipant, internal::InstanceHandle};

#[derive(Debug, Error)]
pub enum ReturnCodes {
    #[error("Success")]
    Ok = 0,
    #[error("Non specific error")]
    Error,
    #[error("Feature Unsupported")]
    Unsupported,
    #[error("Bad parameter value")]
    BadParameter,
    #[error("Precondition for operation not met")]
    PreconditionNotMet,
    #[error("operation fails because of a lack of resources")]
    OutOfResources,
    #[error("When a configurable feature is not enabled")]
    NotEnabled,
    #[error("When an attempt is made to modify an immutable policy")]
    ImmutablePolicy,
    #[error("When an attempt is made to modify an inconsistent policy")]
    InconsistentPolicy,
    #[error("When an attempt is made to delete something more than once")]
    AlreadyDeleted,
    #[error("When a timeout has occurred")]
    Timeout,
    #[error("When expected data is not provided")]
    NoData,
    #[error("When a function is called when it should not be")]
    IllegalOperation,
    #[error("When credentials are not enough to use the function")]
    NotAllowedBySecurity,
    #[error("")]
    XRetCodeBase,
    #[error("")]
    XRetCode,
    #[error("")]
    InProgress,
    #[error("")]
    TryAgain,
    #[error("")]
    Interrupted,
    #[error("")]
    NotAllowed,
    #[error("")]
    HostNotFound,
    #[error("")]
    NoNetwork,
    #[error("")]
    NoConnection,
    #[error("")]
    NotEnoughSpace,
    #[error("")]
    OutOfRange,
    #[error("")]
    ResultTooLarge,
}

impl From<i32> for ReturnCodes {
    fn from(value: i32) -> Self {
        match value.abs() {
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
            16 => ReturnCodes::TryAgain,
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


pub(crate) trait FetchableEntity {
    fn fetch(&self) -> cyclonedds_sys::dds_entity_t;
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
    use std::{
        ffi::{c_int, c_void, CString},
        time::Duration,
    };

    pub struct Qos {
        pub(crate) qos: *mut cyclonedds_sys::dds_qos_t,
    }

    impl Qos {
        /// Allocate memory and initialize default QoS-policies.
        pub fn new() -> Qos {
            Qos {
                qos: unsafe { cyclonedds_sys::dds_create_qos() },
            }
        }
        /// Reset a QoS-policies structure to default values.
        pub fn reset(&mut self) {
            unsafe {
                cyclonedds_sys::dds_reset_qos(self.qos);
            }
        }
        /// Copy all QoS-policies from one structure to another, unless already
        /// set.
        ///
        /// Policies are copied from src to dst, unless src already has the
        /// policy set to a non-default value.
        /// * `other` - Pointer to the source qos
        pub fn merge(&mut self, other: &Qos) {
            unsafe {
                cyclonedds_sys::dds_merge_qos(self.qos, other.qos);
            }
        }
        /// Set the userdata of a [Qos] structure.
        /// * `value` - Pointer to the userdata
        pub fn qset_userdata(&mut self, value: &str) {
            unsafe {
                cyclonedds_sys::dds_qset_userdata(
                    self.qos,
                    value.as_bytes().as_ptr() as *const c_void,
                    value.len(),
                );
            }
        }
        /// Set the topicdata of a qos structure.
        /// @param value - Pointer to the topicdata
        /// @param sz - Size of the topicdata stored in value
        pub fn qset_topicdata(&mut self, value: &str) {
            unsafe {
                cyclonedds_sys::dds_qset_topicdata(
                    self.qos,
                    value.as_bytes().as_ptr() as *const c_void,
                    value.len(),
                );
            }
        }
        /// Set the groupdata of a qos structure.
        pub fn qset_groupdata(&mut self, value: &str) {
            unsafe {
                cyclonedds_sys::dds_qset_groupdata(
                    self.qos,
                    value.as_bytes().as_ptr() as *const c_void,
                    value.len(),
                );
            }
        }
        /// Set the durability policy of a qos structure.
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
        /// Set the resource limits policy of a qos structure.
        /// * `max_samples` - Number of samples resource-limit value
        /// * `max_instances` - Number of instances resource-limit value
        /// * `max_samples_per_instance` - Number of samples per read resource-limit value
        pub fn qset_resource_limits(
            &mut self,
            max_samples: i32,
            max_instances: i32,
            max_samples_per_instance: i32,
        ) {
            unsafe {
                cyclonedds_sys::dds_qset_resource_limits(
                    self.qos,
                    max_samples,
                    max_instances,
                    max_samples_per_instance,
                );
            }
        }

        /// Set the presentation policy of a qos structure.
        /// * `access_scope` - Access-scope kind
        /// * `coherent_access` - Coherent access enable value
        /// * `ordered_access` - Ordered access enable value
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
        /// Set the lifespan policy of a [Qos] structure.
        /// * `duration` - Lifespan duration
        pub fn qset_lifespan(&mut self, duration: Duration) {
            unsafe {
                cyclonedds_sys::dds_qset_lifespan(self.qos, duration.as_nanos() as i64);
            }
        }
        /// Set the deadline policy of a [Qos] structure.
        /// * `duration` - Deadline duration
        pub fn qset_deadline(&mut self, duration: Duration) {
            unsafe {
                cyclonedds_sys::dds_qset_deadline(self.qos, duration.as_nanos() as i64);
            }
        }
        /// Set the latency budget policy of a [Qos] structure.
        /// * `duration` - Latency budget duration
        pub fn dds_qset_latency_budget(&mut self, duration: Duration) {
            unsafe {
                cyclonedds_sys::dds_qset_latency_budget(self.qos, duration.as_nanos() as i64);
            }
        }
        /// Set the ownership policy of a [Qos] structure.
        /// * `kind` - Ownership kind
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
        /// Set the liveliness policy of a [Qos] structure.
        /// * `kind` - Liveliness kind
        /// * `lease_duration` - Liveliness lease duration
        pub fn set_liveliness(
            &mut self,
            kind: cyclonedds_sys::dds_liveliness_kind,
            lease_duration: Duration,
        ) {
            unsafe {
                cyclonedds_sys::dds_qset_liveliness(
                    self.qos,
                    kind,
                    lease_duration.as_nanos() as i64,
                );
            }
        }

        /// Set the time-based filter policy of a [Qos] structure.
        /// * `minimum_separation` - Time-based filter minimum separation
        pub fn set_time_based_filter(&mut self, minimum_separation: Duration) {
            unsafe {
                cyclonedds_sys::dds_qset_time_based_filter(
                    self.qos,
                    minimum_separation.as_nanos() as i64,
                );
            }
        }
        /// Set the partition policy of a [Qos] structure.
        /// * `ps` - Array of partition names
        pub fn set_partition(&mut self, ps: &[&str]) {
            // Convert the Rust string slices into CStrings.
            let c_strings: Vec<CString> = ps.iter().map(|s| CString::new(*s).unwrap()).collect();
            // Collect pointers to the C strings.
            let mut c_ptrs: Vec<*const std::ffi::c_char> =
                c_strings.iter().map(|cs| cs.as_ptr()).collect();
            // Use the number of partitions from the slice length.
            let n = c_ptrs.len() as u32;
            unsafe {
                cyclonedds_sys::dds_qset_partition(self.qos, n, c_ptrs.as_mut_ptr());
            }
        }
        /// Set the partition policy of a [Qos] structure.
        /// * `name` -  Pointer to string(s) storing partition name(s)
        pub fn set_partition1(&mut self, name: &str) {
            unsafe {
                cyclonedds_sys::dds_qset_partition1(self.qos, CString::new(name).unwrap().as_ptr());
            }
        }
        /// Set the reliability policy of a [Qos] structure.
        /// * `kind` - Reliability kind
        /// * `max_blocking_time` - Reliability max blocking time
        pub fn set_reliability(
            &mut self,
            kind: cyclonedds_sys::dds_reliability_kind,
            max_blocking_time: Duration,
        ) {
            unsafe {
                cyclonedds_sys::dds_qset_reliability(
                    self.qos,
                    kind,
                    max_blocking_time.as_nanos() as i64,
                );
            }
        }
        /// Set the transport-priority policy of a qos structure
        /// * `value` - Transport priority
        pub fn set_transport_priority(&mut self, value: i32) {
            unsafe {
                cyclonedds_sys::dds_qset_transport_priority(self.qos, value as c_int);
            }
        }
        /// Set the destination-order policy of a qos structure
        /// * `kind` - Destination-order kind
        pub fn set_destination_order(&mut self, kind: cyclonedds_sys::dds_destination_order_kind) {
            unsafe {
                cyclonedds_sys::dds_qset_destination_order(self.qos, kind);
            }
        }
        /// Set the writer data lifecycle policy of a qos structure
        /// * `autodispose` - Writer data lifecycle autodispose
        pub fn set_writer_data_lifecycle(&mut self, autodispose: bool) {
            unsafe {
                cyclonedds_sys::dds_qset_writer_data_lifecycle(self.qos, autodispose);
            }
        }
        /// Set the reader data lifecycle policy of a qos structure
        /// * `autopurge_nowriter_samples_delay` - Delay for purging of samples from instances in a
        /// no-writers state
        /// * `autopurge_disposed_samples_delay` - Delay for purging of samples from disposed instances
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
        /// Set the writer batching policy of a qos structure.
        ///
        /// When batching is disabled, each write/dispose/unregister operation
        /// results in its own RTPS message that is sent out onto the
        /// transport. For small data types, this means most messages
        /// (and hence network packets) are small. As a consequence the fixed
        /// cost of processing a message (or packet) increases load.
        ///
        /// Enabling write batching causes the samples to be aggregated into a
        /// single larger RTPS message. This improves efficiency by spreading
        /// the fixed cost out over more samples. Naturally this increases
        /// latency a bit.
        ///
        /// The batching mechanism may or may not send out packets on a
        /// write/&c. operation. It buffers only a limited amount and will send
        /// out what has been buffered when a new write/&c. can not be added.
        /// To guarantee that the buffered data is sent, one must call “dds_flush”.
        ///
        /// * `batch_updates` - Whether writes should be batched
        pub fn set_writer_batching(&mut self, batch_updates: bool) {
            unsafe {
                cyclonedds_sys::dds_qset_writer_batching(self.qos, batch_updates);
            }
        }
        /// Set the durability service policy of a qos structure
        /// 
        /// * `service_cleanup_delay` - Service cleanup delay for purging of 
        ///     abandoned instances from the durability service
        /// * `history_kind` - History policy kind applied by the durability 
        ///     service.
        /// * `history_depth` - History policy depth applied by the durability 
        ///     service.
        /// * `max_samples` - Number of samples resource-limit policy applied 
        ///     by the durability service.
        /// * `max_instances` - Number of instances resource-limit policy 
        ///     applied by the durability service.
        /// * `max_samples_per_read` - Number of samples per instance 
        ///     resource-limit policy applied by the durability service
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

        /// Set the ignore local policy of a qos structure.
        /// 
        /// * `ignore` - Ignore local policy
        pub fn set_ignorelocal(&mut self, ignore: cyclonedds_sys::dds_ignorelocal_kind) {
            unsafe {
                cyclonedds_sys::dds_qset_ignorelocal(self.qos, ignore);
            }
        }
        /// Stores a property with the provided name and string value in a qos 
        /// structure.
        ///
        /// In the case a property with the provided name already exists in the 
        /// qos structure, the value for this entry is overwritten with the 
        /// provided string value. If more than one property with the provided 
        /// name exists, only the value of the first of these properties is updated.
        /// 
        /// * `name` - Pointer to name of the property
        /// * `value` - Pointer to string value to be stored in the property
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
                cyclonedds_sys::dds_qset_entity_name(
                    self.qos,
                    CString::new(name).unwrap().as_ptr(),
                );
            }
        }

        pub fn set_psmx_instances(&mut self, instances: usize, values: &[&str]) {
            unsafe {
                let c_strings: Vec<CString> =
                    values.iter().map(|&s| CString::new(s).unwrap()).collect();
                let c_ptrs: Vec<*const i8> = c_strings.iter().map(|s: &CString| s.as_ptr()).collect();
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
                let qos_output = std::ptr::null_mut();
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

pub trait Listener {}

pub trait Guid {

    fn guid(&self) -> [u8; 16];

}
