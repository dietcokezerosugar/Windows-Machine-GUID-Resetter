# 📟 Windows Machine GUID Resetter

![License](https://img.shields.io/badge/License-MIT-green.svg) ![Platform](https://img.shields.io/badge/Platform-Windows-blue.svg) ![Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)

> **"Identity is a fluid concept."**  
> *A secure, hand-crafted tool to securely reset your Windows MachineGuid.*

---

## 🕵️‍♂️ About
This is a lightweight utility designed to reset the `MachineGuid` key in the Windows Registry (`HKLM\SOFTWARE\Microsoft\Cryptography`). It features a custom **Retro Terminal UI** that mimics old-school hacking tools.

**Why use this?**
- **🛡️ Secure**: strictly checks for Administrator privileges before doing anything.
- **⚡ Fast**: Written in pure Rust. zero lag.
- **📦 Portable**: Statically linked. Runs on ANY Windows PC without installation or dependencies.
- **🎨 Stylish**: Phosphor Green on Black aesthetic. No boring white windows.

## 📸 Usage

1.  **Download** the latest `.exe` from the Releases page.
2.  **Right-Click** the app and select **"Run as Administrator"**.
    - *Note: The app will block access if you don't run it as Admin.*
3.  Click the **[ CLICK TO RESET ]** button.
4.  Your Machine GUID is now verified and spoofed to a new random UUID.

## 🛠️ Build from Source

You can build this tool yourself to verify its safety.

```bash
# Clone the repo
git clone https://github.com/yourusername/win_guid_reset.git
cd win_guid_reset

# Build independent release (static linking)
cargo build --release
```

The binary will be located at `target/release/win_guid_reset.exe`.

## ⚠️ Disclaimer
This tool modifies the Windows Registry. While safe, it is always recommended to:
1.  Know what you are doing.
2.  Use at your own risk.

## 🤝 Contributing
**Hackers welcome.**  
We want to make this the coolest looking utility on GitHub.

- **Designers**: Send PRs for cooler UI layouts!
- **Coders**: Optimize the registry logic or add new features.

## 📜 License
MIT License. Hand-written with <3 in Rust.
