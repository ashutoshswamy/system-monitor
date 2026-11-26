# System Monitor

A native macOS system monitor written in Rust with [GPUI](https://www.gpui.rs) and [sysinfo](https://crates.io/crates/sysinfo). It refreshes once per second and shows CPU, memory, thermals, storage, network and processes.

## Features

- **Five views.** Switch with the tab strip or **⌘1–⌘5**.
  - **Overview:** every panel at once.
  - **Processor:** total CPU usage, a two-minute history for each core, and thermals.
  - **Memory:** memory and swap, plus processes sorted by memory.
  - **Network & Disk:** network and disk throughput, traffic per interface, and storage volumes.
  - **Processes:** every running process.
- **Sortable process table.** Click a column header (Name, PID, CPU, Memory) to sort by it. Click again to reverse the order.
- **Ten themes** from the Appearance dropdown in the sidebar: System (follows macOS light/dark), Graphite, Daylight, Midnight, Moss, Dusk, Tide, Ember, Sand and Frost.
- **Remembers your theme and view** between launches. They are saved in `~/Library/Application Support/System Monitor/settings`.
- **Sidebar vitals:** host, OS, uptime, load average and process count.

## Install

Download `System-Monitor-<version>-macos-arm64.zip` from [Releases](../../releases), unzip it, and move **System Monitor.app** to `/Applications`.

The app is ad-hoc signed but not notarized, so macOS blocks it the first time. Right-click the app, choose **Open**, then confirm. Or run:

```sh
xattr -dr com.apple.quarantine "/Applications/System Monitor.app"
```

The release build is for Apple Silicon only. On an Intel Mac, build from source.

## Build from source

You need Rust (edition 2024, so Rust 1.85 or newer) and the Xcode command line tools.

```sh
cargo run --release          # run directly
sh scripts/bundle.sh         # build target/release/System Monitor.app and a zip
cargo test                   # run the tests
```

## Project layout

```
src/
  main.rs        window setup
  app.rs         root view: views, shortcuts, saved settings
  metrics.rs     sampling and rolling history (no UI)
  theme.rs       color themes
  panels/        one file per panel (cpu, memory, processes, ...)
  ui/            shared widgets: graph, dial, meters, logo
scripts/
  bundle.sh      .app bundle, icon, ad-hoc signature, zip
```

## Limitations

- macOS only. Window chrome and sensor names assume macOS.
- Some temperature sensors are unavailable without elevated permissions, so the Thermals panel may show nothing.
- The app only shows data. It cannot kill or renice processes.
