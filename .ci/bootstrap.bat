@ECHO OFF

if not exist .ci (cd ..)
if not exist .ci exit /b 1

SETLOCAL ENABLEEXTENSIONS ENABLEDELAYEDEXPANSION

rem Check if the C++ compiler is there and skip downloading if it exists
if exist internal\c\c_compiler\bin\c++.exe goto skipccompsetup

rem Create the c_compiler directory that should contain the mingw binaries
mkdir internal\c\c_compiler

rem Check the processor type and then set the MINGW variable to correct mingw filename
rem reg Query "HKLM\Hardware\Description\System\CentralProcessor\0" | find /i "x86" > NUL && set MINGW=mingw32 || 

set MINGW=mingw64

rem Set the correct file to download based on processor type
rem if "%MINGW%"=="mingw64" (
	set url="https://github.com/niXman/mingw-builds-binaries/releases/download/12.2.0-rt_v10-rev0/x86_64-12.2.0-release-win32-seh-rt_v10-rev0.7z"
rem ) else (
rem   set url="https://github.com/niXman/mingw-builds-binaries/releases/download/12.2.0-rt_v10-rev0/i686-12.2.0-release-win32-sjlj-rt_v10-rev0.7z"
rem )

echo Downloading %url%...
curl -L %url% -o temp.7z

echo Downloading 7zr.exe...
curl -L https://www.7-zip.org/a/7zr.exe -o 7zr.exe

echo Extracting C++ Compiler...
7zr.exe x temp.7z -y

echo Moving C++ compiler...
for /f %%a in ('dir %MINGW% /b') do move /y "%MINGW%\%%a" internal\c\c_compiler\

echo Cleaning up..
rd %MINGW%
del 7zr.exe
del temp.7z

:skipccompsetup

echo Building library 'LibQB'
cd internal\c\libqb\os\win
call setup_build.bat
IF ERRORLEVEL 1 exit /b 1

cd ..\..\..\..\..

echo Building library 'FreeType'
cd internal\c\parts\video\font\ttf\os\win
call setup_build.bat
IF ERRORLEVEL 1 exit /b 1

cd ..\..\..\..\..\..\..\..

echo Building library 'Core:FreeGLUT'
cd internal\c\parts\core\os\win
call setup_build.bat
IF ERRORLEVEL 1 exit /b 1

cd ..\..\..\..\..\..

echo Bootstrapping QB64
copy internal\source\*.* internal\temp\ >nul
copy source\qbhd.ico internal\temp\ >nul
copy source\icon.rc internal\temp\ >nul
cd internal\c
c_compiler\bin\windres.exe -i ..\temp\icon.rc -o ..\temp\icon.o
c_compiler\bin\g++ -mconsole -s -Wfatal-errors -w -Wall qbx.cpp libqb\os\win\libqb_setup.o ..\temp\icon.o -D DEPENDENCY_LOADFONT  parts\video\font\ttf\os\win\src.o -D DEPENDENCY_SOCKETS -D DEPENDENCY_NO_PRINTER -D DEPENDENCY_ICON -D DEPENDENCY_NO_SCREENIMAGE parts\core\os\win\src.a -lopengl32 -lglu32   -mwindows -static-libgcc -static-libstdc++ -D GLEW_STATIC -D FREEGLUT_STATIC     -lws2_32 -lwinmm -lgdi32 -o "..\..\qb64_bootstrap.exe"
IF ERRORLEVEL 1 exit /b 1
