use gpui::{Div, div, prelude::*, px, relative, rgb};

use crate::{
    metrics::Metrics,
    theme::Theme,
    ui::{
        dial::dial,
        graph::{graph, plot},
        kit::{Grow, MONO, label, num, panel, readout, row, stat},
    },
};

pub fn render(m: &Metrics, t: &Theme) -> Div {
    let s = &m.sys;
    let now = m.cpu.now();
    let n = s.cpus().len();
    let freq = s.cpus().iter().map(|c| c.frequency()).max().unwrap_or(0);

    let gauge = div()
        .relative()
        .flex_none()
        .child(dial(t, now, t.level(now), 176.))
        .child(
            div()
                .absolute()
                .inset_0()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap(px(2.))
                .child(readout(t, format!("{now:.0}"), "%", t.level(now), 42.))
                .child(label(t, if freq > 0 { format!("{:.2} GHz", freq as f32 / 1000.) } else { "usage".into() })),
        );

    let cores = div().flex().gap(px(3.)).children(s.cpus().iter().enumerate().map(|(i, c)| {
        let u = c.usage();
        div()
            .flex_1()
            .flex()
            .flex_col()
            .gap(px(5.))
            .child(
                div()
                    .h(px(40.))
                    .rounded(px(2.))
                    .bg(rgb(t.track))
                    .flex()
                    .flex_col()
                    .justify_end()
                    .child(div().w_full().rounded(px(2.)).bg(rgb(t.level(u))).h(relative((u / 100.).clamp(0., 1.)))),
            )
            .child(
                div()
                    .flex()
                    .justify_center()
                    .font_family(MONO)
                    .text_size(px(9.))
                    .text_color(rgb(t.dim))
                    .child(format!("{}", i + 1)),
            )
    }));

    panel(t, "Processor", m.brand.clone()).child(
        div().flex().gap(px(28.)).items_center().child(gauge).child(
            div()
                .flex_1()
                .min_w_0()
                .flex()
                .flex_col()
                .gap(px(14.))
                .child(row(
                    div()
                        .flex()
                        .gap(px(20.))
                        .child(stat(t, "Average", format!("{:.0}%", m.cpu.avg())))
                        .child(stat(t, "Peak", format!("{:.0}%", m.cpu.peak()))),
                    label(t, format!("{n} threads  {} cores", m.physical)),
                ))
                .child(graph(t, vec![(m.cpu.values(), t.accent)], 100., 96.))
                .child(cores),
        ),
    )
}

/// Small multiples: two minutes of history for every logical core, four per row.
pub fn cores(m: &Metrics, t: &Theme) -> Div {
    let cell = |i: usize, h: &crate::metrics::History| {
        let now = h.now();
        div()
            .flex()
            .flex_col()
            .gap(px(6.))
            .child(row(label(t, format!("Core {}", i + 1)), num(format!("{now:.0}%")).text_color(rgb(t.level(now)))))
            .child(plot(t, vec![(h.values(), t.level(now))], 100., 44.))
    };
    let cells: Vec<_> = m.cores.iter().enumerate().collect();
    panel(t, "Cores", format!("{} logical, last 2 min", m.cores.len())).gap(px(20.)).children(cells.chunks(4).map(|c| {
        div()
            .flex()
            .gap(px(20.))
            .children(c.iter().map(|&(i, h)| cell(i, h).grow(1.)))
            // Pad a short last row so its cells keep the same width.
            .children((c.len()..4).map(|_| div().grow(1.)))
    }))
}
