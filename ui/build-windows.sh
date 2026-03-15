#!/bin/bash
# Windows 交叉编译脚本（在 macOS 上运行）

echo "添加 Windows 编译目标..."
rustup target add x86_64-pc-windows-msvc

echo "编译 Windows 版本..."
cd src-tauri
cargo build --target x86_64-pc-windows-msvc --release

echo "构建完成！"
echo "可执行文件位于: target/x86_64-pc-windows-msvc/release/openclaw-control-ui.exe"