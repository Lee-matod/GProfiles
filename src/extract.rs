// I have no clue how this works, but it does!

use core::mem::MaybeUninit;
use std::ffi::OsStr;
use std::mem;
use std::os::windows::ffi::OsStrExt;

use image::RgbaImage;
use winapi::shared::minwindef::{DWORD, LPVOID};
use winapi::shared::windef::{HBITMAP, HICON};
use winapi::um::shellapi::{SHGetFileInfoW, SHFILEINFOW};
use winapi::um::wingdi::{
    DeleteObject, GetBitmapBits, GetObjectW, BITMAP, BITMAPINFOHEADER, BI_RGB,
};
use winapi::um::winnt::VOID;
use winapi::um::winuser::{DestroyIcon, GetIconInfo, ICONINFO};
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::{GetDC, ReleaseDC};

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
