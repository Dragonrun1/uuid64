use std::ffi::{c_char, CStr, CString};

#[unsafe(no_mangle)]
pub extern "C" fn uuid64_new_v4() -> *mut c_char {
    CString::new(uuid64_core::Uuid64::new_v4())
        .expect("uuid string contains no null bytes")
        .into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn uuid64_new_v7() -> *mut c_char {
    CString::new(uuid64_core::Uuid64::new_v7())
        .expect("uuid string contains no null bytes")
        .into_raw()
}

/// # Safety
/// `ptr` must be a valid null-terminated string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uuid64_encode_uuid(ptr: *const c_char) -> *mut c_char {
    let input = unsafe { CStr::from_ptr(ptr) }.to_string_lossy();
    match uuid64_core::Uuid64::encode_uuid(&input) {
        Ok(encoded) => CString::new(encoded)
            .expect("encoded string contains no null bytes")
            .into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// # Safety
/// `ptr` must be a valid null-terminated string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uuid64_decode_uuid(ptr: *const c_char) -> *mut c_char {
    let input = unsafe { CStr::from_ptr(ptr) }.to_string_lossy();
    match uuid64_core::Uuid64::decode_uuid(&input) {
        Ok(decoded) => CString::new(decoded)
            .expect("decoded string contains no null bytes")
            .into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// # Safety
/// `ptr` must be a pointer previously returned by this library
/// and must not be used after this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uuid64_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        drop(unsafe { CString::from_raw(ptr) });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    fn to_cstring(s: &str) -> CString {
        CString::new(s).unwrap()
    }

    unsafe fn ptr_to_string(ptr: *mut c_char) -> String {
        assert!(!ptr.is_null());
        let s = unsafe { CStr::from_ptr(ptr) }
            .to_str()
            .unwrap()
            .to_owned();
        unsafe { uuid64_free(ptr) };
        s
    }

    #[test]
    fn new_v4_returns_nonnull() {
        let ptr = uuid64_new_v4();
        assert!(!ptr.is_null());
        unsafe { uuid64_free(ptr) };
    }

    #[test]
    fn new_v7_returns_nonnull() {
        let ptr = uuid64_new_v7();
        assert!(!ptr.is_null());
        unsafe { uuid64_free(ptr) };
    }

    #[test]
    fn new_v4_is_22_chars() {
        let ptr = uuid64_new_v4();
        let s = unsafe { ptr_to_string(ptr) };
        assert_eq!(s.len(), 22);
    }

    #[test]
    fn new_v7_is_22_chars() {
        let ptr = uuid64_new_v7();
        let s = unsafe { ptr_to_string(ptr) };
        assert_eq!(s.len(), 22);
    }

    #[test]
    fn encode_decode_roundtrip() {
        let input = to_cstring("550e8400-e29b-41d4-a716-446655440000");
        let encoded_ptr = unsafe { uuid64_encode_uuid(input.as_ptr()) };
        let encoded = unsafe { ptr_to_string(encoded_ptr) };
        assert_eq!(encoded.len(), 22);

        let encoded_c = to_cstring(&encoded);
        let decoded_ptr = unsafe { uuid64_decode_uuid(encoded_c.as_ptr()) };
        let decoded = unsafe { ptr_to_string(decoded_ptr) };
        assert_eq!(decoded, "550e8400-e29b-41d4-a716-446655440000");
    }

    #[test]
    fn encode_uuid_invalid_input_returns_null() {
        let input = to_cstring("not-a-uuid");
        let ptr = unsafe { uuid64_encode_uuid(input.as_ptr()) };
        assert!(ptr.is_null());
    }

    #[test]
    fn decode_uuid_invalid_length_returns_null() {
        let input = to_cstring("tooshort");
        let ptr = unsafe { uuid64_decode_uuid(input.as_ptr()) };
        assert!(ptr.is_null());
    }

    #[test]
    fn decode_uuid_invalid_character_returns_null() {
        let input = to_cstring("!!!!!!!!!!!!!!!!!!!!!!");
        let ptr = unsafe { uuid64_decode_uuid(input.as_ptr()) };
        assert!(ptr.is_null());
    }

    #[test]
    fn free_null_is_safe() {
        // Must not panic or segfault
        unsafe { uuid64_free(std::ptr::null_mut()) };
    }

    #[test]
    fn v4_roundtrip_via_ffi() {
        let encoded_ptr = uuid64_new_v4();
        let encoded = unsafe {
            CStr::from_ptr(encoded_ptr).to_str().unwrap().to_owned()
        };
        let encoded_c = to_cstring(&encoded);
        let decoded_ptr = unsafe { uuid64_decode_uuid(encoded_c.as_ptr()) };
        let decoded = unsafe { ptr_to_string(decoded_ptr) };

        // Re-encode the decoded UUID and check it matches the original
        let reencoded_c = to_cstring(&decoded);
        let reencoded_ptr = unsafe { uuid64_encode_uuid(reencoded_c.as_ptr()) };
        let reencoded = unsafe { ptr_to_string(reencoded_ptr) };

        unsafe { uuid64_free(encoded_ptr) };
        assert_eq!(encoded, reencoded);
    }
}
