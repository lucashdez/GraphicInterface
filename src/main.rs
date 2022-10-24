use windows_sys::Win32::Graphics::Gdi::*;
use windows_sys::Win32::System::Diagnostics::Debug::*;
use windows_sys::Win32::System::LibraryLoader::*;
use windows_sys::Win32::UI::WindowsAndMessaging::*;
use windows_sys::Win32::Foundation::*;
use windows_sys::core::*;

macro_rules! zero {
    () => {
        unsafe {core::mem::zeroed()}
    };
}


fn win32_resize_dib_section(width: i32, height: i32) {
  // TODO: Finalizar los bitmaps para los colores.
  let bitmap_info: BITMAPINFO = zero!();
  bitmap_info.bmiHeader.biSize = std::mem::size_of::<BITMAPINFO>() as u32;
  bitmap_info.bmiHeader.biWidth = width;
  bitmap_info.bmiHeader.biHeight = height;
  let bitmap_memory: *mut std::os::raw::c_void;
  let bitmap_handle: HBITMAP = unsafe { CreateDIBSection(
    device_context, &bitmap_info, 
    DIB_RGB_COLORS, 
    &mut bitmap_memory, 
    0, 0)
  };
}

fn win32_update_window(device_context: isize, x: i32, y: i32, width: i32, height: i32) {
  unsafe { StretchDIBits(
    device_context, 
    x, 
    y, 
    width, 
    height, 
    x, 
    y, 
    width, 
    height, 
    lpbits, 
    lpbmi, 
    DIB_RGB_COLORS, 
    SRCCOPY);
  }
}

extern "system"
fn win32_window_proc(window: isize,
               message: u32,
               wparam: usize,
               lparam: isize,) -> isize {
  let mut result = 0;
  match message {
    WM_SIZE => {
      let mut client_rect:RECT = zero!();
      unsafe { GetClientRect(window, &mut client_rect) };
      let width: i32 = client_rect.right - client_rect.left;
      let height: i32 = client_rect.bottom - client_rect.top;
      win32_resize_dib_section(width, height);
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
      let device_context:isize = unsafe { BeginPaint(window, &mut paint) };
      let x = paint.rcPaint.left;
      let y = paint.rcPaint.top;
      let height = paint.rcPaint.bottom - paint.rcPaint.top;
      let width = paint.rcPaint.right - paint.rcPaint.left;
      let operation: u32 = WHITENESS;
      unsafe { PatBlt(device_context, x, y, width, height, operation); }
      unsafe { EndPaint(window, &paint) };

    },

    _ => {
      result = unsafe {DefWindowProcA(window, message, wparam, lparam)};
    }
  }
  return result;
}



fn main() {
  let instance: isize = unsafe { GetModuleHandleA(std::ptr::null()) };
  let mut window_class:WNDCLASSA = zero!();
  window_class.style = CS_OWNDC|CS_HREDRAW|CS_VREDRAW;
  window_class.lpfnWndProc = Some(win32_window_proc);
  window_class.hInstance = instance;
  window_class.lpszClassName = s!("GraphicsInterfaceWindowClass");

  let register_result = unsafe { RegisterClassA(&window_class) };
  if register_result > 0 {
    let window_handle = unsafe { 
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
          unsafe { OutputDebugStringA(s!("Quiting window")) };
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
