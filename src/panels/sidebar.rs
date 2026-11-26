//! Left rail: machine identity, at-a-glance vitals, appearance picker.

use gpui::{
    BoxShadow, Context, Div, FontWeight, MouseButton, anchored, deferred, div, point, prelude::*, px, rgb,
};
use sysinfo::System;

use crate::{
    app::Dashboard,
    format::{bytes, pct, uptime},
    metrics::Metrics,
    theme::{Choice, DAYLIGHT, GRAPHITE, THEMES, Theme},
    ui::{
        kit::{MONO, label, meter, num, row},
        logo::mark,
    },
};

pub const WIDTH: f32 = 256.;

fn heading(t: &Theme, text: &'static str) -> Div {
    div().text_size(px(12.)).font_weight(FontWeight::SEMIBOLD).text_color(rgb(t.text)).child(text)
}

fn vital(t: &Theme, name: &'static str, value: String, pct: Option<f32>, color: u32) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(7.))
        .child(row(label(t, name), num(value).text_color(rgb(color))))
        .when_some(pct, |d, p| d.child(meter(t, p, color)))
}

/// Mini rendering of a theme: background, panel and accent stacked as chips.
fn preview(th: &Theme) -> Div {
    div()
        .flex_none()
        .flex()
        .items_center()
        .gap(px(3.))
        .p(px(3.))
        .rounded(px(5.))
        .border_1()
        .border_color(rgb(th.line))
        .bg(rgb(th.bg))
        .child(div().size(px(10.)).rounded(px(2.)).bg(rgb(th.panel)))
        .child(div().size(px(10.)).rounded(px(2.)).bg(rgb(th.accent)))
}

fn option(
    t: &Theme,
    i: usize,
    choice: Choice,
    current: Choice,
    name: &'static str,
    swatches: Vec<Div>,
    cx: &mut Context<Dashboard>,
) -> gpui::Stateful<Div> {
    let selected = choice == current;
    div()
        .id(("theme", i))
        .flex()
        .items_center()
        .gap(px(10.))
        .px(px(8.))
        .py(px(6.))
        .rounded(px(7.))
        .border_1()
        .border_color(if selected { rgb(t.accent) } else { t.wash(0x00) })
        .when(selected, |d| d.bg(t.tint(t.accent, 0x14)))
        .when(!selected, |d| d.hover(|s| s.bg(t.wash(0x08))))
        .cursor_pointer()
        .on_click(cx.listener(move |this, _, _, cx| {
            this.set_choice(choice);
            cx.notify();
        }))
        .child(div().flex().gap(px(4.)).children(swatches))
        .child(
            div()
                .text_size(px(12.))
                .text_color(rgb(if selected { t.text } else { t.muted }))
                .child(name),
        )
}

pub fn render(m: &Metrics, t: &Theme, current: Choice, open: bool, cx: &mut Context<Dashboard>) -> Div {
    let s = &m.sys;
    let load = System::load_average().unwrap_or(sysinfo::LoadAvg { one: 0., five: 0., fifteen: 0. });
    let cpu = m.cpu.now();
    let mem = pct(s.used_memory(), s.total_memory());
    let swap = pct(s.used_swap(), s.total_swap());
    let peak = m.temps().first().map(|x| x.1);
    let disk = m
        .volumes()
        .into_iter()
        .find(|d| d.mount_point().as_os_str() == "/")
        .map(|d| pct(d.total_space() - d.available_space(), d.total_space()));

    let brand = div()
        .flex()
        .items_center()
        .gap(px(10.))
        .child(mark(t, 28.))
        .child(div().text_size(px(13.)).font_weight(FontWeight::SEMIBOLD).child("System Monitor"));

    let identity = div()
        .flex()
        .flex_col()
        .gap(px(4.))
        .child(div().text_size(px(15.)).font_weight(FontWeight::SEMIBOLD).truncate().child(m.host.clone()))
        .child(label(t, m.os.clone()))
        .child(label(t, format!("{}  {}", m.kernel, System::cpu_arch())).text_color(rgb(t.dim)));

    let vitals = div()
        .flex()
        .flex_col()
        .gap(px(16.))
        .child(heading(t, "Vitals"))
        .child(vital(t, "CPU", format!("{cpu:.0}%"), Some(cpu), t.level(cpu)))
        .child(vital(t, "Memory", format!("{mem:.0}%"), Some(mem), t.level(mem)))
        .child(vital(t, "Swap", format!("{swap:.0}%"), Some(swap), t.level(swap)))
        .when_some(disk, |d, p| d.child(vital(t, "Startup disk", format!("{p:.0}%"), Some(p), t.level(p))))
        .when_some(peak, |d, c| d.child(vital(t, "Hottest sensor", format!("{c:.0}°C"), Some(c / 110. * 100.), t.heat(c))))
        .child(vital(t, "Download", format!("{}/s", bytes(m.rx.now() as f64)), None, t.text))
        .child(vital(t, "Upload", format!("{}/s", bytes(m.tx.now() as f64)), None, t.text));

    let system = div()
        .flex()
        .flex_col()
        .gap(px(10.))
        .child(heading(t, "System"))
        .child(row(label(t, "Uptime"), num(uptime(System::uptime().unwrap_or(0)))))
        .child(row(label(t, "Load"), num(format!("{:.2} {:.2} {:.2}", load.one, load.five, load.fifteen))))
        .child(row(label(t, "Processes"), num(m.proc_count.to_string())));

    let (name, swatches) = match current {
        Choice::System => ("System", vec![preview(&DAYLIGHT), preview(&GRAPHITE)]),
        Choice::Fixed(i) => (THEMES[i].name, vec![preview(&THEMES[i])]),
    };
    let button = div()
        .id("theme-button")
        .flex()
        .items_center()
        .justify_between()
        .px(px(10.))
        .py(px(7.))
        .rounded(px(7.))
        .border_1()
        .border_color(rgb(if open { t.accent } else { t.line }))
        .bg(rgb(t.panel))
        .cursor_pointer()
        .hover(|s| s.border_color(rgb(t.dim)))
        .on_click(cx.listener(|this, _, _, cx| {
            this.theme_open = !this.theme_open;
            cx.notify();
        }))
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(10.))
                .child(div().flex().gap(px(4.)).children(swatches))
                .child(div().text_size(px(12.)).child(name)),
        )
        .child(div().text_size(px(10.)).text_color(rgb(t.dim)).child(if open { "▴" } else { "▾" }));

    let appearance = div()
        .flex()
        .flex_col()
        .gap(px(6.))
        .child(heading(t, "Appearance").mb(px(4.)))
        .child(button)
        .when(open, |d| {
            let mut options =
                vec![option(t, 0, Choice::System, current, "System", vec![preview(&DAYLIGHT), preview(&GRAPHITE)], cx)];
            for (i, th) in THEMES.iter().enumerate() {
                options.push(option(t, i + 1, Choice::Fixed(i), current, th.name, vec![preview(th)], cx));
            }
            // Deferred + anchored: floats over the rest of the sidebar, shifted up if it would leave the window.
            d.child(deferred(
                anchored().snap_to_window_with_margin(px(8.)).child(
                    div()
                        .id("theme-menu")
                        .occlude()
                        .w(px(WIDTH - 40.))
                        .p(px(4.))
                        .flex()
                        .flex_col()
                        .gap(px(2.))
                        .bg(rgb(t.panel))
                        .border_1()
                        .border_color(rgb(t.line))
                        .rounded(px(9.))
                        .shadow(vec![BoxShadow {
                            color: t.shadow(),
                            offset: point(px(0.), px(10.)),
                            blur_radius: px(28.),
                            spread_radius: px(-6.),
                        }])
                        .children(options),
                ),
            ))
        });

    let divider = || div().h(px(1.)).w_full().bg(rgb(t.line));

    div()
        .w(px(WIDTH))
        .flex_none()
        .h_full()
        .flex()
        .flex_col()
        .bg(rgb(t.sidebar))
        .border_r_1()
        .border_color(rgb(t.line))
        .font_family(crate::ui::kit::SANS)
        .child(
            // Top strip clears the traffic lights and drags the window.
            div()
                .h(px(52.))
                .flex_none()
                .on_mouse_down(MouseButton::Left, |_, window, _| window.start_window_move()),
        )
        .child(
            div()
                .id("sidebar")
                .flex_1()
                .overflow_y_scroll()
                .flex()
                .flex_col()
                .gap(px(22.))
                .px(px(20.))
                .pb(px(20.))
                .child(brand)
                .child(identity)
                .child(divider())
                .child(vitals)
                .child(divider())
                .child(system)
                .child(divider())
                .child(appearance)
                .child(div().font_family(MONO).text_size(px(10.)).text_color(rgb(t.dim)).child("Refreshes every second")),
        )
}
