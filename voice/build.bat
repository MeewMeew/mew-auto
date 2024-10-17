@echo off
REM Run build command
pyinstaller --clean --onefile --icon=icon.ico --upx-dir="../bin/" --collect-all=vosk --name=voice main.py