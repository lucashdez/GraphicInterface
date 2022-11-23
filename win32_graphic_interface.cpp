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

struct win32_offscreen_buffer
{
    BITMAPINFO Info;
    void *Memory;
    int Width;
    int Height;
    int BytesPerPixel;
    int Pitch;
};

struct win32_window_dimension
{
    int Width;
    int Height;
};

global_variable bool Running;
global_variable win32_offscreen_buffer GlobalBackBuffer;

//{{{ Win32GetWindowDimension
win32_window_dimension
Win32GetWindowDimension(HWND Window)
{
    win32_window_dimension Result;

    RECT ClientRect;
    GetClientRect(Window, &ClientRect);
    Result.Width = ClientRect.right - ClientRect.left;
    Result.Height = ClientRect.bottom - ClientRect.top;

    return (Result);
}
//}}}

// {{{ RenderWeirdGradient
internal void
RenderWeirdGradient(win32_offscreen_buffer Buffer, int XOffset, int YOffset)
{
    Buffer.Pitch = Buffer.Width * Buffer.BytesPerPixel;
    u8 *Row = (u8 *)Buffer.Memory;
    for (int Y = 0; Y < Buffer.Height; ++Y)
    {
        u32 *Pixel = (u32 *)Row;
        for (int X = 0; X < Buffer.Width; ++X)
        {
            u8 Blue = (X + XOffset);
            u8 Green = (Y + YOffset);
            *Pixel++ = ((Green << 8) | Blue);
        }
        Row += Buffer.Pitch;
    }
}
// }}}

// {{{ Win32ResizeDIBSection
internal void
Win32ResizeDIBSection(win32_offscreen_buffer *Buffer, int Width, int Height)
{

    if (Buffer->Memory)
    {
        VirtualFree(Buffer->Memory, 0, MEM_RELEASE);
    }

    Buffer->BytesPerPixel = 4;
    Buffer->Width = Width;
    Buffer->Height = Height;

    Buffer->Info.bmiHeader.biSize = sizeof(Buffer->Info.bmiHeader);
    Buffer->Info.bmiHeader.biWidth = Width;
    Buffer->Info.bmiHeader.biHeight = Height;
    Buffer->Info.bmiHeader.biPlanes = 1;
    Buffer->Info.bmiHeader.biBitCount = 32;
    Buffer->Info.bmiHeader.biCompression = BI_RGB;
    int BitmapMemorySize =
        (Buffer->Width * Buffer->Height) * Buffer->BytesPerPixel;
    Buffer->Memory =
        VirtualAlloc(0, BitmapMemorySize, MEM_COMMIT, PAGE_READWRITE);
}
// }}}

// {{{ Win32DisplayBufferInWindow
internal void
Win32DisplayBufferInWindow(HDC DeviceContext, int WindowWidth, int WindowHeight,
                           win32_offscreen_buffer Buffer, int X, int Y,
                           int Width, int Height)
{
    // TODO : Correct aspect ratio
    StretchDIBits(DeviceContext,
                  /*X, Y, Width, Height,
                  X, Y, Width, Height, */
                  0, 0, WindowWidth, WindowHeight, 0, 0, Buffer.Width,
                  Buffer.Height, Buffer.Memory, &Buffer.Info, DIB_RGB_COLORS,
                  SRCCOPY);
}
//}}}

//{{{ Win32MainWindowCallback
LRESULT CALLBACK
Win32MainWindowCallback(HWND Window, UINT Message, WPARAM wParam, LPARAM lParam)
{
    LRESULT Result = 0;
    switch (Message)
    {
    case WM_SIZE:
    {
        win32_window_dimension Dimension = Win32GetWindowDimension(Window);
        // NOTE : Assigned 720p
        Win32ResizeDIBSection(&GlobalBackBuffer, 1280, 720);
    }
    break;

    case WM_DESTROY:
    {
        Running = false;
    }
    break;

    case WM_CLOSE:
    {
        Running = false;
    }
    break;

    case WM_ACTIVATEAPP:
    {
    }
    break;

    case WM_PAINT:
    {
        PAINTSTRUCT Paint;
        HDC DeviceContext = BeginPaint(Window, &Paint);
        int X = Paint.rcPaint.left;
        int Y = Paint.rcPaint.top;
        int Width = Paint.rcPaint.right - Paint.rcPaint.left;
        int Height = Paint.rcPaint.bottom - Paint.rcPaint.top;
        win32_window_dimension Dimension = Win32GetWindowDimension(Window);
        Win32DisplayBufferInWindow(DeviceContext, Dimension.Width,
                                   Dimension.Height, GlobalBackBuffer, X, Y,
                                   Width, Height);
        EndPaint(Window, &Paint);
    }
    break;

    default:
    {
        Result = DefWindowProcA(Window, Message, wParam, lParam);
    }
    break;
    }
    return (Result);
}

//}}}

int CALLBACK
WinMain(HINSTANCE Instance, HINSTANCE hPrevInstance, LPSTR lpCmdLine,
        int nShowCmd)
{
    WNDCLASSA WindowClass = {};
    WindowClass.style = CS_HREDRAW | CS_VREDRAW;
    WindowClass.lpfnWndProc = Win32MainWindowCallback;
    WindowClass.hInstance = Instance;
    WindowClass.lpszClassName = "GraphicInterfaceWindowClass";
    if (RegisterClass(&WindowClass))
    {
        HWND Window = CreateWindowEx(
            0, WindowClass.lpszClassName, "Graphic Interface",
            WS_OVERLAPPEDWINDOW | WS_VISIBLE, CW_USEDEFAULT, CW_USEDEFAULT,
            CW_USEDEFAULT, CW_USEDEFAULT, 0, 0, Instance, 0);
        if (Window)
        {
            MSG Message;
            Running = true;
            int XOffset = 0;
            int YOffset = 0;
            while (Running)
            {
                while (PeekMessage(&Message, 0, 0, 0, PM_REMOVE))
                {
                    if (Message.message == WM_QUIT)
                    {
                        Running = false;
                    }
                    TranslateMessage(&Message);
                    DispatchMessage(&Message);
                };

                RenderWeirdGradient(GlobalBackBuffer, XOffset, YOffset);
                ++XOffset;
                HDC DeviceContext = GetDC(Window);
                win32_window_dimension Dimension =
                    Win32GetWindowDimension(Window);
                Win32DisplayBufferInWindow(
                    DeviceContext, Dimension.Width, Dimension.Height,
                    GlobalBackBuffer, 0, 0, Dimension.Width, Dimension.Height);
                ReleaseDC(Window, DeviceContext);
            }
        }
        else
        {
            // TODO : LOGGIN
        }
    }
    else
    {
        // TODO : LOGGIN
    }
    return (0);
}
