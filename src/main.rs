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
               msg: u32,
               wparam: usize,
               lparam: isize,) -> isize {
  let mut result = 0;

  return result;
}



fn main() {
  let h_instance: isize = unsafe { GetModuleHandleW(std::ptr::null()) };
  let mut window_class:WNDCLASSW = zero!();
  window_class.style = CS_OWNDC|CS_HREDRAW|CS_VREDRAW;
  window_class.lpfnWndProc = Some(window_proc);
  window_class.hInstance = h_instance;
  window_class.lpszClassName = w!("GraphicsInterfaceWindowClass");
}
