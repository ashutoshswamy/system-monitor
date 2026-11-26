use gpui::{Div, div, prelude::*, px, relative, rgb};

use crate::{
    format::{bytes, pct},
    metrics::Metrics,
    theme::Theme,
    ui::{
        graph::graph,
        kit::{label, legend, meter, num, panel, readout, row},
    },
};

pub fn render(m: &Metrics, t: &Theme) -> Div {
    let s = &m.sys;
    let (total, used, avail, free) = (s.total_memory(), s.used_memory(), s.available_memory(), s.free_memory());
    let cached = avail.saturating_sub(free);
    let used_pct = pct(used, total);
    let swap_pct = pct(s.used_swap(), s.total_swap());
    let share = |v: u64| relative((v as f32 / total.max(1) as f32).clamp(0., 1.));
    let cached_color = t.tint(t.accent, 0x60);

    panel(t, "Memory", format!("{} installed", bytes(total as f64)))
        .child(row(
            readout(t, bytes(used as f64), "in use", t.level(used_pct), 26.),
            num(format!("{used_pct:.0}%")).text_color(rgb(t.level(used_pct))),
        ))
        .child(
            div()
                .flex()
                .gap(px(2.))
                .h(px(8.))
                .w_full()
                .rounded(px(3.))
                .overflow_hidden()
                .bg(rgb(t.track))
                .child(div().h_full().w(share(used)).bg(rgb(t.level(used_pct))))
                .child(div().h_full().w(share(cached)).bg(cached_color)),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(7.))
                .child(legend(t, rgb(t.level(used_pct)), "In use", bytes(used as f64)))
                .child(legend(t, cached_color, "Cached", bytes(cached as f64)))
                .child(legend(t, rgb(t.track), "Free", bytes(free as f64))),
        )
        .child(graph(t, vec![(m.mem.values(), t.accent)], 100., 56.))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(6.))
                .child(row(
                    label(t, "Swap"),
                    num(format!("{} of {}", bytes(s.used_swap() as f64), bytes(s.total_swap() as f64))),
                ))
                .child(meter(t, swap_pct, t.level(swap_pct))),
        )
}
