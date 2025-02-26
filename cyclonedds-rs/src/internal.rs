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
