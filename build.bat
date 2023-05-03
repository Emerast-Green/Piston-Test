cargo build -r
move .\target\release\piston_test.exe .\piston_test.exe
del "game.rar"
"C:\Program Files\WinRAR\WinRAR.exe" a "game.rar" "assets"
"C:\Program Files\WinRAR\WinRAR.exe" a "game.rar" "piston_test.exe"