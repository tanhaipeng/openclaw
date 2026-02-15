# OpenClaw Dashboard (Tauri 客户端)

用 Tauri 2 构建的桌面客户端，窗口内直接加载并渲染 [OpenClaw Gateway](http://127.0.0.1:18789/) 的 Dashboard 页面。

## 前置条件

- **Node.js** 22+
- **pnpm**（与仓库根目录一致）
- **Rust**：需安装 [rustup](https://rustup.rs/)，并设置默认 toolchain：
  ```bash
  rustup default stable
  ```
- 本机已启动 Gateway，且 Dashboard 可在浏览器访问：`http://127.0.0.1:18789/`

## 开发

在仓库根目录：

```bash
pnpm dashboard:dev
```

或在当前目录：

```bash
cd apps/tauri-dashboard
pnpm install   # 若尚未安装
pnpm run dev
```

会编译 Rust 并打开一个桌面窗口，窗口内加载 `http://127.0.0.1:18789/`。

## 构建

```bash
pnpm dashboard:build
```

或在当前目录：`pnpm run build`。产物在 `src-tauri/target/release/`（以及各平台的 bundle 目录）。

## Gateway Token（避免 1008 unauthorized）

Gateway 要求鉴权时，Dashboard 的 WebSocket 需要 token。本客户端会**自动从 OpenClaw 配置里读取 token**，并加载带 `?token=...` 的 URL，这样页面加载后会自动带上 token 连接。

- **配置位置**：`~/.openclaw/openclaw.json` 中的 `gateway.auth.token`（或设置环境变量 `OPENCLAW_CONFIG_PATH` 指向其它配置文件）。
- 若尚未配置 token，可运行：`openclaw config set gateway.auth.token <你的 token>`，或由 Gateway 首次启动时自动生成后执行 `openclaw config get gateway.auth.token` 查看。
- 若不使用配置文件，也可在浏览器中打开带 token 的 Dashboard 地址，在 Control UI 设置里粘贴 token；Tauri 窗口与浏览器**不共享** localStorage，因此 Tauri 端仍建议用上述配置文件方式。

## 配置说明

- 加载地址在运行时由 Rust 代码固定为 `http://127.0.0.1:18789/`（与 `tauri.conf.json` 中 build 的默认一致）；若将来支持其它端口，需改 `src-tauri/src/config.rs` 中的 `BASE` 常量并重新编译。
