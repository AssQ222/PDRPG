# PDRPG (Personal Development RPG)

A desktop application for personal development that combines task management, goal tracking, and habit building with RPG gamification elements. Built with Tauri + Rust + Svelte + TypeScript.

## 🚀 Quick Start (Windows)

### Prerequisites
- Windows 10/11
- PowerShell 5.1 or later
- [Node.js](https://nodejs.org/) 18+ with [pnpm](https://pnpm.io/)

### One-time Setup
```bash
# Clone the repository
git clone <your-repo-url>
cd PDRPG

# Run automated setup (installs Rust, configures toolchain, sets PATH)
pnpm setup

# If setup fails, restart your terminal and try again
```

### Development
```bash
# Start development server (always run from project root)
pnpm tauri dev

# Alternative: Run frontend and backend separately
# Terminal 1 (Frontend):
pnpm dev

# Terminal 2 (Backend):
cd src-tauri
cargo run
```

### Troubleshooting

**❌ Error: `failed to get cargo metadata: program not found`**
- Solution: Run `pnpm setup` and restart your terminal
- Related: [tauri-apps/tauri#5223](https://github.com/tauri-apps/tauri/issues/5223)

**❌ Error: `could not find Cargo.toml in D:\path\to\PDRPG`**
- Solution: Always run `pnpm tauri dev` from the project root directory
- For backend only: Run `cargo run` from the `src-tauri` directory

**❌ Rust not found after setup**
- Manually install: [rustup.rs](https://rustup.rs/)
- Add to PATH: `%USERPROFILE%\.cargo\bin`
- Configure toolchain: `rustup default stable-msvc`

## 🛠️ Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

The project includes VS Code configuration for:
- Rust debugging with LLDB
- Automatic formatting (Rust + Svelte + TypeScript)
- Integrated tasks for building and running
- rust-analyzer configured for Windows MSVC
