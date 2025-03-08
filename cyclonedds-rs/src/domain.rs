// safe.rs
use core::panic;
use std::num::NonZeroU32;

use std::ptr;
use std::{ffi::c_uint, fs::File};

use thiserror::Error;

use crate::core::FetchableEntity;
use crate::{
    core::{qos::Qos, Entity, EntityParticipantError, ReturnCodes},
    internal::InstanceHandle,
    publisher::Publisher,
    subscriber::Subscriber,
    topic::{AnyTopic, Topic, TopicType},
};

/// A safe wrapper around a Cyclone DDS Participant.
///
/// In Cyclone DDS, participants are represented by the
/// [cyclonedds_sys::dds_entity_t] type.
pub struct DomainParticipant {
    pub(super) participant: cyclonedds_sys::dds_entity_t,
}

impl DomainParticipant {
    /// Creates a new [DomainParticipant] for the given domain.
    ///
    /// Returns a [Result] with a safe [DomainParticipant] on success,
    /// or the negative error code on failure.
    ///
    /// * `domain_id` - The domain ID to create the participant. Typically
    ///     ranges from 0-232.
    pub fn new(domain_id: u32) -> Result<Self, DomainCreationError> {
        if domain_id > 232 {
            // This line emits a warning during the build.
            println!(
                "cargo:warning=Domain ID: ({}) exceeds typical range ({})",
                domain_id, 232
            );
        }

        unsafe {
            // Create a participant. The `dds_create_participant` function
            //returns the participant handle
            // or a negative error code if creation fails.
            let participant = cyclonedds_sys::dds_create_participant(
                domain_id as c_uint,
                ptr::null(),
                ptr::null(),
            );

            if participant < 0 {
                Err(DomainCreationError::from(-participant))
            } else {
                Ok(DomainParticipant { participant })
            }
        }
    }
    /// Creates a domain with a given configuration, specified as an
    /// initializer (unstable interface)
    ///
    /// To explicitly create a domain based on a configuration passed as a raw
    /// initializer rather than as an XML string. This allows bypassing the XML
    /// parsing, but tightly couples the initializing to implementation. See
    /// dds/ddsi/ddsi_config.h:ddsi_config_init_default for a way to initialize
    /// the default configuration. A domain created in this manner must be
    /// explicitly deleted by calling dds_delete on the domain
    /// (or on DDS_CYCLONEDDS_HANDLE).
    ///
    /// It will not be created if a domain with the given domain id already
    /// exists. This could have been created implicitly by a previous call to
    /// this function, dds_create_participant or dds_create_domain_with_rawconfig.
    ///
    /// Please be aware that the given domain_id always takes precedence over
    /// the configuration.
    ///
    /// * `domain` – The domain to be created. DDS_DEFAULT_DOMAIN is not
    ///     allowed.
    /// * `config` – [in] A configuration initializer. The lifetime of any
    ///     pointers in config must be at least that of the lifetime of the
    ///     domain.
    pub fn raw_config(
        domain_id: NonZeroU32,
        config: &DomainParticipantConfigParams,
    ) -> Result<DomainParticipant, DomainCreationError> {
        let domain = unsafe {
            cyclonedds_sys::dds_create_domain_with_rawconfig(
                domain_id.get() as c_uint,
                &config.config,
            )
        };

        if domain < 0 {
            Err(DomainCreationError::from(domain))
        } else {
            Ok(DomainParticipant {
                participant: domain,
            })
        }
    }

    /// Returns the [Subscriber] of this [`DomainParticipant`].
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn subscriber(&mut self) -> Result<Subscriber, ReturnCodes> {
        match Subscriber::new(self) {
            Ok(subscriber) => Ok(subscriber),
            Err(error) => Err(error),
        }
    }

    /// Returns the [Publisher] of this [`DomainParticipant`].
    ///
    /// # Errors
    ///
    /// This function will return an error if the publisher could not be created.
    pub fn publisher(&mut self) -> Result<Publisher, ReturnCodes> {
        Publisher::new(self)
    }

    pub fn topic<T: TopicType>(&mut self) -> Result<Topic<T>, ReturnCodes> {
        Topic::new(self)
    }

    /// Creates a new `AnyTopic` associated with this `DomainParticipant`.
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    /// - `Ok`: containing the newly created `AnyTopic`.
    /// - `Err`: if there is an error during the creation of the `AnyTopic`.
    pub fn any_topic(&mut self) -> Result<AnyTopic, ReturnCodes> {
        AnyTopic::new(self)
    }

    pub fn qos(&self) -> Result<Qos, ReturnCodes> {
        let qos = ptr::null_mut();
        let return_code = unsafe { cyclonedds_sys::dds_get_qos(self.participant, qos) };

        if return_code == 0 {
            Ok(Qos { qos })
        } else {
            Err(ReturnCodes::from(return_code))
        }
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

impl Default for DomainParticipant {
    fn default() -> Self {
        match DomainParticipant::new(0) {
            Ok(participant) => participant,
            Err(error) => panic!("Failed to create domain participant: {}", error),
        }
    }
}

impl Entity for DomainParticipant {
    fn instance_handle(&self) -> Result<InstanceHandle, ReturnCodes> {
        let mut instance_handle_ptr = 0;
        let return_code = unsafe {
            cyclonedds_sys::dds_get_instance_handle(self.participant, &mut instance_handle_ptr)
        };

        if return_code == 0 {
            Ok(InstanceHandle {
                handle: instance_handle_ptr,
            })
        } else {
            Err(ReturnCodes::from(return_code))
        }
    }

    /*************  ✨ Codeium Command ⭐  *************/
    /// Gets the GUID of the participant.
    ///
    /// The GUID is a unique identifier for the participant that can be used to
    /// identify it in the DDS network.
    /******  081eba80-5f7d-47ac-9db2-c397807c80c0  *******/
    fn guid(&self) -> Result<cyclonedds_sys::dds_guid_t, ReturnCodes> {
        let mut guid = cyclonedds_sys::dds_guid_t { v: [0; 16] };
        match unsafe { cyclonedds_sys::dds_get_guid(self.participant, &mut guid) } {
            0 => Ok(guid),
            rc => Err(ReturnCodes::from(rc)),
        }
    }

    fn participant(&self) -> Result<DomainParticipant, EntityParticipantError> {
        let participant = unsafe { cyclonedds_sys::dds_get_participant(self.participant) };

        if participant < 0 {
            todo!("Error handling")
        } else {
            Ok(DomainParticipant { participant })
        }
    }

    fn domain_id(&self) -> Result<u32, ReturnCodes> {
        let mut id = 0;
        match unsafe { cyclonedds_sys::dds_get_domainid(self.participant, &mut id) } {
            0 => Ok(id),
            _ => Err(ReturnCodes::from(-1)),
        }
    }

    fn triggered(&self) -> Result<(), ReturnCodes> {
        match unsafe { cyclonedds_sys::dds_triggered(self.participant) } {
            0 => Ok(()),
            err => Err(ReturnCodes::from(err)),
        }
    }

    fn get_topic(&self) -> Result<impl Entity, ReturnCodes> {
        Ok(AnyTopic {
            topic: unsafe { cyclonedds_sys::dds_get_topic(self.participant) },
        })
    }

    fn assert_liveliness(&self) -> Result<(), ReturnCodes> {
        match unsafe { cyclonedds_sys::dds_assert_liveliness(self.participant) } {
            0 => Ok(()),
            err => Err(ReturnCodes::from(err)),
        }
    }
}

impl FetchableEntity for DomainParticipant {
    fn fetch(&self) -> i32 {
        self.participant
    }
}

#[derive(Error, Debug)]
pub enum DomainCreationError {
    #[error("The domain to be created. DDS_DEFAULT_DOMAIN is not allowed.")]
    BadParameter,
    #[error("The domain already existed and cannot be created again.")]
    PreconditionNotMet,
    #[error("An internal error has occurred")]
    Error,
}

impl From<i32> for DomainCreationError {
    fn from(value: i32) -> Self {
        match value.abs() {
            cyclonedds_sys::DDS_RETCODE_BAD_PARAMETER => DomainCreationError::BadParameter,
            cyclonedds_sys::DDS_RETCODE_PRECONDITION_NOT_MET => {
                DomainCreationError::PreconditionNotMet
            }
            cyclonedds_sys::DDS_RETCODE_ERROR => DomainCreationError::Error,
            _ => panic!("Unknown error code {}", value),
        }
    }
}

pub struct DomainParticipandBuilder {}

pub trait DomainParticipantListener: Drop {}

pub mod qos {
    pub struct DomainParticipantQos {}
}

pub struct DomainParticipantConfigParams {
    config: cyclonedds_sys::ddsi_config,
}

impl TryFrom<File> for DomainParticipantConfigParams {
    type Error = ReturnCodes;

    fn try_from(value: File) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use serde::Serialize;

    use crate::{
        core::Entity,
        domain::DomainParticipant,
        internal::{KeyDescriptor, TopicDescriptor, TypeMetaSer},
        topic::TopicType,
    };

    #[derive(Default, Clone, Serialize, PartialEq, Debug)]
    struct HelloWorldData {
        id: i32,
        data: String,
    }

    impl TopicType for HelloWorldData {
        fn name() -> &'static str {
            std::any::type_name::<Self>().rsplit("::").next().unwrap()
        }

        fn type_name() -> &'static str {
            // Get the fully qualified name and remove the crate name.
            let full = std::any::type_name::<Self>();
            if let Some((_, rest)) = full.split_once("::") {
                rest
            } else {
                full
            }
        }

        fn topic_descriptor() -> crate::internal::TopicDescriptor {
            crate::internal::TopicDescriptor {
                m_size: 32,
                m_align: 0,
                m_flagset: 1 << 6,
                m_nkeys: 1,
                m_typename: Self::type_name().to_string(),
                m_keys: vec![KeyDescriptor {
                    name: "userID".to_string(),
                    m_offset: 5,
                    index: 0,
                }],
                m_nops: 3,
                m_ops: vec![],
                m_meta: "".to_string(),
                type_information: vec![16777216 | 1 << 0 | 1 << 3 | 196608 | 1 << 2],
                type_mapping: TypeMetaSer {
                    data: vec![
                        0x4c, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0xf1, 0x51, 0x01, 0x00,
                        0x6e, 0x42, 0x14, 0x9e, 0xb1, 0x41, 0xa0, 0xd7, 0x25, 0x94, 0xc6, 0x50,
                        0xd7, 0x3c, 0x00, 0x34, 0x00, 0x00, 0x00, 0xf1, 0x51, 0x01, 0x00, 0x01,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x24, 0x00, 0x00, 0x00, 0x02,
                        0x00, 0x00, 0x00, 0x0b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x31,
                        0x00, 0x04, 0x58, 0x5c, 0x95, 0x70, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x01,
                        0x00, 0x00, 0x00, 0x01, 0x00, 0x70, 0x00, 0x78, 0xe7, 0x31, 0x02, 0x7a,
                        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0xf2, 0xb1, 0xa3, 0xb1, 0xfb,
                        0x8b, 0x1a, 0x60, 0x51, 0x60, 0x14, 0x29, 0x7c, 0x3b, 0x8a, 0x00, 0x62,
                        0x00, 0x00, 0x00, 0xf2, 0x51, 0x01, 0x00, 0x1c, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x48, 0x65, 0x6c, 0x6c, 0x6f,
                        0x57, 0x6f, 0x72, 0x6c, 0x64, 0x44, 0x61, 0x74, 0x61, 0x3a, 0x3a, 0x4d,
                        0x73, 0x67, 0x00, 0x3a, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x15,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x31, 0x00, 0x04, 0x00, 0x07,
                        0x00, 0x00, 0x00, 0x75, 0x73, 0x65, 0x72, 0x49, 0x44, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x16, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
                        0x00, 0x70, 0x00, 0x08, 0x00, 0x00, 0x00, 0x6d, 0x65, 0x73, 0x73, 0x61,
                        0x67, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x22, 0x00, 0x00, 0x00, 0x01,
                        0x00, 0x00, 0x00, 0xf2, 0xb1, 0xa3, 0xb1, 0xfb, 0x8b, 0x1a, 0x60, 0x51,
                        0x60, 0x14, 0x29, 0x7c, 0x3b, 0x8a, 0xf1, 0x6e, 0x42, 0x14, 0x9e, 0xb1,
                        0x41, 0xa0, 0xd7, 0x25, 0x94, 0xc6, 0x50, 0xd7, 0x3c,
                    ],
                },
                restrict_data_representation: 0,
            }
        }
    }

    #[test]
    fn test_participant() {
        let participant = super::DomainParticipant::new(0).unwrap();
        drop(participant);
    }
    #[test]
    fn test_participant_subscriber() {
        let mut participant = DomainParticipant::default();

        let subscriber = participant
            .subscriber()
            .expect("Unable to create subscriber");
    }

    #[test]
    fn test_participant_publisher() {
        let mut participant = DomainParticipant::default();

        let publisher = participant.publisher().expect("Unable to create publisher");
    }

    #[test]
    fn test_participant_topic() {
        let mut participant = DomainParticipant::default();

        let topic = participant
            .topic::<HelloWorldData>()
            .expect("Unable to create topic");
    }

    #[test]
    fn test_participant_any_topic() {
        let mut participant = DomainParticipant::default();

        let topic = participant.any_topic().expect("Unable to create topic");
    }

    // Test that a DomainParticipant cleans up without error.
    #[test]
    fn test_participant_lifecycle() {
        {
            let participant = DomainParticipant::new(0).expect("Failed to create participant");

            participant
                .assert_liveliness()
                .expect("Failed to assert liveliness");
            // Optionally, do something with the participant here.
        }
        // After the scope ends, the participant is dropped.
        // Pause briefly to allow asynchronous cleanup if necessary.
        std::thread::sleep(Duration::from_millis(50));
        // If no panic occurs, the test passes.
    }
}
