@echo off
set input=.\target\release\MewAuto.exe
set output=.\target\release\MewAuto_compress.exe

REM Run cargo build before running this script.
cargo build --release

REM Run UPX to compress the executable.
E:\Tools\upx-4.2.4-win64\upx.exe --best --all-methods --all-filters --force -o %output% %input%