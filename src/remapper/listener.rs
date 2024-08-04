use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_KEYBOARD, KEYBD_EVENT_FLAGS, VIRTUAL_KEY,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageW, GetMessageW, SetWindowsHookExA, TranslateMessage,
    UnhookWindowsHookEx, KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL, WM_KEYDOWN,
};

use super::ACTIVE_KEYMAP;

unsafe extern "system" fn keyboard_hook(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code >= 0 && w_param.0 as u32 == WM_KEYDOWN {
        let kb_hook_struct = *(l_param.0 as *const KBDLLHOOKSTRUCT);
        let keycode = kb_hook_struct.vkCode as u16;

        if let Some(keybinds) = ACTIVE_KEYMAP.get_mut().unwrap() {
            if let Some(new_key) = keybinds.get(&keycode) {
                let mut input = INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: std::mem::zeroed(),
                };
                input.Anonymous.ki.wVk = VIRTUAL_KEY(new_key.clone());
                input.Anonymous.ki.dwFlags = KEYBD_EVENT_FLAGS { 0: 0 };
                SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
                return LRESULT(1);
            }
        }
    }
    CallNextHookEx(None, n_code, w_param, l_param)
}

pub unsafe fn set_hook() -> () {
    let h_hook = SetWindowsHookExA(WH_KEYBOARD_LL, Some(keyboard_hook), None, 0);

    if h_hook.is_err() {
        return;
    }

    let mut msg = MSG::default();
    while GetMessageW(&mut msg, None, 0, 0).as_bool() {
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }

    UnhookWindowsHookEx(h_hook.unwrap()).unwrap();
}
