//! The Dynamic Type API to construct and manipulate data types.

use cyclonedds_sys::{dds_dynamic_type, dds_dynamic_type_t};

use crate::{Entity, ReturnCodes};

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