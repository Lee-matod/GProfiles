use core::mem::MaybeUninit;
use std::ffi::OsString;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::{mem, path, thread};

use image::RgbaImage;
use windows::core::{Result, PCWSTR};
use windows::Win32::Foundation::{GetLastError, BOOL, ERROR_ALREADY_EXISTS, HANDLE, HWND, LPARAM};
use windows::Win32::Graphics::Gdi::{
    DeleteObject, GetBitmapBits, GetDC, GetObjectW, ReleaseDC, BITMAP, BITMAPINFOHEADER, HBITMAP,
    HGDIOBJ,
};
use windows::Win32::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES;
use windows::Win32::System::ProcessStatus::GetModuleFileNameExW;
use windows::Win32::System::Threading::{
    CreateMutexExW, OpenProcess, MUTEX_ALL_ACCESS, MUTEX_MODIFY_STATE,
    PROCESS_QUERY_LIMITED_INFORMATION,
};
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
use windows::Win32::UI::Shell::{SHGetFileInfoW, SHFILEINFOW, SHGFI_FLAGS};
use windows::Win32::UI::WindowsAndMessaging::{
    DestroyIcon, DispatchMessageW, EnumWindows, GetIconInfo, GetWindowTextW,
    GetWindowThreadProcessId, IsWindowVisible, PeekMessageW, TranslateMessage, HICON, HWND_MESSAGE,
    ICONINFO, MSG, PM_REMOVE,
};

use crate::remapper::KeyboardKey;
use crate::utils::GPROFILES;

static mut APPLICATIONS: Vec<(HWND, String)> = Vec::new();

#[cfg(target_os = "windows")]
pub fn encode_wide(text: &str) -> Vec<u16> {
    use std::ffi::OsStr;

    OsStr::new(text)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}
#[cfg(target_os = "linux")]
pub fn encode_wide(_: &str) -> Vec<u16> {
    vec![]
}

pub unsafe fn get_lock() -> Option<HANDLE> {
    let mutex_name = encode_wide(GPROFILES);

    let mutex = CreateMutexExW(
        None,
        PCWSTR::from_raw(mutex_name.as_ptr()),
        MUTEX_MODIFY_STATE.0,
        MUTEX_ALL_ACCESS.0,
    )
    .unwrap();

    if GetLastError() == ERROR_ALREADY_EXISTS {
        return None;
    }
    return Some(mutex);
}

pub unsafe fn key_input(callback: impl FnOnce(Option<KeyboardKey>) + Send + 'static) -> () {
    thread::spawn(move || {
        let mut msg = MSG::default();

        loop {
            if PeekMessageW(&mut msg, HWND_MESSAGE, 0, 0, PM_REMOVE).as_bool() {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
            for vkey in 1..=255 {
                if GetAsyncKeyState(vkey as i32) as u16 & 0x8000 != 0 {
                    let key = KeyboardKey::from(vkey as u64);
                    if key == KeyboardKey::Escape {
                        slint::invoke_from_event_loop(move || callback(None)).unwrap();
                    } else {
                        slint::invoke_from_event_loop(move || callback(Some(key))).unwrap();
                    }
                    return;
                }
            }
        }
    });
}

pub unsafe fn foreground_apps(needle: &str) -> Vec<path::PathBuf> {
    let mut foreground: Vec<path::PathBuf> = Vec::new();
    let mut active_windows: Vec<HWND> = Vec::new();
    APPLICATIONS.clear();

    EnumWindows(Some(enum_callback), None).unwrap();
    for (hwnd, title) in APPLICATIONS.clone() {
        if IsWindowVisible(hwnd).as_bool() && !title.is_empty() {
            active_windows.push(hwnd);
        }
    }

    for hwnd in active_windows {
        let mut process_id = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut process_id));
        if process_id == 0 {
            continue;
        };

        let process_handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, process_id);
        if process_handle.is_err() {
            continue;
        };

        let mut module_filename = vec![0u16; 1024];
        let size = GetModuleFileNameExW(process_handle.unwrap(), None, &mut module_filename);
        if size == 0 {
            continue;
        };

        #[cfg(target_os = "windows")]
        let module_filename = OsString::from_wide(&module_filename[..size as usize]);
        #[cfg(target_os = "linux")]
        let module_filename = OsString::new();

        let filepath: String = module_filename.to_string_lossy().to_string();
        let as_path = path::PathBuf::from(&filepath);
        let name = match as_path.file_name() {
            Some(filename) => filename.to_string_lossy().to_string(),
            None => continue,
        };
        if !name.to_lowercase().contains(needle) || foreground.contains(&as_path) {
            continue;
        };
        foreground.push(as_path);
    }
    foreground
}

pub unsafe fn get_icon(path: &str) -> Result<RgbaImage> {
    let mut shfi = SHFILEINFOW {
        hIcon: HICON(0),
        iIcon: 0,
        dwAttributes: 0,
        szDisplayName: [0; 260],
        szTypeName: [0; 80],
    };
    let path = encode_wide(path);

    SHGetFileInfoW(
        PCWSTR(path.as_ptr()),
        FILE_FLAGS_AND_ATTRIBUTES(0),
        Some(&mut shfi),
        std::mem::size_of::<u32>() as u32,
        SHGFI_FLAGS(0x000000100),
    );
    let hicon = shfi.hIcon;
    let image = hicon_to_image(hicon);
    DestroyIcon(hicon)?;
    return image;
}

unsafe fn hicon_to_image(icon: HICON) -> Result<RgbaImage> {
    let biheader_size_u32 = u32::try_from(mem::size_of::<BITMAPINFOHEADER>())?;
    let mut info = ICONINFO {
        fIcon: BOOL(0),
        xHotspot: 0,
        yHotspot: 0,
        hbmMask: HBITMAP(0),
        hbmColor: HBITMAP(0),
    };
    GetIconInfo(icon, &mut info)?;
    DeleteObject(info.hbmMask);
    let mut bitmap: MaybeUninit<BITMAP> = MaybeUninit::uninit();

    GetObjectW(
        HGDIOBJ(info.hbmColor.0 as isize),
        mem::size_of::<BITMAP>() as i32,
        Some(bitmap.as_mut_ptr() as *mut std::ffi::c_void),
    );

    let bitmap = bitmap.assume_init_ref();

    let width_u32 = u32::try_from(bitmap.bmWidth)?;
    let height_u32 = u32::try_from(bitmap.bmHeight)?;
    let width_usize = usize::try_from(bitmap.bmWidth)?;
    let height_usize = usize::try_from(bitmap.bmHeight)?;
    let buf_size = width_usize
        .checked_mul(height_usize)
        .and_then(|size| size.checked_mul(4))
        .unwrap();
    let mut buf: Vec<u8> = Vec::with_capacity(buf_size);

    let dc = GetDC(HWND(0));

    BITMAPINFOHEADER {
        biSize: biheader_size_u32,
        biWidth: bitmap.bmWidth,
        biHeight: -bitmap.bmHeight.abs(),
        biPlanes: 1,
        biBitCount: 32,
        biCompression: 0,
        biSizeImage: 0,
        biXPelsPerMeter: 0,
        biYPelsPerMeter: 0,
        biClrUsed: 0,
        biClrImportant: 0,
    };

    let mut bmp: Vec<u8> = vec![0; buf_size];
    let _mr_right = GetBitmapBits(
        HBITMAP(info.hbmColor.0 as isize),
        buf_size as i32,
        bmp.as_mut_ptr() as *mut std::ffi::c_void,
    );
    buf.set_len(bmp.capacity());
    ReleaseDC(windows::Win32::Foundation::HWND(0), dc);
    DeleteObject(info.hbmColor);

    for chunk in bmp.chunks_exact_mut(4) {
        let [b, _, r, _] = chunk else { unreachable!() };
        mem::swap(b, r);
    }
    Ok(RgbaImage::from_vec(width_u32, height_u32, bmp).unwrap())
}

unsafe extern "system" fn enum_callback(hwnd: HWND, _: LPARAM) -> BOOL {
    let mut buffer = [0u16; 512];
    if GetWindowTextW(hwnd, &mut buffer) > 0 {
        let title = String::from_utf16_lossy(&buffer);
        APPLICATIONS.push((hwnd, title));
    };
    true.into()
}
