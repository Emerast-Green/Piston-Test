# build for windows - output at ./game_win.tar
cargo build --target=x86_64-pc-windows-gnu --release
mv ./target/x86_64-pc-windows-gnu/release/piston_test.exe ./piston_test.exe
rm game_win.tar
tar -cf "game_win.tar" piston_test.exe assets
# build for linux - output at ./game_lin.tar
cargo build --release
mv ./target/release/piston_test ./piston_test
rm game_lin.tar
tar -cf "game_lin.tar" piston_test assets