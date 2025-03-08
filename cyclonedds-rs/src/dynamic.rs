//! The Dynamic Type API to construct and manipulate data types.

use std::ffi::CString;

use cyclonedds_sys::{dds_dynamic_type, dds_dynamic_type_t};

use crate::core::{Entity, ReturnCodes};

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
    dynamic_type: dds_dynamic_type_t,
}

impl DynamicType {
    pub(crate) fn create(
        entity: &impl Entity,
        descriptor: cyclonedds_sys::dds_dynamic_type_descriptor_t,
    ) -> Result<DynamicType, ReturnCodes> {
        // todo!("not implemented");
        let dynamic_type;
        unsafe {
            dynamic_type = cyclonedds_sys::dds_dynamic_type_create(
                entity.participant().unwrap().participant,
                descriptor,
            );
        }
        // TODO: check for errors
        Ok(DynamicType { dynamic_type })
    }
    /// Set the extensibility of a Dynamic Type.
    /// -
    pub fn set_extensibility(
        &mut self,
        extensibility: &TypeExtensibility,
    ) -> Result<(), ReturnCodes> {
        let extensibility = match extensibility {
            TypeExtensibility::Final => {
                cyclonedds_sys::dds_dynamic_type_extensibility_DDS_DYNAMIC_TYPE_EXT_FINAL
            }
            TypeExtensibility::Appendable => {
                cyclonedds_sys::dds_dynamic_type_extensibility_DDS_DYNAMIC_TYPE_EXT_APPENDABLE
            }
            TypeExtensibility::Mutable => {
                cyclonedds_sys::dds_dynamic_type_extensibility_DDS_DYNAMIC_TYPE_EXT_MUTABLE
            }
        };

        let return_code;
        unsafe {
            return_code = cyclonedds_sys::dds_dynamic_type_set_extensibility(
                &mut self.dynamic_type,
                extensibility,
            );
        };

        if return_code != 0 {
            Err(ReturnCodes::from(return_code))
        } else {
            Ok(())
        }
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

pub struct MemberDescriptor {
    name: String,
    id: u32,
    dynamic_type: TypeSpec,
    default_value: String,
    index: u32,
    num_labels: u32,
    labels: Vec<i32>,
    default_label: bool,
}

impl From<MemberDescriptor> for cyclonedds_sys::dds_dynamic_member_descriptor_t {
    fn from(member_descriptor: MemberDescriptor) -> Self {
        cyclonedds_sys::dds_dynamic_member_descriptor_t {
            name: CString::new(member_descriptor.name).unwrap().as_ptr(),
            id: member_descriptor.id,
            type_: member_descriptor.dynamic_type.into(),
            default_value: CString::new(member_descriptor.default_value)
                .unwrap()
                .as_ptr() as *mut i8,
            index: member_descriptor.index,
            num_labels: member_descriptor.num_labels,
            labels: &mut member_descriptor.labels.clone() as *mut Vec<i32> as *mut i32,
            default_label: member_descriptor.default_label,
        }
    }
}
/// Dynamic Type specification: a reference to dynamic type, which can be a
/// primitive type kind (just the type kind enumeration value), or a
/// (primitive or non-primitive) dynamic type reference.
pub struct TypeSpec {}

impl From<TypeSpec> for cyclonedds_sys::dds_dynamic_type_spec_t {
    fn from(value: TypeSpec) -> Self {
        todo!()
    }
}
