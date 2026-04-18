use std::fmt;

#[derive(Debug)]
pub struct Win32Error {
    pub code: u32,
    pub message: String,
}

impl Win32Error {
    pub fn last() -> Self {
        let code = unsafe { windows::Win32::Foundation::GetLastError().0 };
        Self {
            code,
            message: format!("Win32 error 0x{:08X}", code),
        }
    }
}

impl fmt::Display for Win32Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Win32Error(0x{:08X}): {}", self.code, self.message)
    }
}

impl std::error::Error for Win32Error {}
