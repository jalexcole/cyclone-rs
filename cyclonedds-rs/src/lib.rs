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
pub mod xtypes;

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
