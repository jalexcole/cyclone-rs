use core::slice;
use std::{
    ffi::{CStr, CString},
    io::Read,
    mem::{self, offset_of},
    process::{Command, Stdio},
    ptr,
};

use crate::topic::{MetaSer, TopicType};

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
        // match entity_kind {
        //     0 => EntityKind::DontCare,
        //     1 => EntityKind::Topic,
        //     2 => EntityKind::Particpant,
        //     3 => EntityKind::Reader,
        //     4 => EntityKind::Writer,
        //     5 => EntityKind::Subscriber,
        //     6 => EntityKind::Publisher,
        //     7 => EntityKind::CondRead,
        //     8 => EntityKind::CondQuarry,
        //     9 => EntityKind::CondGuard,
        //     10 => EntityKind::Waitset,
        //     11 => EntityKind::Domain,
        //     12 => EntityKind::CycloneDds,
        //     _ => panic!("Unknown entity kind"),
        // }

        match entity_kind {
            cyclonedds_sys::dds_entity_kind::DDS_KIND_DONTCARE => todo!(),
            cyclonedds_sys::dds_entity_kind::DDS_KIND_TOPIC => todo!(),
            cyclonedds_sys::dds_entity_kind::DDS_KIND_PARTICIPANT => todo!(),
            cyclonedds_sys::dds_entity_kind::DDS_KIND_READER => todo!(),
            cyclonedds_sys::dds_entity_kind::DDS_KIND_WRITER => todo!(),
            cyclonedds_sys::dds_entity_kind::DDS_KIND_SUBSCRIBER => todo!(),
            cyclonedds_sys::dds_entity_kind::DDS_KIND_PUBLISHER => todo!(),
            cyclonedds_sys::dds_entity_kind::DDS_KIND_COND_READ => todo!(),
            cyclonedds_sys::dds_entity_kind::DDS_KIND_COND_QUERY => todo!(),
            cyclonedds_sys::dds_entity_kind::DDS_KIND_COND_GUARD => todo!(),
            cyclonedds_sys::dds_entity_kind::DDS_KIND_WAITSET => todo!(),
            cyclonedds_sys::dds_entity_kind::DDS_KIND_DOMAIN => todo!(),
            cyclonedds_sys::dds_entity_kind::DDS_KIND_CYCLONEDDS => todo!(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct TopicDescriptor {
    /// Size of topic type
    pub(crate) m_size: u32,
    /// Alignment of topic type
    pub(crate) m_align: u32,
    /// Flags
    pub(crate) m_flagset: u32,
    /// Number of keys (can be 0)
    pub(crate) m_nkeys: u32,
    /// Type name
    pub(crate) m_typename: String,
    /// Key descriptors (NULL iff m_nkeys 0)
    pub(crate) m_keys: Vec<KeyDescriptor>,
    /// Number of operations in m_ops
    pub(crate) m_nops: u32,
    /// Marshalling meta data
    pub(crate) m_ops: Vec<u32>,
    /// XML topic description meta data
    pub(crate) m_meta: String,
    /// XCDR2 serialized TypeInformation, only present if flag
    /// [cyclonedds_sys::DDS_TOPIC_XTYPES_METADATA] is set
    pub(crate) type_information: Vec<u32>,
    /// XCDR2 serialized TypeMapping: maps type-id to type object and minimal
    /// to complete type id, only present if flag
    /// [cyclonedds_sys::DDS_TOPIC_XTYPES_METADATA] is set
    pub(crate) type_mapping: TypeMetaSer,
    /// restrictions on the data representations allowed for the top-level type
    /// for this topic, only present if flag
    /// [cyclonedds_sys::DDS_TOPIC_RESTRICT_DATA_REPRESENTATION]
    pub(crate) restrict_data_representation: u32,
}

impl From<cyclonedds_sys::dds_topic_descriptor_t> for TopicDescriptor {
    fn from(topic_descriptor: cyclonedds_sys::dds_topic_descriptor_t) -> Self {
        Self {
            m_size: topic_descriptor.m_size,
            m_align: topic_descriptor.m_align,
            m_flagset: topic_descriptor.m_flagset,
            m_nkeys: topic_descriptor.m_nkeys,
            m_typename: unsafe {
                CStr::from_ptr(topic_descriptor.m_typename)
                    .to_str()
                    .unwrap()
                    .to_string()
            },
            m_keys: unsafe {
                slice::from_raw_parts(topic_descriptor.m_keys, topic_descriptor.m_nkeys as usize)
                    .to_vec()
                    .iter()
                    .map(|k| KeyDescriptor::from(*k))
                    .collect()
            },
            m_nops: topic_descriptor.m_nops,
            m_ops: unsafe {
                slice::from_raw_parts(topic_descriptor.m_ops, topic_descriptor.m_nops as usize)
                    .to_vec()
            },
            m_meta: unsafe {
                CStr::from_ptr(topic_descriptor.m_typename)
                    .to_str()
                    .unwrap()
                    .to_string()
            },
            type_information: todo!(
                "topic_descriptor.type_information has not been implemented yet"
            ),
            type_mapping: topic_descriptor.type_mapping.into(),
            restrict_data_representation: topic_descriptor.restrict_data_representation,
        }
    }
}

impl From<TopicDescriptor> for cyclonedds_sys::dds_topic_descriptor_t {
    fn from(topic_descriptor: TopicDescriptor) -> Self {
        cyclonedds_sys::dds_topic_descriptor_t {
            m_size: topic_descriptor.m_size,
            m_align: topic_descriptor.m_align,
            m_flagset: topic_descriptor.m_flagset,
            m_nkeys: topic_descriptor.m_nkeys,
            m_typename: CString::new(topic_descriptor.m_typename)
                .expect("Unable to convert typename to c string")
                .as_c_str()
                .as_ptr(),
            m_keys: topic_descriptor
                .m_keys
                .iter()
                .map(|k| k.clone().into())
                .collect::<Vec<cyclonedds_sys::dds_key_descriptor_t>>()
                .as_slice()
                .as_ptr(),
            m_nops: topic_descriptor.m_nops,
            m_ops: topic_descriptor.m_ops.as_slice().as_ptr(),
            m_meta: CString::new(topic_descriptor.m_meta)
                .expect("Unable to convert typename to c string")
                .as_c_str()
                .as_ptr(),
            type_information: cyclonedds_sys::dds_type_meta_ser {
                data: topic_descriptor.type_information.as_ptr() as *const u8,
                sz: topic_descriptor.type_information.len() as u32,
            },
            type_mapping: topic_descriptor.type_mapping.into(),
            restrict_data_representation: topic_descriptor.restrict_data_representation,
        }
    }
}

/// Used to describe a named key field in a type with the offset from the start
/// of a struct.
#[derive(Debug, Clone)]
pub struct KeyDescriptor {
    pub(crate) name: String,
    pub(crate) m_offset: u32,
    pub(crate) index: u32,
}

impl From<cyclonedds_sys::dds_key_descriptor_t> for KeyDescriptor {
    fn from(key_descriptor: cyclonedds_sys::dds_key_descriptor_t) -> Self {
        Self {
            name: unsafe {
                CStr::from_ptr(key_descriptor.m_name)
                    .to_str()
                    .unwrap()
                    .to_string()
            },
            m_offset: key_descriptor.m_offset,
            index: key_descriptor.m_idx,
        }
    }
}

impl From<KeyDescriptor> for cyclonedds_sys::dds_key_descriptor_t {
    fn from(key_descriptor: KeyDescriptor) -> Self {
        cyclonedds_sys::dds_key_descriptor_t {
            m_name: CString::new(key_descriptor.name).unwrap().as_ptr(),
            m_offset: key_descriptor.m_offset,
            m_idx: key_descriptor.index,
        }
    }
}
#[derive(Debug, Clone)]
pub struct TypeMetaSer {
    pub(crate) data: Vec<u8>,
}

impl From<TypeMetaSer> for cyclonedds_sys::dds_type_meta_ser {
    fn from(value: TypeMetaSer) -> Self {
        cyclonedds_sys::dds_type_meta_ser {
            data: value.data.as_ptr(),
            sz: value.data.len() as u32,
        }
    }
}

impl From<cyclonedds_sys::dds_type_meta_ser> for TypeMetaSer {
    fn from(value: cyclonedds_sys::dds_type_meta_ser) -> Self {
        Self {
            data: unsafe { slice::from_raw_parts(value.data, value.sz as usize).to_vec() },
        }
    }
}

use std::ffi::c_char;
use std::os::raw::c_int;
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::FmtSubscriber;

// extern "C" {
//     fn dds_set_log_sink(sink: extern "C" fn(level: c_int, message: *const c_char));
// }

/// Map CycloneDDS log levels to `tracing` levels
fn convert_log_level(level: c_int) -> tracing::Level {
    match level {
        1 => tracing::Level::ERROR, // Fatal
        2 => tracing::Level::ERROR, // Error
        3 => tracing::Level::WARN,  // Warning
        4 => tracing::Level::INFO,  // Info
        5 => tracing::Level::DEBUG, // Debug
        6 => tracing::Level::TRACE, // Trace
        _ => tracing::Level::TRACE, // Default to trace
    }
}

/// Rust wrapper for the CycloneDDS log sink function
extern "C" fn rust_log_sink(level: c_int, message: *const c_char) {
    if message.is_null() {
        return;
    }

    unsafe {
        let c_str = CStr::from_ptr(message);
        let msg = c_str.to_string_lossy();

        let log_level = convert_log_level(level);

        match log_level {
            tracing::Level::ERROR => error!("{}", msg),
            tracing::Level::WARN => warn!("{}", msg),
            tracing::Level::INFO => info!("{}", msg),
            tracing::Level::DEBUG => debug!("{}", msg),
            tracing::Level::TRACE => trace!("{}", msg),
        }
    }
}

fn capture_cyclonedds_logs() -> String {
    let mut child = Command::new("your_cyclonedds_app")
        .env("CYCLONEDDS_LOG_LEVEL", "trace")
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start CycloneDDS process");

    let mut output = String::new();
    if let Some(ref mut stderr) = child.stderr {
        stderr.read_to_string(&mut output).unwrap();
    }

    output
}

/// Registers Rust logging sink with CycloneDDS
// pub fn enable_cyclonedds_tracing() {
//     unsafe {
//         dds_set_log_sink(rust_log_sink);
//     }
// }

#[cfg(test)]
mod tests {
    use tracing::trace;
    use tracing_subscriber::FmtSubscriber;

    use crate::domain::DomainParticipant;

    // use crate::internal::enable_cyclonedds_tracing;

    #[test]
    fn test_cyclonedds_trace_logging() {
        // Set the finest level for CycloneDDS logging
        std::env::set_var("CYCLONEDDS_LOG_LEVEL", "trace");

        // Initialize `tracing`
        let subscriber = FmtSubscriber::builder()
            .with_max_level(tracing::Level::TRACE)
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set tracing subscriber");

        let participant = DomainParticipant::default();
        // Enable CycloneDDS logging via tracing
        // enable_cyclonedds_tracing();

        // Simulate logging (CycloneDDS should now use `tracing` for its logs)
        trace!("CycloneDDS logging is now at TRACE level");

        // Insert your CycloneDDS usage here...
    }

    #[test]
    fn test_capture_cyclonedds_logs() {
        let logs = crate::internal::capture_cyclonedds_logs();
        assert!(logs.contains("TRACE"), "Logs did not capture TRACE level");
    }
}
