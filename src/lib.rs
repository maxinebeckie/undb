//! library part

pub fn read_uninit_mem() {
    todo!();
}

pub mod create_invalid_primitive_values {

    //TODO:

    // null Refs, boxes, fn ptrs

    // bools not 0 or 1

    // ? enums with invalid discriminant values

    // chars not valid, ?nonsurrogate Unicode code points

    // str not valid UTF-8

    // fat ptrs with invalid vtables, slice lens

    // any value of !
}

pub fn reference_outlive_referent() {
    todo!();
}

pub fn write_shared_ref() {
    todo!();
}

pub fn many_mutable_access() {
    todo!();
}

pub fn deref_null_ptr() {
    todo!();
}

pub fn deref_incorrectly_aligned_ptr() {
    todo!();
}

pub fn deref_dangling_ptr() {
    todo!();
}

pub fn access_mem_outside_alloc() {
    todo!();
}

pub fn data_race() {
    todo!();
}

pub fn unwind_across_ffi() {
    todo!();
}
