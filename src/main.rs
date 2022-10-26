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

fn win32_resize_dib_section(
  bitmap_memory: &mut *mut std::os::raw::c_void,
  bitmap_info: &mut BITMAPINFO, 
  width: i32, height: i32
) -> () {
  
  // TODO: Finalizar los bitmaps para los colores.
  // TODO : Hacerlo safe
  bitmap_info.bmiHeader.biSize = std::mem::size_of::<BITMAPINFO>() as u32;
  bitmap_info.bmiHeader.biWidth = width;
  bitmap_info.bmiHeader.biHeight = height;
  bitmap_info.bmiHeader.biPlanes = 1;
  bitmap_info.bmiHeader.biBitCount = 32;
  bitmap_info.bmiHeader.biCompression = BI_RGB;
  const BYTES_PER_PIXEL: i32 = 4;
  let bitmap_memory_size: usize = ((width * height) * BYTES_PER_PIXEL) as usize;
  dbg!(bitmap_memory_size);
  unsafe { 
      *bitmap_memory = VirtualAlloc(
      *bitmap_memory, 
      bitmap_memory_size, 
      MEM_COMMIT, 
      PAGE_READWRITE
    )
  };

}

// {{{WIN32_UPDATE_WINDOW
fn win32_update_window(
  bitmap_memory: &mut *mut std::os::raw::c_void,
  bitmap_info: &mut BITMAPINFO,
  device_context: isize, 
  x: i32, 
  y: i32, 
  width: i32, 
  height: i32
) -> () {
  unsafe { StretchDIBits(
    device_context, 
    x, y, width, height, 
    x, y, width, height, 
    *bitmap_memory, 
    bitmap_info, 
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
  let mut result = 0;
  let mut bitmap_device_context:isize = 0;
  match message {
    WM_SIZE => {
      let mut bitmap_info: BITMAPINFO = zero!();
      let mut client_rect: RECT = zero!();
      let mut bitmap_handle: isize = 0;
      let mut bitmap_memory: *mut std::os::raw::c_void = zero!();
      unsafe { GetClientRect(window, &mut client_rect) };
      let width: i32 = client_rect.right - client_rect.left;
      let height: i32 = client_rect.bottom - client_rect.top;
      win32_resize_dib_section(&mut bitmap_memory, &mut bitmap_info, width, height);
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
      let paint_device_context = unsafe { BeginPaint(window, &mut paint) };
      let x = paint.rcPaint.left;
      let y = paint.rcPaint.top;
      let height = paint.rcPaint.bottom - paint.rcPaint.top;
      let width = paint.rcPaint.right - paint.rcPaint.left;
      let operation: u32 = WHITENESS;
      unsafe { PatBlt(paint_device_context, x, y, width, height, operation); }
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
