use core::ffi::c_void;
use std::ptr::null_mut;
#[link(name = "User32")]
#[link(name = "Kernel32")]
extern "system" {
    pub fn GetModuleHandleA(lpModuleName: *const u16) -> *mut c_void;
}

extern "system" {
    pub fn GetLastError() -> u32;
}

extern "system" {
    pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> u16;
}

extern "system" {
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
}
extern "system" {
    pub fn DefWindowProcW(
        hWnd: *mut c_void,
        Msg: u32,
        wParam: *const u16,
        lParam: isize,
    ) -> isize;
}

extern "system" {
    pub fn GetMessageW(
        lpMsg: &mut MSG,
        hWnd: *mut c_void,
        wMsgFilterMin: u32,
        wMsgFilterMax: u32,
    ) -> i32;
}

extern "system" {
    pub fn TranslateMessage(lpMsg: *const MSG) -> i32;
    pub fn DispatchMessageW(lpMsg: *const MSG) -> isize;
}

extern "system" {
    pub fn DestroyWindow(hwnd: *mut c_void) -> i32;
    pub fn PostQuitMessage(nExitCode: i32);
}

extern "system" {
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

type WNDPROC = Option<
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
    style: u32,
    lpfn_wnd_proc: WNDPROC,
    cb_cls_extra: i32,
    cb_wnd_extra: i32,
    h_instance: *mut c_void,
    h_icon: *mut c_void,
    h_cursor: *mut c_void,
    hbr_background: *mut c_void,
    lpsz_menu_name: *const u16,
    lpsz_class_name: *const u16,
}
unsafe_impl_default_zeroed!(WNDCLASSW);

#[repr(C)]
pub struct POINT
{
    x: i32,
    y: i32,
}
unsafe_impl_default_zeroed!(POINT);

#[repr(C)]
pub struct MSG
{
    hwnd: *mut c_void,
    message: u32,
    w_param: *const u16,
    l_param: isize,
    time: u32,
    pt: POINT,
    l_private: u32,
}
unsafe_impl_default_zeroed!(MSG);

#[repr(C)]
pub struct PAINTSTRUCT
{
    hdc: *mut c_void,
    f_erase: i32,
    rc_paint: RECT,
    f_restore: i32,
    f_inc_update: i32,
    rgv_reserved: [u8; 32],
}
unsafe_impl_default_zeroed!(PAINTSTRUCT);

#[repr(C)]
pub struct RECT
{
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}
unsafe_impl_default_zeroed!(RECT);

// NOTE : MESSAGE VALUES
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_DESTROY: u32 = 0x0002;
pub const WM_PAINT: u32 = 0x000F;

unsafe extern "system" fn window_procedure(
    hwnd: *mut c_void,
    u_msg: u32,
    w_param: *const u16,
    l_param: isize,
) -> isize
{
    match u_msg
    {
        WM_CLOSE =>
        {
            DestroyWindow(hwnd);
        }
        WM_DESTROY => PostQuitMessage(0),
        WM_PAINT =>
        {
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut ps);
            let _success =
                FillRect(hdc, &ps.rc_paint, (70 as u32) as *mut c_void);
            EndPaint(hwnd, &ps);
        }
        _ => return DefWindowProcW(hwnd, u_msg, w_param, l_param),
    }
    0
}

pub fn wide_null(s: &str) -> Vec<u16>
{
    s.encode_utf16().chain(Some(0)).collect()
}

fn main()
{
    let h_instance = unsafe { GetModuleHandleA(core::ptr::null()) };
    let mut window_class: WNDCLASSW = WNDCLASSW::default();
    let window_class_name = wide_null("New Window Class");
    window_class.lpfn_wnd_proc = Some(window_procedure);
    window_class.h_instance = h_instance;
    window_class.lpsz_class_name = window_class_name.as_ptr();

    let atom = unsafe { RegisterClassW(&window_class) };
    if atom == 0
    {
        let last_error = unsafe { GetLastError() };
        panic!("Could not register class, error code: {}", last_error);
    }

    //  Styles
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

    let window_name = wide_null("New Window");
    let window_handle = unsafe {
        CreateWindowExW(
            0,
            window_class_name.as_ptr(),
            window_name.as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            h_instance,
            core::ptr::null_mut(),
        )
    };
    if window_handle.is_null()
    {
        let last_error = unsafe { GetLastError() };
        panic!("Could not create window, error code: {}", last_error);
    }

    let mut msg = MSG::default();
    loop
    {
        let message_return = unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) };
        if message_return == 0
        {
            break;
        }
        else if message_return == -1
        {
            let last_error = unsafe { GetLastError() };
            panic!("Error with `GetMessageW`, error code: {}", last_error);
        }
        else
        {
            unsafe {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }
}
