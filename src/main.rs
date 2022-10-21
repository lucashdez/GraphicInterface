use windows_sys::Win32::Graphics::Gdi::*;
use windows_sys::Win32::System::Diagnostics::Debug::*;
use windows_sys::Win32::System::LibraryLoader::*;
use windows_sys::Win32::UI::WindowsAndMessaging::*;
use windows_sys::core::*;

macro_rules! zero {
    () => {
        unsafe {core::mem::zeroed()}
    };
}

extern "system"
fn window_proc(window: isize,
               message: u32,
               wparam: usize,
               lparam: isize,) -> isize {
  let mut result = 0;
  match message {
    WM_SIZE => {
      unsafe { OutputDebugStringW(w!("WM_SIZE")) };
    },

    WM_DESTROY => {
      unsafe { OutputDebugStringW(w!("WM_DESTROY")) };
    },

    WM_CLOSE => {
      unsafe { OutputDebugStringW(w!("WM_CLOSE")) };
    },

    WM_ACTIVATEAPP => {
      unsafe { OutputDebugStringW(w!("WM_ACTIVATEAPP")) };
    },

    WM_PAINT => {
      let mut paint:PAINTSTRUCT = zero!();
      let device_context:isize = unsafe { BeginPaint(window, &mut paint) };
      let x = paint.rcPaint.left;
      let y = paint.rcPaint.top;
      let height = paint.rcPaint.bottom - paint.rcPaint.top;
      let width = paint.rcPaint.right - paint.rcPaint.left;
      let mut operation;

      unsafe { PatBlt(device_context, x, y, width, height, operation); }

      unsafe { EndPaint(window, &paint) };

    },

    _ => {
      result = unsafe {DefWindowProcW(window, message, wparam, lparam)};
    }
  }
  return result;
}



fn main() {
  let instance: isize = unsafe { GetModuleHandleW(std::ptr::null()) };
  let mut window_class:WNDCLASSW = zero!();
  window_class.style = CS_OWNDC|CS_HREDRAW|CS_VREDRAW;
  window_class.lpfnWndProc = Some(window_proc);
  window_class.hInstance = instance;
  window_class.lpszClassName = w!("GraphicsInterfaceWindowClass");

  let register_result = unsafe { RegisterClassW(&window_class) };
  if register_result > 0 {
    let window_handle = unsafe { 
      CreateWindowExW(
        0, 
        window_class.lpszClassName, 
        w!("Graphics Interface"), 
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
        let message_result = unsafe { GetMessageW(&mut message, 0, 0, 0) };
        if message_result == -1 {
          panic!("NO MESSAGES FOUND AAAAAAAAA");
        } else if message_result == 0 {
          panic!("SOMETHING");
        } else {
          unsafe {
            TranslateMessage(&message);
            DispatchMessageW(&message);
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
