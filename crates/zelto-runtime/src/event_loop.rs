use anyhow::Result;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Foundation::HWND;

/// Run the Win32 message loop until WM_QUIT.
pub fn run_message_loop() -> Result<i32> {
    let mut msg = MSG::default();
    loop {
        let ret = unsafe { GetMessageW(&mut msg, HWND::default(), 0, 0) };
        match ret.0 {
            -1 => return Err(anyhow::anyhow!("GetMessageW failed")),
            0 => return Ok(msg.wParam.0 as i32),
            _ => unsafe {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }
}
