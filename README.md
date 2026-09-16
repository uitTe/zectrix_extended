# zectrix-extended

基于 **Tauri 2 + Vue 3 + TypeScript + Vite** 的桌面应用。

## 技术栈

| 层 | 选型 |
|---|---|
| 桌面框架 | Tauri 2 |
| 前端 | Vue 3（`<script setup lang="ts">` 单文件组件） |
| 构建 | Vite 8 + vue-tsc |
| 后端 | Rust（stable，`x86_64-pc-windows-msvc`） |
| 包管理器 | yarn |

## 开发命令

| 目的 | 命令 |
|---|---|
| 桌面应用调试（热重载） | `yarn tauri dev` |
| 前端热重载（仅浏览器） | `yarn dev` |
| 类型检查 | `yarn typecheck` |
| 前端构建 | `yarn build` |
| 打包发布 | `yarn tauri build` |
| 环境自检 | `yarn tauri info` |

## 目录结构

- `src/` — 前端源码：`main.ts`（入口，`createApp` 挂载）、`App.vue`（根组件）、`components/`（业务组件）、`composables/`（共享状态单例）、`api/`（invoke 封装）、`types/`（类型定义）、`assets/`（静态资源）、`styles.scss`（全局样式，SCSS）
- `src-tauri/` — Rust 后端：`src/lib.rs`（`#[tauri::command]` 注册）、`tauri.conf.json`（窗口与打包配置）、`capabilities/`（Tauri v2 权限声明）
- 前端调用 Rust：`import { invoke } from "@tauri-apps/api/core"`

## 推荐 IDE 配置

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

