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

pub enum XReturnCode {}

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



pub trait Listener {}

pub trait Guid {
    fn guid(&self) -> [u8; 16];
}


#[cfg(test)]
mod tests {

}