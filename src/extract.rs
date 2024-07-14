use core::mem::MaybeUninit;
use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::{mem, path};

use image::RgbaImage;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
use windows::Win32::Graphics::Gdi::{
    DeleteObject, GetBitmapBits, GetDC, GetObjectW, ReleaseDC, BITMAP, BITMAPINFOHEADER, HBITMAP,
    HGDIOBJ,
};
use windows::Win32::System::ProcessStatus::GetModuleFileNameExW;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION};
use windows::Win32::UI::Shell::{SHGetFileInfoW, SHFILEINFOW};
use windows::Win32::UI::WindowsAndMessaging::{
    DestroyIcon, EnumWindows, GetIconInfo, GetWindowTextW, GetWindowThreadProcessId,
    IsWindowVisible, HICON, ICONINFO,
};

static mut APPLICATIONS: Vec<(HWND, String)> = Vec::new();

/// Safely tries to perform a canonicalization for a given path.
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

/// Retrieves the executable paths of all currently running foreground applications.
pub unsafe fn foreground_apps(needle: &str) -> Vec<path::PathBuf> {
    let mut foreground: Vec<path::PathBuf> = Vec::new();
    let mut active_windows: Vec<HWND> = Vec::new();
    APPLICATIONS.clear();

    let _ = EnumWindows(Some(enum_callback), None);
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
    }
    foreground
}

/// Retrieves the icon attached to an executable through the given executable path.
pub unsafe fn get_icon(path: &str) -> RgbaImage {
    let mut shfi = SHFILEINFOW {
        hIcon: HICON(0),
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
        windows::core::PCWSTR(path.as_ptr()),
        windows::Win32::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES(0),
        Some(&mut shfi),
        std::mem::size_of::<u32>() as u32,
        windows::Win32::UI::Shell::SHGFI_FLAGS(0x000000100),
    );
    let hicon = shfi.hIcon;
    let image = hicon_to_image(hicon);
    let _ = DestroyIcon(hicon);
    return image;
}

unsafe fn hicon_to_image(icon: HICON) -> RgbaImage {
    let biheader_size_u32 = u32::try_from(mem::size_of::<BITMAPINFOHEADER>()).unwrap();
    let mut info = ICONINFO {
        fIcon: BOOL(0),
        xHotspot: 0,
        yHotspot: 0,
        hbmMask: HBITMAP(0),
        hbmColor: HBITMAP(0),
    };
    let _ = GetIconInfo(icon, &mut info);
    DeleteObject(info.hbmMask);
    let mut bitmap: MaybeUninit<BITMAP> = MaybeUninit::uninit();

    GetObjectW(
        HGDIOBJ(info.hbmColor.0 as isize),
        mem::size_of::<BITMAP>() as i32,
        Some(bitmap.as_mut_ptr() as *mut std::ffi::c_void),
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
    RgbaImage::from_vec(width_u32, height_u32, bmp).unwrap()
}
