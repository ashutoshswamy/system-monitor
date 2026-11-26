//! Root view: owns metrics, theme choice, view and sort; lays out the panels.

use std::{fs, path::PathBuf, time::Duration};

use gpui::{
    Context, FocusHandle, KeyDownEvent, MouseButton, Window, WindowAppearance, div, prelude::*, px, rgb,
};

use crate::{
    metrics::Metrics,
    panels::{
        cpu, memory,
        processes::{self, Col, Sort},
        sidebar, storage, thermals, throughput,
    },
    theme::{Choice, THEMES},
    ui::kit::{Grow, MONO, SANS},
};

const TICK: Duration = Duration::from_secs(1);
const GAP: f32 = 16.;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum View {
    Overview,
    Processor,
    Memory,
    Io,
    Processes,
}

impl View {
    pub const ALL: [(View, &'static str); 5] = [
        (View::Overview, "Overview"),
        (View::Processor, "Processor"),
        (View::Memory, "Memory"),
        (View::Io, "Network & Disk"),
        (View::Processes, "Processes"),
    ];
}

pub struct Dashboard {
    metrics: Metrics,
    pub choice: Choice,
    pub theme_open: bool,
    pub sort: Sort,
    view: View,
    appearance: WindowAppearance,
    focus: FocusHandle,
}

// ---- settings: two lines of `key=value` in Application Support ----

fn settings_path() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|h| PathBuf::from(h).join("Library/Application Support/System Monitor/settings"))
}

/// Unknown or missing values fall back to defaults; names, not indexes, so reordering never breaks a saved file.
fn parse_settings(s: &str) -> (Choice, View) {
    let (mut choice, mut view) = (Choice::System, View::Overview);
    for line in s.lines() {
        match line.split_once('=') {
            Some(("theme", n)) => {
                if let Some(i) = THEMES.iter().position(|t| t.name == n) {
                    choice = Choice::Fixed(i);
                }
            }
            Some(("view", n)) => {
                if let Some(&(v, _)) = View::ALL.iter().find(|(_, name)| *name == n) {
                    view = v;
                }
            }
            _ => {}
        }
    }
    (choice, view)
}

fn format_settings(choice: Choice, view: View) -> String {
    let theme = match choice {
        Choice::System => "System",
        Choice::Fixed(i) => THEMES[i].name,
    };
    let view = View::ALL.iter().find(|(v, _)| *v == view).map_or("Overview", |x| x.1);
    format!("theme={theme}\nview={view}\n")
}

impl Dashboard {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(TICK).await;
                let alive = this.update(cx, |d, cx| {
                    d.metrics.sample();
                    cx.notify();
                });
                if alive.is_err() {
                    break;
                }
            }
        })
        .detach();

        // System theme: re-resolve whenever macOS switches light/dark.
        cx.observe_window_appearance(window, |d, window, cx| {
            d.appearance = window.appearance();
            cx.notify();
        })
        .detach();

        let (choice, view) =
            settings_path().and_then(|p| fs::read_to_string(p).ok()).map_or((Choice::System, View::Overview), |s| {
                parse_settings(&s)
            });
        let focus = cx.focus_handle();
        focus.focus(window);
        let mut d = Self {
            metrics: Metrics::new(),
            choice,
            theme_open: false,
            sort: Sort::default(),
            view: View::Overview,
            appearance: window.appearance(),
            focus,
        };
        d.set_view(view);
        d
    }

    fn save(&self) {
        // ponytail: settings are a convenience; a failed write just means defaults next launch.
        if let Some(p) = settings_path() {
            let _ = p.parent().map(fs::create_dir_all);
            let _ = fs::write(p, format_settings(self.choice, self.view));
        }
    }

    pub fn set_choice(&mut self, choice: Choice) {
        self.choice = choice;
        self.theme_open = false;
        self.save();
    }

    fn set_view(&mut self, view: View) {
        // The Memory view is about memory: start it sorted by memory.
        if view == View::Memory && self.view != View::Memory {
            self.sort = Sort { col: Col::Mem, desc: true };
        }
        self.view = view;
        self.save();
    }
}

impl Render for Dashboard {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = self.choice.resolve(self.appearance);
        let m = &self.metrics;
        let gap = px(GAP);
        let sort = self.sort;

        // ponytail: views recombine existing panels; no per-view panel variants until one needs it.
        let body = div().flex().flex_col().gap(gap);
        let body = match self.view {
            View::Overview => body
                .child(cpu::render(m, &t))
                .child(
                    div()
                        .flex()
                        .gap(gap)
                        .child(memory::render(m, &t).grow(1.))
                        .child(throughput::network(m, &t).grow(1.))
                        .child(throughput::disk(m, &t).grow(1.)),
                )
                .child(
                    div()
                        .flex()
                        .items_start()
                        .gap(gap)
                        .child(processes::render(m, &t, sort, Some(10), cx).grow(2.))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(gap)
                                .grow(1.)
                                .min_w_0()
                                .child(thermals::render(m, &t))
                                .child(storage::render(m, &t)),
                        ),
                ),
            View::Processor => body.child(cpu::render(m, &t)).child(cpu::cores(m, &t)).child(thermals::render(m, &t)),
            View::Memory => body.child(
                div()
                    .flex()
                    .items_start()
                    .gap(gap)
                    .child(memory::render(m, &t).grow(1.))
                    .child(processes::render(m, &t, sort, Some(20), cx).grow(2.)),
            ),
            View::Io => body
                .child(
                    div()
                        .flex()
                        .gap(gap)
                        .child(throughput::network(m, &t).grow(1.))
                        .child(throughput::disk(m, &t).grow(1.)),
                )
                .child(throughput::interfaces(m, &t))
                .child(storage::render(m, &t)),
            View::Processes => body.child(processes::render(m, &t, sort, None, cx)),
        };

        let tabs = div().flex().gap(px(4.)).p(px(3.)).rounded(px(8.)).bg(t.wash(0x08)).children(
            View::ALL.iter().enumerate().map(|(i, &(v, name))| {
                let on = v == self.view;
                div()
                    .id(("view", i))
                    .flex()
                    .items_baseline()
                    .gap(px(6.))
                    .px(px(12.))
                    .py(px(5.))
                    .rounded(px(6.))
                    .text_size(px(12.))
                    .cursor_pointer()
                    .text_color(rgb(if on { t.text } else { t.muted }))
                    .when(on, |d| d.bg(rgb(t.panel)).border_1().border_color(rgb(t.line)))
                    .when(!on, |d| d.hover(|s| s.text_color(rgb(t.text))))
                    // Keep the header's drag handler from eating the click.
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .on_click(cx.listener(move |d, _, _, cx| {
                        d.set_view(v);
                        cx.notify();
                    }))
                    .child(name)
                    .child(div().font_family(MONO).text_size(px(10.)).text_color(rgb(t.dim)).child(format!("⌘{}", i + 1)))
            }),
        );

        let main = div()
            .flex_1()
            .min_w_0()
            .h_full()
            .flex()
            .flex_col()
            .child(
                // Header doubles as drag strip, level with the traffic lights.
                div()
                    .flex_none()
                    .flex()
                    .items_center()
                    .px(px(24.))
                    .pt(px(12.))
                    .pb(px(16.))
                    .on_mouse_down(MouseButton::Left, |_, w, _| w.start_window_move())
                    .child(tabs),
            )
            .child(div().id("main").flex_1().overflow_y_scroll().px(px(24.)).pb(px(24.)).child(body));

        div()
            .size_full()
            .relative()
            .flex()
            .bg(rgb(t.bg))
            .text_color(rgb(t.text))
            .font_family(SANS)
            .track_focus(&self.focus)
            .on_key_down(cx.listener(|d, e: &KeyDownEvent, _, cx| {
                let k = &e.keystroke;
                if k.key == "escape" && d.theme_open {
                    d.theme_open = false;
                } else if k.modifiers.platform
                    && let Some(&(v, _)) = k.key.parse::<usize>().ok().and_then(|n| View::ALL.get(n.wrapping_sub(1)))
                {
                    d.set_view(v);
                } else {
                    return;
                }
                cx.notify();
            }))
            .child(sidebar::render(m, &t, self.choice, self.theme_open, cx))
            .child(main)
            // Click-away backdrop for the theme menu; the menu itself is deferred and paints above this.
            .when(self.theme_open, |d| {
                d.child(div().absolute().inset_0().occlude().on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|d, _, _, cx| {
                        d.theme_open = false;
                        cx.notify();
                    }),
                ))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_round_trip() {
        for (c, v) in [(Choice::System, View::Overview), (Choice::Fixed(6), View::Io), (Choice::Fixed(0), View::Processes)] {
            let (c2, v2) = parse_settings(&format_settings(c, v));
            assert!(c2 == c && v2 == v);
        }
        // Garbage and unknown names fall back to defaults.
        let (c, v) = parse_settings("theme=Nope\nview=\njunk");
        assert!(c == Choice::System && v == View::Overview);
    }
}
