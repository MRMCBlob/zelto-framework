use anyhow::{Result, anyhow};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM, RECT};
use windows::Win32::Graphics::Gdi::{HBRUSH, COLOR_WINDOW};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::core::PCWSTR;
use crate::util::{to_wide, wide_ptr};

const ZELTO_CLASS: &str = "ZeltoWindow";

/// A top-level Win32 window managed by Zelto.
pub struct ZeltoWindow {
    pub hwnd: HWND,
}

impl ZeltoWindow {
    pub fn register_class() -> Result<()> {
        let class_name = to_wide(ZELTO_CLASS);
        let hinstance = unsafe { GetModuleHandleW(PCWSTR::null())? };

        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wnd_proc),
            hInstance: hinstance.into(),
            hCursor: unsafe { LoadCursorW(None, IDC_ARROW)? },
            // COLOR_WINDOW + 1 is the standard Win32 idiom for default window background
            hbrBackground: HBRUSH((COLOR_WINDOW.0 as isize + 1) as *mut _),
            lpszClassName: wide_ptr(&class_name),
            ..Default::default()
        };

        unsafe {
            let atom = RegisterClassExW(&wc);
            if atom == 0 {
                return Err(anyhow!("RegisterClassExW failed"));
            }
        }
        Ok(())
    }

    pub fn new(title: &str, width: u32, height: u32, resizable: bool) -> Result<Self> {
        let title_wide = to_wide(title);
        let class_wide = to_wide(ZELTO_CLASS);
        let hinstance = unsafe { GetModuleHandleW(PCWSTR::null())? };

        let style = if resizable {
            WS_OVERLAPPEDWINDOW
        } else {
            WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX
        };

        // Adjust rect so client area matches requested width/height
        let mut rect = RECT {
            left: 0,
            top: 0,
            right: width as i32,
            bottom: height as i32,
        };
        unsafe { AdjustWindowRect(&mut rect, style, false)?; }

        let hwnd = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                wide_ptr(&class_wide),
                wide_ptr(&title_wide),
                style,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                rect.right - rect.left,
                rect.bottom - rect.top,
                None,
                None,
                hinstance,
                None,
            )?
        };

        unsafe {
            ShowWindow(hwnd, SW_SHOWNORMAL);
            windows::Win32::Graphics::Gdi::UpdateWindow(hwnd);
        }

        Ok(Self { hwnd })
    }

    pub fn set_title(&self, title: &str) {
        let wide = to_wide(title);
        unsafe { let _ = SetWindowTextW(self.hwnd, wide_ptr(&wide)); }
    }

    pub fn client_rect(&self) -> (u32, u32) {
        let mut rect = RECT::default();
        unsafe { let _ = GetClientRect(self.hwnd, &mut rect); }
        (rect.right as u32, rect.bottom as u32)
    }
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        WM_COMMAND => {
            // Control notifications arrive here — runtime registers a hook via SetWindowLongPtrW
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
