# PixelWid

A small, always-on-top desktop widget with a live clock, date, and locally
saved to-do list. Its native title bar supports dragging and resizing. The
compact terminal-inspired colors and monospace layout give it a pixel-art feel
without requiring image assets.

## Run on Windows 10/11 (x64)

Install the stable Rust toolchain and the Visual C++ build tools/Windows SDK,
then build the standalone executable from PowerShell in the project directory:

```powershell
$env:RUSTFLAGS = "-C target-feature=+crt-static"
cargo build --release --target x86_64-pc-windows-msvc
.\target\x86_64-pc-windows-msvc\release\pixelwid.exe
```

The MSVC build statically links the C runtime and does not require MinGW on the
build or target machine. You can also download the `pixelwid-windows-x64`
artifact from the `Build Windows executable` GitHub Actions workflow. The app
opens as a resizable, always-on-top window and appears on the taskbar. Tasks
are stored in the operating system's local data directory under
`PixelWid/tasks.json`.

## Graphical session required

PixelWid needs a graphical desktop session to create its window. Building it in
a headless environment such as GitHub Codespaces is supported, but launching it
there fails because there is no display server. Run the app from an interactive
Windows 11 desktop, or from Linux with a working Wayland or X11 session
(`WAYLAND_DISPLAY` or `DISPLAY` set).
