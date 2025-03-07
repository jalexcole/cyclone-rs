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


pub struct TopicDescriptor {
    /// Size of topic type
    m_size: u32,
    /// Alignment of topic type
    m_align: u32,
    /// Flags
    m_flagset: u32,
    /// Number of keys (can be 0)
    m_nkeys: u32,
    /// Type name
    m_typename: String,
    /// Key descriptors (NULL iff m_nkeys 0)
    m_keys: Vec<cyclonedds_sys::dds_key_descriptor_t>,
    /// Number of operations in m_ops
    m_nops: u32,
    /// Marshalling meta data
    m_ops: Vec<u32>,
    /// XML topic description meta data
    m_meta: String,
    /// XCDR2 serialized TypeInformation, only present if flag 
    /// [cyclonedds_sys::DDS_TOPIC_XTYPES_METADATA] is set
    type_information: Vec<u8>,
    /// XCDR2 serialized TypeMapping: maps type-id to type object and minimal 
    /// to complete type id, only present if flag 
    /// [cyclonedds_sys::DDS_TOPIC_XTYPES_METADATA] is set
    type_mapping: cyclonedds_sys::dds_type_meta_ser,
    /// restrictions on the data representations allowed for the top-level type 
    /// for this topic, only present if flag 
    /// [cyclonedds_sys::DDS_TOPIC_RESTRICT_DATA_REPRESENTATION]
    restrict_data_representation: u32,
}

impl From<cyclonedds_sys::dds_topic_descriptor_t> for TopicDescriptor {
    fn from(topic_descriptor: cyclonedds_sys::dds_topic_descriptor_t) -> Self {
        Self {
            m_size: topic_descriptor.m_size,
            m_align: topic_descriptor.m_align,
            m_flagset: topic_descriptor.m_flagset,
            m_nkeys: topic_descriptor.m_nkeys,
            m_typename: todo!("topic_descriptor.m_typename"),
            m_keys: todo!("topic_descriptor.m_keys"),
            m_nops: topic_descriptor.m_nops,
            m_ops: todo!(),
            m_meta:todo!("topic_descriptor.m_meta"),
            type_information:todo!("topic_descriptor.type_information"),
            type_mapping: topic_descriptor.type_mapping,
            restrict_data_representation: topic_descriptor.restrict_data_representation,
        }
    }
}

impl From <TopicDescriptor> for cyclonedds_sys::dds_topic_descriptor_t {
    fn from(topic_descriptor: TopicDescriptor) -> Self {
        cyclonedds_sys::dds_topic_descriptor_t {
            m_size: topic_descriptor.m_size,
            m_align: topic_descriptor.m_align,
            m_flagset: topic_descriptor.m_flagset,
            m_nkeys: topic_descriptor.m_nkeys,
            m_typename: todo!("topic_descriptor.m_typename"),
            m_keys: todo!("topic_descriptor.m_keys"),
            m_nops: topic_descriptor.m_nops,
            m_ops: todo!(),
            m_meta:todo!("topic_descriptor.m_meta"),
            type_information:todo!("topic_descriptor.type_information"),
            type_mapping: topic_descriptor.type_mapping,
            restrict_data_representation: topic_descriptor.restrict_data_representation,
        }
    }
}



