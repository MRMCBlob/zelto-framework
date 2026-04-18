use anyhow::Result;
use windows::Win32::Foundation::{HWND, RECT, WPARAM, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::UI::Input::KeyboardAndMouse::EnableWindow;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::core::PCWSTR;
use crate::util::{to_wide, wide_ptr};

/// Pixel rect used to position controls.
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

/// Native Win32 push button.
pub struct NativeButton {
    pub hwnd: HWND,
    pub id: u16,
}

impl NativeButton {
    pub fn create(parent: HWND, label: &str, rect: Rect, id: u16) -> Result<Self> {
        let label_wide = to_wide(label);
        let class_wide = to_wide("BUTTON");
        let hinstance = unsafe { GetModuleHandleW(PCWSTR::null())? };

        let hwnd = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                wide_ptr(&class_wide),
                wide_ptr(&label_wide),
                WS_CHILD | WS_VISIBLE | WINDOW_STYLE(0x00000000), // BS_PUSHBUTTON = 0
                rect.x,
                rect.y,
                rect.width,
                rect.height,
                parent,
                windows::Win32::UI::WindowsAndMessaging::HMENU(id as isize as *mut _),
                hinstance,
                None,
            )?
        };

        Ok(Self { hwnd, id })
    }

    pub fn set_label(&self, label: &str) {
        let wide = to_wide(label);
        unsafe { let _ = SetWindowTextW(self.hwnd, wide_ptr(&wide)); }
    }

    pub fn set_enabled(&self, enabled: bool) {
        unsafe { let _ = EnableWindow(self.hwnd, enabled); }
    }

    pub fn set_rect(&self, rect: Rect) {
        unsafe {
            let _ = SetWindowPos(
                self.hwnd,
                HWND::default(),
                rect.x, rect.y, rect.width, rect.height,
                SWP_NOZORDER | SWP_NOACTIVATE,
            );
        }
    }
}

/// Native Win32 single-line text edit.
pub struct NativeTextInput {
    pub hwnd: HWND,
    pub id: u16,
}

impl NativeTextInput {
    pub fn create(parent: HWND, placeholder: &str, rect: Rect, id: u16) -> Result<Self> {
        let class_wide = to_wide("EDIT");
        let text_wide = to_wide("");
        let hinstance = unsafe { GetModuleHandleW(PCWSTR::null())? };

        let hwnd = unsafe {
            CreateWindowExW(
                WS_EX_CLIENTEDGE,
                wide_ptr(&class_wide),
                wide_ptr(&text_wide),
                WS_CHILD | WS_VISIBLE | WINDOW_STYLE(0x00000080 | 0x00000040), // ES_AUTOHSCROLL | ES_LEFT
                rect.x,
                rect.y,
                rect.width,
                rect.height,
                parent,
                windows::Win32::UI::WindowsAndMessaging::HMENU(id as isize as *mut _),
                hinstance,
                None,
            )?
        };

        Ok(Self { hwnd, id })
    }

    pub fn get_text(&self) -> String {
        let len = unsafe { GetWindowTextLengthW(self.hwnd) };
        if len == 0 {
            return String::new();
        }
        let mut buf: Vec<u16> = vec![0u16; (len + 1) as usize];
        unsafe { GetWindowTextW(self.hwnd, &mut buf); }
        String::from_utf16_lossy(&buf[..len as usize])
    }

    pub fn set_text(&self, text: &str) {
        let wide = to_wide(text);
        unsafe { let _ = SetWindowTextW(self.hwnd, wide_ptr(&wide)); }
    }

    pub fn set_rect(&self, rect: Rect) {
        unsafe {
            let _ = SetWindowPos(
                self.hwnd,
                HWND::default(),
                rect.x, rect.y, rect.width, rect.height,
                SWP_NOZORDER | SWP_NOACTIVATE,
            );
        }
    }
}

/// Native Win32 static text label.
pub struct NativeLabel {
    pub hwnd: HWND,
}

impl NativeLabel {
    pub fn create(parent: HWND, text: &str, rect: Rect) -> Result<Self> {
        let class_wide = to_wide("STATIC");
        let text_wide = to_wide(text);
        let hinstance = unsafe { GetModuleHandleW(PCWSTR::null())? };

        let hwnd = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                wide_ptr(&class_wide),
                wide_ptr(&text_wide),
                WS_CHILD | WS_VISIBLE,
                rect.x,
                rect.y,
                rect.width,
                rect.height,
                parent,
                None,
                hinstance,
                None,
            )?
        };

        Ok(Self { hwnd })
    }

    pub fn set_text(&self, text: &str) {
        let wide = to_wide(text);
        unsafe { let _ = SetWindowTextW(self.hwnd, wide_ptr(&wide)); }
    }

    pub fn set_rect(&self, rect: Rect) {
        unsafe {
            let _ = SetWindowPos(
                self.hwnd,
                HWND::default(),
                rect.x, rect.y, rect.width, rect.height,
                SWP_NOZORDER | SWP_NOACTIVATE,
            );
        }
    }
}

/// Native Win32 checkbox.
pub struct NativeCheckbox {
    pub hwnd: HWND,
    pub id: u16,
}

impl NativeCheckbox {
    pub fn create(parent: HWND, label: &str, checked: bool, rect: Rect, id: u16) -> Result<Self> {
        let class_wide = to_wide("BUTTON");
        let text_wide = to_wide(label);
        let hinstance = unsafe { GetModuleHandleW(PCWSTR::null())? };

        let hwnd = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                wide_ptr(&class_wide),
                wide_ptr(&text_wide),
                // BS_AUTOCHECKBOX = 0x3, WS_CHILD | WS_VISIBLE
                WS_CHILD | WS_VISIBLE | WINDOW_STYLE(0x00000003),
                rect.x,
                rect.y,
                rect.width,
                rect.height,
                parent,
                windows::Win32::UI::WindowsAndMessaging::HMENU(id as isize as *mut _),
                hinstance,
                None,
            )?
        };

        if checked {
            unsafe {
                SendMessageW(hwnd, BM_SETCHECK, WPARAM(1), LPARAM(0));
            }
        }

        Ok(Self { hwnd, id })
    }

    pub fn is_checked(&self) -> bool {
        let result = unsafe { SendMessageW(self.hwnd, BM_GETCHECK, WPARAM(0), LPARAM(0)) };
        result.0 == 1
    }

    pub fn set_checked(&self, checked: bool) {
        unsafe {
            SendMessageW(self.hwnd, BM_SETCHECK, WPARAM(checked as usize), LPARAM(0));
        }
    }
}
