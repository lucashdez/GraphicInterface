use std::borrow::Borrow;

use windows_sys::Win32::Graphics::Gdi::*;
use windows_sys::Win32::System::Diagnostics::Debug::*;
use windows_sys::Win32::System::LibraryLoader::*;
use windows_sys::Win32::UI::WindowsAndMessaging::*;
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::System::Memory::*;
use windows_sys::core::*;

macro_rules! zero {
    () => {
        unsafe {core::mem::zeroed()}
    }
}

static mut BITMAP_MEMORY: *mut std::os::raw::c_void = core::ptr::null_mut();
static mut BITMAP_INFO: *mut BITMAPINFO = core::ptr::null_mut();

//{{{ WIN32_RESIZE_DIB_SECTION
fn win32_resize_dib_section(
  bitmap_memory: &mut *mut std::os::raw::c_void,
  bitmap_info: &mut *mut BITMAPINFO, 
  bitmap_width: i32, bitmap_height: i32
) -> () {
  if bitmap_info.is_null() {
    unsafe { 
      *bitmap_info = VirtualAlloc(
        std::ptr::null(), 
        core::mem::size_of::<BITMAPINFO>(), 
        MEM_COMMIT, 
        PAGE_READWRITE
      ) as *mut BITMAPINFO
    };
  }
  println!("{}",unsafe { (**bitmap_info).bmiHeader.biSize });
  unsafe {
    (**bitmap_info).bmiHeader.biSize = std::mem::size_of::<BITMAPINFO>() as u32;
    (**bitmap_info).bmiHeader.biWidth = bitmap_width;
    (**bitmap_info).bmiHeader.biHeight = -bitmap_height;
    (**bitmap_info).bmiHeader.biPlanes = 1;
    (**bitmap_info).bmiHeader.biBitCount = 32;
    (**bitmap_info).bmiHeader.biCompression = BI_RGB;
  }
  dbg!(bitmap_info);
  const BYTES_PER_PIXEL: i32 = 4;
  let bitmap_memory_size: usize = ((bitmap_width * bitmap_height) * BYTES_PER_PIXEL) as usize;
  if !bitmap_memory.is_null() {
    unsafe { VirtualFree(*bitmap_memory, 0, MEM_RELEASE) };
  } else {
    unsafe { 
      *bitmap_memory = VirtualAlloc(
        std::ptr::null(), 
        bitmap_memory_size, 
        MEM_COMMIT, 
        PAGE_READWRITE
      )
    };
  }

  


  dbg!(bitmap_memory);
}
//}}}

// {{{WIN32_UPDATE_WINDOW
fn win32_update_window(
  bitmap_memory: &mut *mut std::os::raw::c_void,
  bitmap_info: &mut *mut BITMAPINFO,
  device_context: isize, 
  window_rect: &RECT,
  width: i32, 
  height: i32
) -> () {
  let bitmap_width = width;
  let bitmap_height = height;
  let window_width = window_rect.right - window_rect.left;
  let window_height = window_rect.top - window_rect.bottom;
  unsafe { StretchDIBits(
    device_context, 
    0, 0, bitmap_width, bitmap_height,
    0, 0, window_width, window_height,
    *bitmap_memory, 
    *bitmap_info, 
    DIB_RGB_COLORS, 
    SRCCOPY);
  }
}
// }}}

//{{{ WIN32_PROC: MSG HANDLER
extern "system"
fn win32_window_proc(
  window: isize,
  message: u32,
  wparam: usize,
  lparam: isize,
) -> isize {
  //TODO : For now
  let mut result = 0;
  match message {
    WM_SIZE => {
      let mut client_rect: RECT = zero!();
      unsafe { GetClientRect(window, &mut client_rect) };
      let width: i32 = client_rect.right - client_rect.left;
      let height: i32 = client_rect.bottom - client_rect.top;
      unsafe { win32_resize_dib_section(&mut BITMAP_MEMORY, &mut BITMAP_INFO, width, height) }
      unsafe { OutputDebugStringA(s!("WM_SIZE")) };
    },

    WM_DESTROY => {
      // TODO : Handle this as an error
      unsafe { OutputDebugStringA(s!("WM_DESTROY")) };
    },

    WM_CLOSE => {
      // TODO : Some message to the user
      unsafe { PostQuitMessage(0) };
      unsafe { OutputDebugStringA(s!("WM_CLOSE")) };
    },

    WM_ACTIVATEAPP => {
      unsafe { OutputDebugStringA(s!("WM_ACTIVATEAPP")) };
    },

    WM_PAINT => {
      let mut paint:PAINTSTRUCT = zero!();
      let device_context = unsafe { BeginPaint(window, &mut paint) };
      let height = paint.rcPaint.bottom - paint.rcPaint.top;
      let width = paint.rcPaint.right - paint.rcPaint.left;
      let mut client_rect: RECT = zero!();
      unsafe { GetClientRect(window, &mut client_rect) };
      unsafe { dbg!(BITMAP_MEMORY); }
      win32_update_window(
        unsafe { &mut BITMAP_MEMORY }, 
        unsafe { &mut BITMAP_INFO},
        device_context, 
        &client_rect, 
        width, 
        height
      );
      unsafe { EndPaint(window, &paint) };

    },

    _ => {
      result = unsafe {DefWindowProcA(window, message, wparam, lparam)};
    }
  }
  return result;
}
//}}}


fn main() {
  let instance: isize = unsafe { GetModuleHandleA(std::ptr::null()) };
  let mut window_class:WNDCLASSA = zero!();
  window_class.style = CS_OWNDC|CS_HREDRAW|CS_VREDRAW;
  window_class.lpfnWndProc = Some(win32_window_proc);
  window_class.hInstance = instance;
  window_class.lpszClassName = s!("GraphicsInterfaceWindowClass");

  let register_result = unsafe { RegisterClassA(&window_class) };
  if register_result > 0 {
    let window_handle: isize = unsafe { 
      CreateWindowExA(
        0, 
        window_class.lpszClassName, 
        s!("Graphics Interface"), 
        WS_OVERLAPPEDWINDOW|WS_VISIBLE, 
        CW_USEDEFAULT, 
        CW_USEDEFAULT, 
        CW_USEDEFAULT, 
        CW_USEDEFAULT, 
        0, 
        0, 
        instance, 
        std::ptr::null()) };
    if window_handle > 0 {
      let mut message: MSG = zero!();
      
      loop {
        let message_result = unsafe { GetMessageA(&mut message, 0, 0, 0) };
        if message_result == -1 {
          panic!("NO MESSAGES FOUND AAAAAAAAA");
        } else if message_result == 0 {
          dbg!(message_result);
          break;
        } else {
          unsafe {
            TranslateMessage(&message);
            DispatchMessageA(&message);
          }
        }
      }
    } else {
      // TODO: window handle error logging
    } 
  } else {
    // TODO: Handle register_result error;
  }
}
