main: win32_graphic_interface.cpp
	pushd build & cl -Zi ../win32_graphic_interface.cpp user32.lib gdi32.lib & popd
run: 
	./build/win32_graphic_interface.exe
