use core::ffi::c_void;
use std::ptr::{
    null,
    null_mut,
};
#[link(name = "User32")]
#[link(name = "Kernel32")]
extern "system" {
    pub fn GetModuleHandleW(lpModuleName: *const u16) -> *mut c_void;
    pub fn GetLastError() -> u32;
    pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> u16;
    pub fn CreateWindowExW(
        dwExStyle: u32,
        lpClassName: *const u16,
        lpWindowName: *const u16,
        dwStyle: u32,
        X: i32,
        Y: i32,
        nWidth: i32,
        nHeight: i32,
        hWndParent: *mut c_void,
        hMenu: *mut c_void,
        hInstance: *mut c_void,
        lpParam: *mut c_void,
    ) -> *mut c_void;

    pub fn SetWindowLongPtrW(
        hwnd: *mut c_void,
        nIndex: i32,
        dwNewLong: isize,
    ) -> isize;

    pub fn GetWindowLongPtrW(hwnd: *mut c_void, nIndex: i32) -> isize;
    pub fn DefWindowProcW(
        hWnd: *mut c_void,
        Msg: u32,
        wParam: *const u16,
        lParam: isize,
    ) -> isize;

    pub fn GetMessageW(
        lpMsg: &mut MSG,
        hWnd: *mut c_void,
        wMsgFilterMin: u32,
        wMsgFilterMax: u32,
    ) -> i32;

    pub fn MessageBoxW(
        hwnd: *mut c_void,
        lpText: *const u16,
        lpCaption: *const u16,
        uType: u32,
    ) -> i32;
    pub fn TranslateMessage(lpMsg: *const MSG) -> i32;
    pub fn DispatchMessageW(lpMsg: *const MSG) -> isize;

    pub fn LoadCursorW(
        hInstance: *mut c_void,
        lpCursorName: *const u16,
    ) -> *mut c_void;

    pub fn SetCursor(hCursor: *mut c_void) -> *mut c_void;
    pub fn DestroyWindow(hwnd: *mut c_void) -> i32;
    pub fn PostQuitMessage(nExitCode: i32);
    pub fn BeginPaint(
        hWnd: *mut c_void,
        lpPaint: *const PAINTSTRUCT,
    ) -> *mut c_void;
    pub fn FillRect(
        hdc: *mut c_void,
        lprc: *const RECT,
        hbr: *mut c_void,
    ) -> i32;
    pub fn EndPaint(hwnd: *mut c_void, lpPaint: *const PAINTSTRUCT) -> i32;
    pub fn FormatMessageW(
        dwFlags: u32,
        lpSource: *mut c_void,
        dwMessageId: u32,
        dwLanguageId: u32,
        lpBuffer: *mut u16,
        nSize: u32,
        Arguments: *mut std::ffi::c_char,
    ) -> u32;

    pub fn LocalFree(hMem: *mut c_void) -> *mut c_void;
    pub fn SetLastError(dwErrCode: u32);

}

macro_rules! unsafe_impl_default_zeroed {
    ($t:ty) => {
        impl Default for $t
        {
            #[inline]
            #[must_use]
            fn default() -> Self
            {
                unsafe { core::mem::zeroed() }
            }
        }
    };
}

pub type WNDPROC = Option<
    unsafe extern "system" fn(
        hwnd: *mut c_void,
        uMsg: u32,
        wParam: *const u16,
        lParam: isize,
    ) -> isize,
>;

#[repr(C)]
pub struct WNDCLASSW
{
    pub style: u32,
    pub lpfn_wnd_proc: WNDPROC,
    pub cb_cls_extra: i32,
    pub cb_wnd_extra: i32,
    pub h_instance: *mut c_void,
    pub h_icon: *mut c_void,
    pub h_cursor: *mut c_void,
    pub hbr_background: *mut c_void,
    pub lpsz_menu_name: *const u16,
    pub lpsz_class_name: *const u16,
}

unsafe_impl_default_zeroed!(WNDCLASSW);

#[repr(C)]
pub struct POINT
{
    pub x: i32,
    pub y: i32,
}
unsafe_impl_default_zeroed!(POINT);

#[repr(C)]
pub struct MSG
{
    pub hwnd: *mut c_void,
    pub message: u32,
    pub w_param: *const u16,
    pub l_param: isize,
    pub time: u32,
    pub pt: POINT,
    pub l_private: u32,
}
unsafe_impl_default_zeroed!(MSG);

#[repr(C)]
pub struct PAINTSTRUCT
{
    pub hdc: *mut c_void,
    pub f_erase: i32,
    pub rc_paint: RECT,
    pub f_restore: i32,
    pub f_inc_update: i32,
    pub rgv_reserved: [u8; 32],
}
unsafe_impl_default_zeroed!(PAINTSTRUCT);

#[repr(C)]
pub struct RECT
{
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
unsafe_impl_default_zeroed!(RECT);

#[repr(C)]
pub struct CREATESTRUCTW
{
    pub lp_create_params: *mut c_void,
    pub h_instance: *mut c_void,
    pub h_menu: u32,
    pub hwnd_parent: *mut c_void,
    pub cy: i32,
    pub cx: i32,
    pub y: i32,
    pub x: i32,
    pub style: i32,
    pub lpsz_name: *const u16,
    pub lpsz_class: *const u16,
    pub dw_ex_style: u32,
}
unsafe_impl_default_zeroed!(CREATESTRUCTW);

pub const fn make_int_resource_w(i: u16) -> *const u16
{
    i as i32 as *const u16
}

pub const IDC_ARROW: *const u16 = make_int_resource_w(32512);

// NOTE : MESSAGE VALUES
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_DESTROY: u32 = 0x0002;
pub const WM_PAINT: u32 = 0x000F;
pub const WM_NCCREATE: u32 = 0x0081;
pub const WM_CREATE: u32 = 0x0001;
pub const WM_SETCURSOR: u32 = 0x0020;
pub const WM_QUIT: u32 = 0x0012;
pub const GWLP_USERDATA: i32 = -21;

pub fn wide_null(s: &str) -> Vec<u16>
{
    s.encode_utf16().chain(Some(0)).collect()
}

pub fn get_process_handle() -> *mut c_void
{
    //SAFETY: It always gives us the handler to the process.
    unsafe { GetModuleHandleW(null()) }
}

pub enum IDCursor
{
    AppStarting = 32650,
    Arrow = 32512,
    Cross = 32515,
    Hand = 32649,
    Help = 32651,
    IBeam = 32513,
    No = 32648,
    SizeAll = 32646,
    SizeNeSw = 32643,
    SizeNS = 32645,
    SizeNWSE = 32642,
    SizeWE = 32644,
    UpArrow = 32516,
    Wait = 32514,
}

pub fn load_predefined_cursor(
    cursor: IDCursor,
) -> Result<*mut c_void, Win32Error>
{
    let h_cursor =
        unsafe { LoadCursorW(null_mut(), make_int_resource_w(cursor as u16)) };
    if h_cursor.is_null()
    {
        Err(get_last_error())
    }
    else
    {
        Ok(h_cursor)
    }
}

pub unsafe fn register_class(
    window_class: &WNDCLASSW,
) -> Result<u16, Win32Error>
{
    let atom = RegisterClassW(window_class);
    if atom == 0
    {
        Err(get_last_error())
    }
    else
    {
        Ok(atom)
    }
}

pub fn get_last_error() -> Win32Error
{
    Win32Error(unsafe { GetLastError() })
}

const FORMAT_MESSAGE_ALLOCATE_BUFFER: u32 = 0x00000100;
const FORMAT_MESSAGE_FROM_SYSTEM: u32 = 0x00001000;
const FORMAT_MESSAGE_IGNORE_INSERTS: u32 = 0x00000200;
#[derive(Debug)]
#[repr(transparent)]
pub struct Win32Error(pub u32);
impl std::error::Error for Win32Error {}
impl core::fmt::Display for Win32Error
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> std::fmt::Result
    {
        if self.0 & (1 << 29) > 0
        {
            return write!(f, "Win32ApplicationError({})", self.0);
        }

        let dw_flags: u32 = FORMAT_MESSAGE_ALLOCATE_BUFFER
            | FORMAT_MESSAGE_FROM_SYSTEM
            | FORMAT_MESSAGE_IGNORE_INSERTS;
        let lp_source: *mut c_void = null_mut();
        let dw_message_id: u32 = self.0;
        let dw_language_id: u32 = 0;
        let mut buffer: *mut u16 = null_mut();
        let lp_buffer: *mut u16 = &mut buffer as *mut *mut u16 as *mut u16;
        let n_size: u32 = 0;
        let arguments: *mut std::ffi::c_char = null_mut();
        let tchar_counting_excluding_null = unsafe {
            FormatMessageW(
                dw_flags,
                lp_source,
                dw_message_id,
                dw_language_id,
                lp_buffer,
                n_size,
                arguments,
            )
        };
        if tchar_counting_excluding_null == 0 || buffer.is_null()
        {
            return Err(core::fmt::Error);
        }
        else
        {
            struct OnDropLocalFree(*mut c_void);
            impl Drop for OnDropLocalFree
            {
                fn drop(&mut self)
                {
                    // SAFETY: Windows will handle the free for us
                    unsafe { LocalFree(self.0) };
                }
            }
            let _on_drop = OnDropLocalFree(buffer as *mut c_void);
            // SAFETY: We only get the parts when they're filled. If no chars,
            // this cannot execute.
            let buffer_slice: &[u16] = unsafe {
                core::slice::from_raw_parts(
                    buffer,
                    tchar_counting_excluding_null as usize,
                )
            };
            for decode_result in
                core::char::decode_utf16(buffer_slice.iter().copied())
            {
                match decode_result
                {
                    Ok('\r') | Ok('\n') => write!(f, "")?,
                    Ok(ch) => write!(f, "{}", ch)?,
                    Err(_) => write!(f, "⍝")?,
                }
            }
            Ok(())
        }
    }
}

pub unsafe fn create_window_ex_w(
    ex_style: u32,
    class_name: *const u16,
    window_name: *const u16,
    style: u32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    parent: *mut c_void,
    menu: *mut c_void,
    instance: *mut c_void,
    param: *mut c_void,
) -> Result<*mut c_void, Win32Error>
{
    let hwnd = CreateWindowExW(
        ex_style,
        class_name,
        window_name,
        style,
        x,
        y,
        width,
        height,
        parent,
        menu,
        instance,
        param,
    );
    if hwnd.is_null()
    {
        Err(get_last_error())
    }
    else
    {
        Ok(hwnd)
    }
}

const WS_OVERLAPPED: u32 = 0x00000000;
const WS_CAPTION: u32 = 0x00C00000;
const WS_SYSMENU: u32 = 0x00080000;
const WS_THICKFRAME: u32 = 0x00040000;
const WS_MINIMIZEBOX: u32 = 0x00020000;
const WS_MAXIMIZEBOX: u32 = 0x00010000;
const WS_VISIBLE: u32 = 0x10000000;
// Look at WS_THICKFRAME
const WS_OVERLAPPEDWINDOW: u32 = WS_OVERLAPPED
    | WS_CAPTION
    | WS_SYSMENU
    | WS_THICKFRAME
    | WS_MINIMIZEBOX
    | WS_MAXIMIZEBOX;
const CW_USEDEFAULT: i32 = 0x80000000_u32 as i32;

pub fn create_window_app(
    class_name: &str,
    window_name: &str,
    position: Option<[i32; 2]>,
    size: Option<[i32; 2]>,
    create_param: *mut i32,
) -> Result<*mut c_void, Win32Error>
{
    let real_pos = match position
    {
        Some(pos) => pos,
        None => [CW_USEDEFAULT; 2],
    };
    let real_size = match size
    {
        Some(sz) => sz,
        None => [CW_USEDEFAULT; 2],
    };

    let hwnd = unsafe {
        create_window_ex_w(
            0,
            wide_null(class_name).as_ptr(),
            wide_null(window_name).as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            real_pos[0],
            real_pos[1],
            real_size[0],
            real_size[1],
            null_mut(),
            null_mut(),
            get_process_handle(),
            create_param.cast(),
        )
    };
    if hwnd.is_ok()
    {
        Ok(hwnd.unwrap())
    }
    else
    {
        Err(get_last_error())
    }
}

#[inline(always)]
pub fn get_any_message() -> Result<MSG, Win32Error>
{
    let mut msg = MSG::default();
    let output = unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) };
    if output == -1
    {
        Err(get_last_error())
    }
    else
    {
        Ok(msg)
    }
}

pub fn translate_message(msg: &MSG) -> bool
{
    0 != unsafe { TranslateMessage(msg) }
}

pub fn set_last_error(e: Win32Error)
{
    unsafe { SetLastError(e.0) }
}

/**
 *  returns the previous userdata pointer
 */
pub unsafe fn set_window_userdata<T>(
    hwnd: *mut c_void,
    ptr: *mut T,
) -> Result<*mut T, Win32Error>
{
    set_last_error(Win32Error(0));
    let out = SetWindowLongPtrW(hwnd, GWLP_USERDATA, ptr as isize);
    if out == 0
    {
        let last_error = get_last_error();
        if last_error.0 != 0
        {
            Err(last_error)
        }
        else
        {
            Ok(out as *mut T)
        }
    }
    else
    {
        Ok(out as *mut T)
    }
}

pub unsafe fn get_window_userdata<T>(
    hwnd: *mut c_void,
) -> Result<*mut T, Win32Error>
{
    set_last_error(Win32Error(0));
    let out = GetWindowLongPtrW(hwnd, GWLP_USERDATA);
    if out == 0
    {
        let last_error = get_last_error();
        if last_error.0 != 0
        {
            Err(last_error)
        }
        else
        {
            Ok(out as *mut T)
        }
    }
    else
    {
        Ok(out as *mut T)
    }
}
