#include "Windows.h"
#include "stdint.h"

#define internal static
#define global_variable static
#define local_variable static


typedef uint8_t u8;
typedef uint16_t u16;
typedef uint32_t u32;
typedef uint64_t u64;

typedef int8_t i8;
typedef int16_t i16;
typedef int32_t i32;
typedef int64_t i64;

global_variable bool Running;

// TODO: Optimizations
global_variable BITMAPINFO BitmapInfo;
global_variable void *BitmapMemory;
global_variable int BitmapWidth;
global_variable int BitmapHeight;

internal void 
RenderWeirdGradient(int XOffset, int YOffset)
{
  int BytesPerPixel = 4;
  int Pitch = BitmapWidth*BytesPerPixel;
  u8 *Row = (u8 *)BitmapMemory;
  for (int Y = 0;
       Y < BitmapHeight;
       ++Y) 
  {
    u32 *Pixel = (u32 *)Row;
    for(int X = 0;
        X < BitmapWidth;
        ++X) 
    {
      u8 Blue = (X + XOffset);
      u8 Green = (Y + YOffset);
      *Pixel++ = ((Green << 8) | Blue);
    }
    Row += Pitch;
  }
}

internal void
Win32ResizeDIBSection(int Width, int Height) 
{
  
  if (BitmapMemory) 
  {
    VirtualFree(BitmapMemory, 0, MEM_RELEASE);
  }
  
  BitmapWidth = Width;
  BitmapHeight = Height;
  BitmapInfo.bmiHeader.biSize = sizeof(BitmapInfo.bmiHeader);
  BitmapInfo.bmiHeader.biWidth = Width;
  BitmapInfo.bmiHeader.biHeight = Height;
  BitmapInfo.bmiHeader.biPlanes = 1;
  BitmapInfo.bmiHeader.biBitCount = 32;
  BitmapInfo.bmiHeader.biCompression = BI_RGB;
  int BytesPerPixel = 4;
  int BitmapMemorySize = (Width*Height)*BytesPerPixel;
  BitmapMemory = VirtualAlloc(0, BitmapMemorySize, MEM_COMMIT, PAGE_READWRITE);
  
  RenderWeirdGradient(128,0);
  
}

// {{{Win32UpdateWindow
internal void
Win32UpdateWindow(HDC DeviceContext, RECT WindowRect,int X, int Y, int Width, int Height) 
{
  int WindowWidth = WindowRect.right - WindowRect.left;
  int WindowHeight = WindowRect.bottom - WindowRect.top;
  StretchDIBits(DeviceContext, 
                /*X, Y, Width, Height, 
                X, Y, Width, Height, */
                0, 0, BitmapWidth, BitmapHeight,
                0, 0, WindowWidth, WindowHeight,
                BitmapMemory, 
                &BitmapInfo, 
                DIB_RGB_COLORS, 
                SRCCOPY);
}
//}}}

//{{{Win32MainWindowCallback
LRESULT CALLBACK 
Win32MainWindowCallback(HWND Window,
                        UINT Message,
                        WPARAM wParam,
                        LPARAM lParam) 
{
  LRESULT Result = 0;
  switch(Message) 
  {
    case WM_SIZE:
    {
      RECT ClientRect;
      GetClientRect(Window, &ClientRect);
      int Width = ClientRect.right - ClientRect.left;
      int Height = ClientRect.bottom - ClientRect.top;
      Win32ResizeDIBSection(Width, Height);
    } break;
    
    case WM_DESTROY:
    {
      Running = false;
    } break;
    
    case WM_CLOSE:
    {
      Running = false;
    } break;
    
    case WM_ACTIVATEAPP:
    {
      
    } break;
    
    case WM_PAINT:
    {
      PAINTSTRUCT Paint;
      HDC DeviceContext = BeginPaint(Window, &Paint);
      int X = Paint.rcPaint.left;
      int Y = Paint.rcPaint.top;
      int Width = Paint.rcPaint.right - Paint.rcPaint.left;
      int Height = Paint.rcPaint.bottom - Paint.rcPaint.top;
      RECT WindowRect;
      GetClientRect(Window, &WindowRect);
      Win32UpdateWindow(DeviceContext, WindowRect, X, Y, Width, Height);
      EndPaint(Window, &Paint);
    } break;
    
    default:
    {
      Result = DefWindowProcA(Window, Message, wParam, lParam);
    } break;
  }
  return(Result);
}

//}}}

int CALLBACK
WinMain(HINSTANCE Instance, 
        HINSTANCE hPrevInstance, 
        LPSTR lpCmdLine, 
        int nShowCmd) 
{
  WNDCLASSA WindowClass = {};
  WindowClass.style = CS_HREDRAW|CS_VREDRAW;
  WindowClass.lpfnWndProc = Win32MainWindowCallback;
  WindowClass.hInstance = Instance;
  WindowClass.lpszClassName = "GraphicInterfaceWindowClass";
  if (RegisterClass(&WindowClass)) 
  {
    HWND Window = CreateWindowEx(
                                 0,
                                 WindowClass.lpszClassName,
                                 "Graphic Interface",
                                 WS_OVERLAPPEDWINDOW|WS_VISIBLE,
                                 CW_USEDEFAULT,
                                 CW_USEDEFAULT,
                                 CW_USEDEFAULT,
                                 CW_USEDEFAULT,
                                 0,
                                 0,
                                 Instance,
                                 0);
    if (Window) 
    {
      MSG Message;
      Running = true;
      int XOffset = 0;
      int YOffset = 0;
      while (Running) 
      {
        
        while (PeekMessage(&Message, 0, 0, 0, PM_REMOVE) ) 
        {
          if(Message.message == WM_QUIT) 
          {
            Running = false;
          }
          TranslateMessage(&Message);
          DispatchMessage(&Message);
        };
        
        RenderWeirdGradient(XOffset, YOffset);
        ++XOffset;
        HDC DeviceContext = GetDC(Window);
        RECT ClientRect;
        GetClientRect(Window, &ClientRect);
        int WindowWidth = ClientRect.right - ClientRect.left;
        int WindowHeight = ClientRect.bottom - ClientRect.top;
        Win32UpdateWindow(DeviceContext, ClientRect, 0, 0, WindowWidth, WindowHeight);
        ReleaseDC(Window, DeviceContext);
      }
    } else 
    {
      // TODO : LOGGIN
    }
  } else 
  {
    // TODO : LOGGIN
  }
  
  
  return(0);
}
