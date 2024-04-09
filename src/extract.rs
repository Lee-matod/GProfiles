// I have no clue how this works, but it does!

use core::mem::MaybeUninit;
use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::{mem, path};

use image::RgbaImage;
use winapi::shared::minwindef::{DWORD, LPVOID};
use winapi::shared::windef::{HBITMAP, HICON};
use winapi::um::shellapi::{SHGetFileInfoW, SHFILEINFOW};
use winapi::um::wingdi::{
    DeleteObject, GetBitmapBits, GetObjectW, BITMAP, BITMAPINFOHEADER, BI_RGB,
};
use winapi::um::winnt::VOID;
use winapi::um::winuser::{DestroyIcon, GetIconInfo, ICONINFO};
use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
use windows::Win32::Graphics::Gdi::{GetDC, ReleaseDC};
use windows::Win32::System::ProcessStatus::GetModuleFileNameExW;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowTextW, GetWindowThreadProcessId, IsWindowVisible,
};

static mut APPLICATIONS: Vec<(HWND, String)> = Vec::new();

pub fn safe_canonicalize(path: &path::Path) -> String {
    let canon = match path.canonicalize() {
        Ok(pathbuf) => pathbuf,
        Err(_) => return path.to_string_lossy().to_string(),
    };
    let full_path = canon.to_string_lossy().to_string();
    match full_path.strip_prefix("\\\\?\\") {
        Some(stripped) => stripped.to_string(),
        None => full_path,
    }
}

unsafe extern "system" fn enum_callback(hwnd: HWND, _: LPARAM) -> BOOL {
    let mut buffer = [0u16; 512];
    if GetWindowTextW(hwnd, &mut buffer) > 0 {
        let title = String::from_utf16_lossy(&buffer);
        APPLICATIONS.push((hwnd, title));
    };
    true.into()
}

pub unsafe fn foreground_apps(needle: &str) -> Vec<path::PathBuf> {
    let mut foreground: Vec<path::PathBuf> = Vec::new();
    let mut active_windows: Vec<HWND> = Vec::new();
    APPLICATIONS.clear();

    let _ = EnumWindows(Some(enum_callback), None);
    for (hwnd, title) in APPLICATIONS.clone() {
        if IsWindowVisible(hwnd).as_bool() && !title.is_empty() {
            active_windows.push(hwnd);
        }
    };

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

        let module_filename = OsString::from_wide(&module_filename[..size as usize]);
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
    };
    foreground
}

pub unsafe fn get_icon(path: &str) -> RgbaImage {
    let mut shfi = SHFILEINFOW {
        hIcon: std::mem::size_of::<HICON>() as HICON,
        iIcon: 0,
        dwAttributes: 0,
        szDisplayName: [0; 260],
        szTypeName: [0; 80],
    };
    let path: Vec<u16> = OsStr::new(path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    SHGetFileInfoW(
        path.as_ptr(),
        0,
        &mut shfi,
        std::mem::size_of::<DWORD>() as u32,
        0x000000100,
    );
    let hicon = shfi.hIcon;
    let image = hicon_to_image(hicon);
    DestroyIcon(hicon);
    return image;
}

unsafe fn hicon_to_image(icon: HICON) -> RgbaImage {
    let bitmap_size_i32 = i32::try_from(mem::size_of::<BITMAP>()).unwrap();
    let biheader_size_u32 = u32::try_from(mem::size_of::<BITMAPINFOHEADER>()).unwrap();
    let mut info = ICONINFO {
        fIcon: 0,
        xHotspot: 0,
        yHotspot: 0,
        hbmMask: std::mem::size_of::<HBITMAP>() as HBITMAP,
        hbmColor: std::mem::size_of::<HBITMAP>() as HBITMAP,
    };
    GetIconInfo(icon, &mut info);
    DeleteObject(info.hbmMask as *mut VOID);
    let mut bitmap: MaybeUninit<BITMAP> = MaybeUninit::uninit();

    GetObjectW(
        info.hbmColor as *mut VOID,
        bitmap_size_i32,
        bitmap.as_mut_ptr() as *mut VOID,
    );

    let bitmap = bitmap.assume_init_ref();

    let width_u32 = u32::try_from(bitmap.bmWidth).unwrap();
    let height_u32 = u32::try_from(bitmap.bmHeight).unwrap();
    let width_usize = usize::try_from(bitmap.bmWidth).unwrap();
    let height_usize = usize::try_from(bitmap.bmHeight).unwrap();
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
        biCompression: BI_RGB,
        biSizeImage: 0,
        biXPelsPerMeter: 0,
        biYPelsPerMeter: 0,
        biClrUsed: 0,
        biClrImportant: 0,
    };

    let mut bmp: Vec<u8> = vec![0; buf_size];
    let _mr_right = GetBitmapBits(info.hbmColor, buf_size as i32, bmp.as_mut_ptr() as LPVOID);
    buf.set_len(bmp.capacity());
    ReleaseDC(windows::Win32::Foundation::HWND(0), dc);
    DeleteObject(info.hbmColor as *mut VOID);

    for chunk in bmp.chunks_exact_mut(4) {
        let [b, _, r, _] = chunk else { unreachable!() };
        mem::swap(b, r);
    }
    RgbaImage::from_vec(width_u32, height_u32, bmp).unwrap()
}
