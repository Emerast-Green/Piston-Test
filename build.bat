cargo build -r
move .\target\release\piston_test.exe .\game\piston_test.exe
del "game.rar"
"C:\Program Files\WinRAR\WinRAR.exe" a "game.rar" "game"