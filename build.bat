@echo off
set input=.\target\release\MewAuto_Old.exe
set output=.\target\release\MewAuto.exe

REM Copy .\voice\dist\voice.exe to .\bin\voice.exe
ECHO Copying voice.exe to bin...
copy .\voice\dist\voice.exe .\bin\voice.exe

REM Run cargo build before running this script.
ECHO Compiling MewAuto...
cargo build --release

REM Rename the old executable to MewAuto_Old.exe
ECHO Renaming MewAuto.exe to MewAuto_Old.exe...
ren .\target\release\MewAuto.exe MewAuto_Old.exe

REM Run UPX to compress the executable.
ECHO Compressing MewAuto_Old.exe...
.\bin\upx.exe --best --all-methods --all-filters --force -o %output% %input%