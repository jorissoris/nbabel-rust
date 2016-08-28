@echo off
cargo build --release
echo %time%
target\release\nbabel.exe < input\input16k
echo %time%
