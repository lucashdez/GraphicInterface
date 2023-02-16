use core::ffi::c_void;
use graphic_interface::win32::*;
use std::ptr::{
    null,
    null_mut,
};

unsafe extern "system" fn window_procedure(
    hwnd: *mut c_void,
    u_msg: u32,
    w_param: *const u16,
    l_param: isize,
) -> isize
{
    match u_msg
    {
        WM_NCCREATE =>
        {
            println!("NC Create");
            let createstruct: *mut CREATESTRUCTW = l_param as *mut _;
            if createstruct.is_null()
            {
                return 0;
            }
            let ptr: *mut i32 = (*createstruct).lp_create_params as *mut i32;
            return set_window_userdata::<i32>(hwnd, ptr).is_ok() as isize;
        }
        WM_CREATE =>
        {
            println!("Create")
        }
        WM_CLOSE =>
        {
            DestroyWindow(hwnd);
        }
        WM_DESTROY =>
        {
            match get_window_userdata::<i32>(hwnd)
            {
                Ok(ptr) if !ptr.is_null() =>
                {
                    drop(Box::from_raw(ptr));
                    println!("Cleaned up the box");
                }
                Ok(_) =>
                {
                    println!("Userdata ptr is null, no cleanup");
                }
                Err(e) =>
                {
                    println!("Error while getting the userdata ptr to clean it up: {}", e);
                }
            }
            PostQuitMessage(0);
        }

        WM_SETCURSOR =>
        {
            let h_instance = GetModuleHandleW(null());
            let cursor = LoadCursorW(h_instance, IDC_ARROW);
            let _old_cursor = SetCursor(cursor);
        }
        WM_PAINT =>
        {
            match get_window_userdata::<i32>(hwnd)
            {
                Ok(ptr) if !ptr.is_null() =>
                {
                    println!("Current ptr: {}", *ptr);
                    *ptr += 1;
                }
                Ok(_) =>
                {
                    println!("userdata ptr is null");
                }
                Err(e) =>
                {
                    println!("Error while getting the userdata ptr: {}", e);
                }
            }
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut ps);
            let _success = FillRect(hdc, &ps.rc_paint, (5 + 1) as *mut c_void);
            EndPaint(hwnd, &ps);
        }
        _ => return DefWindowProcW(hwnd, u_msg, w_param, l_param),
    }
    0
}

fn main()
{
    let h_instance = get_process_handle();
    let mut window_class: WNDCLASSW = WNDCLASSW::default();
    let window_class_name = wide_null("New Window Class");
    window_class.lpfn_wnd_proc = Some(window_procedure);
    window_class.h_instance = h_instance;
    window_class.lpsz_class_name = window_class_name.as_ptr();
    window_class.h_cursor = load_predefined_cursor(IDCursor::Arrow).unwrap();

    unsafe { register_class(&window_class) }.unwrap_or_else(|error| {
        panic!("Could not register class, error code: {}", error);
    });

    //  Styles
    let window_name = wide_null("New Window");
    let lparam: *mut i32 = Box::leak(Box::new(5_i32));
    let window_handle =
        create_window_app("New Window Class", "New Window", None, None, lparam)
            .unwrap_or_else(|error| {
                panic!("Could not create window, error code: {}", error);
            });

    loop
    {
        match get_any_message()
        {
            Ok(msg) =>
            {
                if msg.message == WM_QUIT
                {
                    break;
                }
                translate_message(&msg);
                unsafe { DispatchMessageW(&msg) };
            }
            Err(e) =>
            {
                panic!("Error when getting from the message queue: {}", e);
            }
        }
    }
}
