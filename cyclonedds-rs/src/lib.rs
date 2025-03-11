use std::ffi::{c_char, c_int};

use cyclonedds_sys::*;
use publisher::PublicationMatchedStatus;

pub(crate) mod internal;

pub mod core;
pub mod domain;
pub mod dynamic;
pub mod psmx;
pub mod publisher;
pub mod qos;
pub mod statistics;
pub mod subscriber;
pub mod topic;
pub mod xtypes;
pub mod util;
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


/// Presentation QoS: Applies to Publisher, Subscriber.
#[derive(Debug,Clone, Copy, PartialEq)]
pub enum PresentationAccessScopeKind {
    Instance,
    Topic,
}
/// Ignore-local QoS: Applies to DataReader, DataWriter.
#[derive(Debug,Clone, Copy, PartialEq)]
pub enum IgnorelocalKind {
    None,
    Participant,
    Process,
}
/// Type-consistency QoS: Applies to DataReader, DataWriter.
#[derive(Debug,Clone, Copy, PartialEq)]
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


extern "C" {
    fn dds_set_log_sink(sink: extern "C" fn(level: c_int, message: *const c_char));
}