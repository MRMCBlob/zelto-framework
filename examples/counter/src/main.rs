//! Counter — Phase 1 Zelto example (raw Win32 API).
//!
//! Demonstrates what the TSX compiler will auto-generate in Phase 2.
//! The TSX source this corresponds to:
//!
//! ```tsx
//! function Counter() {
//!   const [count, setCount] = useState(0);
//!   return (
//!     <Window title="Zelto Counter" width={400} height={200}>
//!       <View style={{ flexDirection: "column", padding: 20, gap: 12 }}>
//!         <Text>{count}</Text>
//!         <Button onClick={() => setCount(count + 1)}>+1</Button>
//!         <Button onClick={() => setCount(0)}>Reset</Button>
//!       </View>
//!     </Window>
//!   );
//! }
//! ```

#![windows_subsystem = "windows"]

use anyhow::Result;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Graphics::Gdi::{HBRUSH, COLOR_WINDOW};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::core::PCWSTR;

const ID_BTN_INC: u16 = 101;
const ID_BTN_RESET: u16 = 102;
const CLASS_NAME: &str = "ZeltoCounter";

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

// Per-window state stored via SetWindowLongPtrW (GWLP_USERDATA).
struct WindowState {
    count: i32,
    label: HWND,
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CREATE => {
            let hinstance = GetModuleHandleW(PCWSTR::null()).unwrap();

            // Label for count display
            let label = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                PCWSTR(wide("STATIC").as_ptr()),
                PCWSTR(wide("0").as_ptr()),
                WS_CHILD | WS_VISIBLE | WINDOW_STYLE(0x00000001), // SS_CENTER
                20, 20, 340, 50,
                hwnd, None, hinstance, None,
            ).unwrap();

            // +1 button
            CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                PCWSTR(wide("BUTTON").as_ptr()),
                PCWSTR(wide("+1").as_ptr()),
                WS_CHILD | WS_VISIBLE,
                20, 90, 155, 40,
                hwnd,
                HMENU(ID_BTN_INC as isize as *mut _),
                hinstance, None,
            ).unwrap();

            // Reset button
            CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                PCWSTR(wide("BUTTON").as_ptr()),
                PCWSTR(wide("Reset").as_ptr()),
                WS_CHILD | WS_VISIBLE,
                205, 90, 155, 40,
                hwnd,
                HMENU(ID_BTN_RESET as isize as *mut _),
                hinstance, None,
            ).unwrap();

            // Store mutable state as raw box pointer
            let state = Box::new(WindowState { count: 0, label });
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, Box::into_raw(state) as isize);

            LRESULT(0)
        }
        WM_COMMAND => {
            let ctrl_id = (wparam.0 & 0xFFFF) as u16;
            let notification = ((wparam.0 >> 16) & 0xFFFF) as u16;

            if notification == 0 {
                // BN_CLICKED
                let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut WindowState;
                if !ptr.is_null() {
                    let state = &mut *ptr;
                    match ctrl_id {
                        ID_BTN_INC => state.count += 1,
                        ID_BTN_RESET => state.count = 0,
                        _ => {}
                    }
                    let text = wide(&state.count.to_string());
                    SetWindowTextW(state.label, PCWSTR(text.as_ptr())).unwrap();
                }
            }
            LRESULT(0)
        }
        WM_DESTROY => {
            // Clean up boxed state
            let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut WindowState;
            if !ptr.is_null() {
                drop(Box::from_raw(ptr));
            }
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

fn main() -> Result<()> {
    unsafe {
        let hinstance = GetModuleHandleW(PCWSTR::null())?;
        let class_wide = wide(CLASS_NAME);

        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wnd_proc),
            hInstance: hinstance.into(),
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hbrBackground: HBRUSH((COLOR_WINDOW.0 as isize + 1) as *mut _),
            lpszClassName: PCWSTR(class_wide.as_ptr()),
            ..Default::default()
        };
        RegisterClassExW(&wc);

        let title_wide = wide("Zelto Counter");
        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            PCWSTR(class_wide.as_ptr()),
            PCWSTR(title_wide.as_ptr()),
            WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX,
            CW_USEDEFAULT, CW_USEDEFAULT, 400, 175,
            None, None, hinstance, None,
        )?;

        ShowWindow(hwnd, SW_SHOWNORMAL);
        windows::Win32::Graphics::Gdi::UpdateWindow(hwnd);

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, HWND::default(), 0, 0).as_bool() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

    Ok(())
}
