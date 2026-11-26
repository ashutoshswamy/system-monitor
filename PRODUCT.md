# System Monitor

Native desktop (GPUI, Rust) system monitor. Surface mode: **Operate**.

- Users: developers glancing at machine health while working; window sits beside an editor.
- Shows: CPU (total, per-core, history), memory + swap, thermals, storage, network, host/uptime/load.
- Direction: instrument panel. Near-black graphite, one amber signal accent, coral only for critical state (temp >= 85 C, usage >= 90%). Monospace for numbers only, Avenir Next for labels. Calm, precise, no decoration.
- Themes: System (Daylight when macOS is light, Graphite when dark), Graphite, Daylight, Midnight, Moss, Dusk, Tide, Ember, Sand, Frost. Same roles in each: neutrals, one accent, one critical color.
- Views: Overview (all panels), Processor (CPU + thermals), Memory, I/O (network, disk, storage), Processes. Tab strip at top of main area.
- Assumption (not interviewed): single window, 1s refresh, read-only, macOS primary.
