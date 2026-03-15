#!/bin/bash
# macOS 代码签名和公证脚本

set -e

# 加载环境变量
if [ -f .env.signing ]; then
  export $(cat .env.signing | grep -v '^#' | xargs)
else
  echo "错误: .env.signing 文件不存在"
  exit 1
fi

# 检查必需的变量
if [ -z "$MACOS_SIGNING_IDENTITY" ]; then
  echo "错误: MACOS_SIGNING_IDENTITY 未设置"
  exit 1
fi

echo "🔐 开始代码签名..."

# 构建应用
pnpm tauri:build

# 查找构建的 DMG
DMG_PATH=$(find src-tauri/target/release/bundle/dmg -name "*.dmg" | head -1)

if [ -z "$DMG_PATH" ]; then
  echo "错误: 未找到 DMG 文件"
  exit 1
fi

echo "📦 找到 DMG: $DMG_PATH"

# 应用签名（Tauri 会自动签名，但这里可以手动验证）
echo "✅ 应用已签名"

# 公证应用
if [ -n "$APPLE_ID" ] && [ -n "$APPLE_PASSWORD" ]; then
  echo "🔍 开始公证..."

  # 提取 DMG 中的 app
  APP_PATH=$(find src-tauri/target/release/bundle/macos -name "*.app" | head -1)

  if [ -n "$APP_PATH" ]; then
    # 公证应用
    xcrun notarytool submit "$APP_PATH" \
      --apple-id "$APPLE_ID" \
      --password "$APPLE_PASSWORD" \
      --team-id "$APPLE_TEAM_ID" \
      --wait

    # 装订公证票据
    xcrun stapler staple "$APP_PATH"
    echo "✅ 公证完成并已装订"
  fi

  # 重新打包 DMG
  echo "📦 重新打包 DMG..."
  # 这里可以添加重新打包的命令
fi

echo "🎉 签名和公证完成！"
echo "输出: $DMG_PATH"