#[cfg(target_os = "windows")]
use std::ffi::OsString;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;
#[cfg(target_os = "windows")]
use winapi::shared::guiddef::GUID;
#[cfg(target_os = "windows")]
use winapi::um::shellapi::{self, Shell_NotifyIconW, NOTIFYICONDATAW};


#[cfg(not(target_os = "windows"))]
use libnotify_sys::{
    notify_init,
    notify_notification_new,
    notify_notification_show,
    g_object_unref,
    notify_uninit
};
#[cfg(not(target_os = "windows"))]
use std::ptr;
#[cfg(not(target_os = "windows"))]
use std::ffi::{c_void, CString};

type NtfyRes = Result<(), &'static str>;
const TEXT_LEN: usize = 256;
const TITLE_LEN: usize = 64;

#[cfg(target_os = "windows")]
fn convert_to_wchar_t(text: &str, len: usize) -> Vec<u16> {
    let text_bytes: Vec<u16> = OsString::from(text)
        .as_os_str()
        .encode_wide()
        .take(len)
        .collect();

    text_bytes
}

#[cfg(target_os = "windows")]
pub fn send_notification(text: &str, title: &str) -> NtfyRes{
    // Convert strings
    let info_vec = convert_to_wchar_t(text, TEXT_LEN);
    let info_title_vec = convert_to_wchar_t(title, TITLE_LEN);

    let mut info_buf = [0u16; TEXT_LEN];
    unsafe {
        std::ptr::copy_nonoverlapping(info_vec.as_ptr(), info_buf.as_mut_ptr(), info_vec.len().min(64))
    }

    let mut info_title_buf = [0u16; TITLE_LEN];
    unsafe {
        std::ptr::copy_nonoverlapping(info_title_vec.as_ptr(), info_title_buf.as_mut_ptr(), info_title_vec.len().min(64))
    }


    // Create guid so that notifications work several times in a row
    let guid = {
        let mut gen_guid: GUID = Default::default();
        unsafe {
            winapi::um::combaseapi::CoCreateGuid(&mut gen_guid);
        }
        gen_guid
    };

    let mut info_data = NOTIFYICONDATAW {
        cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
        uFlags: shellapi::NIF_INFO | shellapi::NIF_GUID,
        szInfo: info_buf,
        szInfoTitle: info_title_buf,
        dwInfoFlags: shellapi::NIIF_NONE,
        guidItem: guid,
        ..Default::default()
    };

    let success = unsafe { Shell_NotifyIconW(shellapi::NIM_ADD, &mut info_data) != 0 };

    match success {
        true => Ok(()),
        false => Err("Shell_NotifyIconW failed")
    }
}

#[cfg(not(target_os = "windows"))]
pub fn send_notification(text: &str, title: &str) -> NtfyRes {
    // Map strings to CStrings, which are 0-terminated
    let text_str = CString::new(text).unwrap();
    let title_str = CString::new(title).unwrap();

    unsafe {
        notify_init("desktop-nfty\0".as_ptr() as *const i8);
        let notif = notify_notification_new(
            title_str.as_ptr() as *const i8,
            text_str.as_ptr() as *const i8,
            "dialog-information\0".as_ptr() as *const i8);

        notify_notification_show(notif, ptr::null_mut());
        g_object_unref(notif as *mut c_void);
        notify_uninit();
    }

    Ok(())
}
