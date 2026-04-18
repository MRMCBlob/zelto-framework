use windows::core::PCWSTR;

/// Convert &str to null-terminated UTF-16 Vec for Win32 APIs.
pub fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

pub fn wide_ptr(v: &[u16]) -> PCWSTR {
    PCWSTR(v.as_ptr())
}
