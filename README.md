<div align="center">

# 🌌 Nshorizon

**Minecraft: Java Edition 26.3 rewritten from scratch in pure Rust (`wgpu`).**  
**100% Free & Open-Source · Zero Account Barrier · Native NSGO Authentication · 240+ Hz High-Performance Native Engine**

[![GitHub Release](https://img.shields.io/github/v/release/naborajs/Nshorizon?color=orange&label=Release)](https://github.com/naborajs/Nshorizon/releases/latest)
[![GitHub Stars](https://img.shields.io/github/stars/naborajs/Nshorizon?style=flat&color=yellow&label=Stars)](https://github.com/naborajs/Nshorizon/stargazers)
[![Auth: NSGO Native](https://img.shields.io/badge/Auth-NSGO%20Native-ea580c.svg)](https://go.naborajs.me)
[![License: LGPL-3.0-or-later](https://img.shields.io/badge/License-LGPL--3.0--or--later-blue.svg)](LICENSE)
[![Rust: 1.95+](https://img.shields.io/badge/Rust-1.95%2B-black.svg?logo=rust)](https://www.rust-lang.org)
[![Graphics: wgpu](https://img.shields.io/badge/Graphics-wgpu%20(Vulkan%2FDX12%2FMetal)-purple.svg)](https://wgpu.rs)
[![Platforms](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-informational.svg)](https://github.com/naborajs/Nshorizon/releases/latest)

<br/>

<a href="https://github.com/naborajs/Nshorizon/releases/latest">
  <img src="docs/media/readme/download-button.png" alt="Download Nshorizon Launcher" width="380">
</a>

<br/>

**🎉 100% Free & Open to Everyone — No Paid Minecraft Account Required!**  
*Directly download the setup file from our GitHub Releases below, run it, and play.*

</div>

---

## 📥 Downloads & Direct Releases

You do not need to download anything from external sites. All binaries and installers are built and released directly from our repository:

| Platform | Distribution | Download Link | Quick Description |
| :--- | :--- | :--- | :--- |
| **🪟 Windows (x64 / ARM64)** | **Setup Installer (.exe)** | [**⬇️ Download `nshorizon-launcher-setup.exe`**](https://github.com/naborajs/Nshorizon/releases/latest) | **Recommended.** Installs Nshorizon Launcher with automatic desktop & start menu shortcuts. |
| **🪟 Windows (Portable)** | Standalone Archive (.zip) | [**⬇️ Download `nshorizon-launcher-windows-x64.zip`**](https://github.com/naborajs/Nshorizon/releases/latest) | Extract to any folder and run `horizon-launcher.exe` directly without setup. |
| **🐧 Linux (x86_64)** | AppImage / Tarball | [**⬇️ Download `nshorizon-launcher-linux-x86_64.tar.gz`**](https://github.com/naborajs/Nshorizon/releases/latest) | Self-contained universal executable for Ubuntu, Arch, Fedora, Debian, and NixOS. |
| **🍎 macOS (Apple Silicon / Intel)** | Disk Image (.dmg) | [**⬇️ Download `nshorizon-launcher-macos.dmg`**](https://github.com/naborajs/Nshorizon/releases/latest) | Drag-and-drop installer for macOS 12+ (Monterey and newer). |

> [!TIP]
> **Getting Started in 60 Seconds:**
> 1. Download [`nshorizon-launcher-setup.exe`](https://github.com/naborajs/Nshorizon/releases/latest) from the [Releases](https://github.com/naborajs/Nshorizon/releases) page.
> 2. Run the installer and open **Nshorizon Launcher**.
> 3. Click **NSGO** to sign in with your free NSGO account (or hit *Not now* to play immediately).
> 4. Hit **Play**! The launcher prepares assets and launches the game in seconds.

---

## 🌟 What is Nshorizon?

**Nshorizon** is a high-performance, open-source reimplementation of Minecraft (version 26.3) written from the ground up in **Rust** using **`wgpu`** (Vulkan, DirectX 12, and Metal).

By escaping the Java Virtual Machine (JVM), Nshorizon eliminates garbage collection stutter, reduces memory consumption by over 60%, and delivers near-instantaneous startup times while remaining completely compatible with vanilla Minecraft worlds and servers.

<p align="center">
  <img src="docs/media/readme/hero-lookout.webp" alt="Nshorizon Gameplay Lookout" width="100%">
</p>

### ✨ The Nshorizon Freedom Promise:
- 🚀 **100% Free Forever:** Zero paywalls. No Microsoft account or Minecraft game purchase required.
- 🔥 **Built-in NSGO Integration:** Seamless authentication through [NSGO (`go.naborajs.me`)](https://go.naborajs.me) using OAuth 2.1 PKCE. Persistent multiplayer UUIDs, custom skins, and capes hosted for free.
- 🔀 **Dual-Authentication Freedom:** Want to use your official Microsoft account too? Both **NSGO** and **Microsoft** logins are supported side-by-side with 1-click profile switching.

---

## ⚡ Performance & Benchmarks

<p align="center">
  <img src="docs/media/readme/signs.webp" alt="Performance Benchmarks" width="100%">
</p>

| Metric | Minecraft: Java Edition (Modded / Sodium) | 🌌 Nshorizon (Pure Rust) |
| :--- | :--- | :--- |
| **Startup Time** | 15 – 35+ seconds | **~3.5 seconds** (Instantaneous) |
| **Default Render Distance** | 12 chunks (~441 chunk volume) | **32 chunks** (~3,217 chunk volume, $7.1\times$ higher) |
| **Far Terrain Streaming** | Up to 32 chunks with stutter | **Up to 1,024 chunks** via worker thread pool |
| **Garbage Collection Pauses** | Frequent micro-stutters during GC cycles | **Zero GC Pauses** (Deterministic Rust memory management) |
| **CPU Worker Threads** | Single-thread bottlenecked logic | **Full Multi-Core Utilization** (31+ worker threads) |
| **Dedicated Server Memory** | 1.5 – 4 GB RAM idle | **< 150 MB RAM idle** |

---

## 🎮 Authentication Matrix

| Capability | 🎮 NSGO Account (Default) | 🪟 Microsoft Account |
| :--- | :--- | :--- |
| **Cost** | **100% Free** | Paid ($29.99 Minecraft license) |
| **Account Provider** | [NSGO (`go.naborajs.me`)](https://go.naborajs.me) | Xbox Live / Microsoft |
| **Authentication Flow** | RFC 7636 OAuth 2.1 PKCE (S256) | OAuth 2.0 Device Flow |
| **Custom Skins & Capes** | ✅ Included & Hosted Free via NSGO Yggdrasil | ✅ Included via Mojang Profile |
| **Multiplayer LAN / P2P** | ✅ Supported | ✅ Supported |
| **Online Servers & Horizon SMP** | ✅ Full Online Mode Support | ✅ Official Realms & Online Mode |
| **Offline / Fast Launch** | ✅ Instant Launch | ✅ Supported |

---

## 🎨 Modding, Resource Packs & Shaders

* **Shader Packs:** Built-in GLSL-to-WGSL shader compiler translates standard OptiFine and Iris `.zip` shader packs (`.vsh`, `.fsh`, `shaders.properties`) directly into native GPU compute & fragment shaders.
* **Resource Packs:** 100% vanilla parity. Drop resource packs into your instance directory or manage them from the launcher's **Resource packs** tab.
* **Native Rust Mods:** High-speed native mods run directly on the engine through `crates/horizon-mod-runtime` with zero JVM overhead.

---

## 🛠️ Building from Source

If you want to build Nshorizon locally or contribute to development:

### Prerequisites

- **Rust:** Version `1.95` or newer (`rustup update`).
- **C/C++ Build Environment:**
  - **Windows:** Visual Studio 2022 C++ Build Tools (run in developer prompt via `vcvars64.bat`).
  - **Linux:** `libasound2-dev`, `libudev-dev`, `pkg-config`, `libx11-dev`, `libwayland-dev`.
  - **macOS:** Xcode Command Line Tools (`xcode-select --install`).

### Build & Run Commands

```bash
# 1. Clone the repository
git clone https://github.com/naborajs/Nshorizon.git
cd Nshorizon

# 2. Run the Nshorizon GPUI Desktop Launcher
cargo run --manifest-path launcher/Cargo.toml

# 3. Or compile the fast, optimized game executable directly
cargo build --profile fast -p horizon-game
```

---

## 🏛️ Repository Architecture

```
Nshorizon/
├── launcher/                  # Modern GPUI desktop launcher (NSGO + Microsoft dual auth)
├── crates/
│   ├── horizon-client/        # Core client simulation loop and input dispatcher
│   ├── horizon-client-render/ # wgpu pipeline, chunk meshing, and camera
│   ├── horizon-world/         # World simulation, blocks, entities, and redstone
│   ├── horizon-core/          # Simulation primitives, RNG, and HotSpot exact math
│   ├── horizon-protocol/      # Network protocol 777 codecs and packet serializers
│   ├── horizon-shaders/       # Real-time GLSL-to-WGSL shader translation compiler
│   ├── horizon-server/        # Ultra-low-latency headless dedicated server
│   ├── horizon-cosmetics/     # Custom skin, cape, and cosmetics rendering
│   └── horizon-mod-runtime/   # Native Rust modding interface
├── tools/                     # Parity test suites, public tree checks, and harnesses
└── docs/                      # Parity ledger, architecture specifications, and guides
```

---

## 📜 Licenses & Acknowledgments

- **Nshorizon** is released under the **[GNU LGPL, version 3 or later](LICENSE)** for the project's original work.
- Built upon the pioneering work of the **Horizon Team** ([`horizon-rust/horizon`](https://github.com/horizon-rust/horizon)), founded by [Ludovic Marie](https://lu4ovic.com).
- **NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.**  
  Minecraft is a trademark of Mojang Synergies AB; all original Minecraft game code and assets belong to Mojang and Microsoft under the [Minecraft EULA](https://www.minecraft.net/en-us/eula). Nshorizon does not distribute proprietary Mojang game assets or jars.
