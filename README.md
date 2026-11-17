# PixelWid

A small, always-on-top desktop widget with a live clock, date, and locally
saved to-do list. Its borderless window can be dragged by the `[ PIXELWID // DESK ]`
header. The compact terminal-inspired colors and monospace layout give it a
pixel-art feel without requiring image assets.

## Run on Windows 11

Install the stable Rust toolchain and the Visual C++ build tools/Windows SDK,
then run this from PowerShell in the project directory:

```powershell
cargo run --release
```

The app opens as a native, always-on-top desktop window. Tasks are stored in the
operating system's local data directory under `PixelWid/tasks.json`.

## Graphical session required

PixelWid needs a graphical desktop session to create its window. Building it in
a headless environment such as GitHub Codespaces is supported, but launching it
there fails because there is no display server. Run the app from an interactive
Windows 11 desktop, or from Linux with a working Wayland or X11 session
(`WAYLAND_DISPLAY` or `DISPLAY` set).
