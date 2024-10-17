from vosk import Model, KaldiRecognizer, SetLogLevel
import pyaudio, os, json, sys, ctypes, winreg, asyncio

debug_mode = False
is_hey = False
model_path = "../res/vosk-en"
audio_path = "../res/audio"
ffplay_path = "./ffplay.exe"

if not os.path.exists(model_path):
  model_path = "vosk-en"
  
if not os.path.exists(audio_path):
  audio_path = "audio"
  
if not os.path.exists(ffplay_path):
  ffplay_path = "ffplay"

def read_registry_value():
  try:
    key_path = r"Software\MewAuto"
    value_name = "LISTEN_HEY_PC"
    with winreg.OpenKey(winreg.HKEY_LOCAL_MACHINE, key_path) as registry_key:
      value, regtype = winreg.QueryValueEx(registry_key, value_name)
      return int(value)
  except FileNotFoundError:
    return 0
  except Exception as e:
    print(f"Error: {e}")
    return 0

async def play_audio(file):
  if debug_mode:
    print(f"Playing audio: {file}")
  process = await asyncio.create_subprocess_shell(f"{ffplay_path} -v 0 -nodisp -autoexit ./{audio_path}/{file}")
  await process.wait()
  
async def execute_command(command_name):
  command = ""
  
  match command_name:
    case "restart":
      command = "shutdown /r /t 0"
    case "shutdown":
      command = "shutdown /s /t 0"
    case "mewauto":
      command = "taskkill /f /im mewauto.exe"
    case "voice":
      command = "taskkill /f /im voice.exe"
      
  if command == "":
    return
  
  if debug_mode:
    print(f"Command: {command}")
  process = await asyncio.create_subprocess_shell(command)
  await process.wait()

def turn_off_screen():
    ctypes.windll.user32.SendMessageW(0xFFFF, 0x0112, 0xF170, 2)

async def main():
  SetLogLevel(-1)

  model = Model(model_path)
  recognizer = KaldiRecognizer(model, 16000)

  capture = pyaudio.PyAudio()
  stream = capture.open(format=pyaudio.paInt16, channels=1, rate=16000, input=True, frames_per_buffer=8192)
  stream.start_stream()

  if len(sys.argv) > 1:
    if sys.argv[1] == "-d":
      debug_mode = True
      print("Listening...")

  while True:
    if read_registry_value() == 0:
      stream.stop_stream()
      if debug_mode:
        print("Disabled listen hey pc")
      break
    if not stream.is_active():
      if debug_mode:
        print("Stream resumed")
      stream.start_stream()
    
    data = stream.read(4096)
    
    if len(data) == 0:
        break
    
    if recognizer.AcceptWaveform(data):
      result = json.loads(recognizer.Result())
      if result["text"] != "":
        
        print(result["text"])
        stream.stop_stream()
        
        if debug_mode:
          print(f"Stream paused")
        
        match result["text"]:
          case "hey": 
            is_hey = True
            await play_audio("im_here.mp3")
            continue
          case "sleep":
            if not is_hey:
              continue
            is_hey = False
            await play_audio("sleeping.mp3")
            await turn_off_screen()
          case "restart":
            if not is_hey:
              continue
            is_hey = False
            await play_audio("restarting.mp3")
            await execute_command("restart")
          case "shutdown":
            if not is_hey:
              continue
            is_hey = False
            await play_audio("shutting_down.mp3")
            await execute_command("shutdown")
          case "exit":
            if not is_hey:
              continue
            await play_audio("kill_process.mp3")
            await execute_command("mewauto")
            await execute_command("voice")
            break          

if __name__ == "__main__":
  asyncio.run(main())
          
          
      
        