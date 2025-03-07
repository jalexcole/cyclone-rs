use std::{ptr::null_mut, time::Duration};

use crate::core::ReturnCodes;




pub struct TypeObject {
    type_object: cyclonedds_sys::dds_typeobj_t
}

impl TypeObject {
    pub(crate) fn new(entity: cyclonedds_sys::dds_entity_t, typeid: &TypeId, duration: Duration) -> Result<TypeObject, ReturnCodes> {

        let return_code;
        let mut type_obj = null_mut();
        unsafe {
            return_code = cyclonedds_sys::dds_get_typeobj(entity, &typeid.type_id, duration.as_nanos() as i64, type_obj);
        }

        if return_code != 0 {
            Err(ReturnCodes::from(return_code))
        } else {
            todo!()
        }



    }
}

pub struct TypeId {
    type_id: cyclonedds_sys::dds_typeid_t
}