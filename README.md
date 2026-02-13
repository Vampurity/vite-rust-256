# 🔐 Vue + Rust WebAssembly 字符串加密解密工具

这是一个使用 Vue.js 和 Rust WebAssembly 构建的字符串加密解密工具。它展示了如何在前端项目中集成 Rust 代码来实现高性能的加密解密功能。

## ✨ 功能特性

- 🔒 **字符串加密**: 使用 Rust 实现的 XOR 加密算法
- 🔓 **字符串解密**: 支持解密加密后的字符串
- 🎲 **随机密钥生成**: 自动生成安全的随机密钥
- ⚡ **高性能**: 使用 WebAssembly 实现，性能接近原生代码
- 🎨 **现代化 UI**: 响应式设计，支持移动端
- 🛡️ **类型安全**: Rust 的类型系统确保代码安全性

## 🚀 快速开始

### 前置要求

- Node.js (版本 20.19.0 或更高)
- Rust 工具链
- pnpm (推荐) 或 npm

### 安装步骤

1. **克隆项目**

   ```bash
   git clone <your-repo-url>
   cd vue-rust
   ```

2. **安装 Rust 工具链**

   ```bash
   # 安装 Rust (如果还没有安装)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # 添加 WebAssembly 目标
   rustup target add wasm32-unknown-unknown

   # 安装 wasm-bindgen
   cargo install wasm-bindgen-cli
   ```

3. **安装项目依赖**

   ```bash
   pnpm install
   # 或者使用 npm install
   ```

4. **构建 WebAssembly 模块**

   ```bash
   pnpm run build:wasm
   ```

5. **启动开发服务器**

   ```bash
   pnpm run dev
   ```

6. **打开浏览器**
   访问 `http://localhost:5173` 查看应用

## 🏗️ 项目结构

```
vue-rust/
├── src/
│   ├── App.vue          # Vue 主组件
│   └── main.js          # 应用入口
├── pkg/                 # WebAssembly 生成的文件
│   ├── vue_rust_crypto.js
│   ├── vue_rust_crypto_bg.wasm
│   └── *.d.ts          # TypeScript 类型定义
├── src/lib.rs          # Rust 源代码
├── Cargo.toml          # Rust 项目配置
├── build-wasm.sh       # WebAssembly 构建脚本
└── vite.config.js      # Vite 配置
```

## 🔧 开发指南

### 修改 Rust 代码

1. 编辑 `src/lib.rs` 文件
2. 运行 `pnpm run build:wasm` 重新构建
3. 刷新浏览器查看更改

### 添加新的加密算法

在 `src/lib.rs` 中添加新的函数，并使用 `#[wasm_bindgen]` 宏导出：

```rust
#[wasm_bindgen]
pub fn your_new_function(input: &str) -> String {
    // 你的实现
}
```

### 构建生产版本

```bash
pnpm run build
```

## 🛠️ 技术栈

- **前端**: Vue 3 + Vite
- **后端**: Rust + WebAssembly
- **构建工具**: wasm-bindgen
- **样式**: CSS3 (原生)

## 📝 使用说明

1. **输入密钥**: 在密钥框中输入加密密钥，或点击"生成随机密钥"
2. **输入文本**: 在原始文本框中输入要加密的内容
3. **加密**: 点击"加密"按钮获得加密结果
4. **解密**: 使用相同密钥点击"解密"按钮来解密

## 🔒 安全说明

- 当前实现使用 XOR 加密，适合学习和演示
- 生产环境建议使用更安全的加密算法（如 AES）
- 密钥应该安全存储，不要在前端代码中硬编码

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

MIT License

This template should help get you started developing with Vue 3 in Vite.

## Recommended IDE Setup

[VSCode](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) (and disable Vetur).

## Customize configuration

See [Vite Configuration Reference](https://vite.dev/config/).

## Project Setup

```sh
npm install
```

### Compile and Hot-Reload for Development

```sh
npm run dev
```

### Compile and Minify for Production

```sh
npm run build
```
