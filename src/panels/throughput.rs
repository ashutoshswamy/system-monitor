//! Two-series rate panels: network in/out and disk read/write.

use gpui::{Div, div, prelude::*, px, rgb};

use crate::{
    format::bytes,
    metrics::{History, Metrics},
    theme::Theme,
    ui::{
        graph::graph,
        kit::{empty, label, num, panel, readout, swatch},
    },
};

struct Series<'a> {
    name: &'static str,
    hist: &'a History,
    color: u32,
    note: Option<String>,
}

fn rates(t: &Theme, title: &'static str, a: Series, b: Series) -> Div {
    let peak = a.hist.0.iter().chain(&b.hist.0).fold(1024f32, |x, y| x.max(*y));
    let col = |s: &Series| {
        div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .gap(px(3.))
            .child(div().flex().items_center().gap(px(7.)).child(swatch(rgb(s.color))).child(label(t, s.name)))
            .child(readout(t, bytes(s.hist.now() as f64), "/s", t.text, 20.))
            .when_some(s.note.clone(), |d, n| d.child(label(t, n).text_color(rgb(t.dim))))
    };
    panel(t, title, format!("scale {}/s", bytes(peak as f64)))
        .child(div().flex().gap(px(16.)).child(col(&a)).child(col(&b)))
        .child(graph(t, vec![(a.hist.values(), a.color), (b.hist.values(), b.color)], peak, 64.))
}

pub fn network(m: &Metrics, t: &Theme) -> Div {
    let (rx, tx) = m.net_totals(|n| (n.total_received(), n.total_transmitted()));
    rates(
        t,
        "Network",
        Series { name: "Download", hist: &m.rx, color: t.accent, note: Some(format!("{} since boot", bytes(rx as f64))) },
        Series { name: "Upload", hist: &m.tx, color: t.muted, note: Some(format!("{} since boot", bytes(tx as f64))) },
    )
}

pub fn disk(m: &Metrics, t: &Theme) -> Div {
    rates(
        t,
        "Disk activity",
        Series { name: "Read", hist: &m.rd, color: t.accent, note: None },
        Series { name: "Write", hist: &m.wr, color: t.muted, note: None },
    )
}

/// Per-interface rates and totals, busiest first. Loopback and never-used interfaces are hidden.
pub fn interfaces(m: &Metrics, t: &Theme) -> Div {
    let mut nets: Vec<_> = m
        .nets
        .iter()
        .flat_map(|n| n.iter())
        .map(|(name, d)| (name.to_string_lossy().into_owned(), d))
        .filter(|(name, d)| !name.starts_with("lo") && d.total_received() + d.total_transmitted() > 0)
        .collect();
    nets.sort_by_key(|(_, d)| std::cmp::Reverse(d.total_received() + d.total_transmitted()));

    let cell = |text: String, w: f32| div().w(px(w)).flex().justify_end().child(num(text));
    let head = |text: &'static str, w: f32| {
        div().w(px(w)).flex().justify_end().text_size(px(10.)).text_color(rgb(t.dim)).child(text)
    };
    let line = |i: usize| {
        div()
            .flex()
            .items_center()
            .gap(px(12.))
            .px(px(10.))
            .h(px(28.))
            .rounded(px(5.))
            .when(i % 2 == 1, |d| d.bg(t.wash(0x06)))
    };

    panel(t, "Interfaces", format!("{} in use", nets.len()))
        .gap(px(8.))
        .child(
            line(0)
                .h(px(20.))
                .child(div().flex_1().text_size(px(10.)).text_color(rgb(t.dim)).child("Name"))
                .child(head("Down", 96.))
                .child(head("Up", 96.))
                .child(head("Received", 96.))
                .child(head("Sent", 96.)),
        )
        .children(nets.iter().enumerate().map(|(i, (name, d))| {
            line(i + 1)
                .child(div().flex_1().min_w_0().truncate().text_size(px(12.)).child(name.clone()))
                .child(cell(format!("{}/s", bytes(d.received() as f64)), 96.).text_color(rgb(t.accent)))
                .child(cell(format!("{}/s", bytes(d.transmitted() as f64)), 96.))
                .child(cell(bytes(d.total_received() as f64), 96.).text_color(rgb(t.muted)))
                .child(cell(bytes(d.total_transmitted() as f64), 96.).text_color(rgb(t.muted)))
        }))
        .when(nets.is_empty(), |p| p.child(empty(t, "No active interfaces.")))
}
