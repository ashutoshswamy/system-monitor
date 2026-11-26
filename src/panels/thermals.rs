use gpui::{Div, div, prelude::*, px, rgb};

use crate::{
    metrics::Metrics,
    theme::Theme,
    ui::kit::{empty, label, meter, num, panel, readout, row},
};

const SHOWN: usize = 6;

pub fn render(m: &Metrics, t: &Theme) -> Div {
    let temps = m.temps();
    let count = temps.len();
    let avg = if temps.is_empty() { 0. } else { temps.iter().map(|x| x.1).sum::<f32>() / count as f32 };
    let big = |name: &'static str, v: f32, color: u32| {
        div().flex().flex_col().gap(px(2.)).child(label(t, name)).child(readout(t, format!("{v:.0}"), "°C", color, 26.))
    };

    panel(t, "Thermals", format!("{count} sensors"))
        .when_some(temps.first().map(|x| x.1), |p, peak| {
            p.child(div().flex().gap(px(28.)).child(big("Peak", peak, t.heat(peak))).child(big("Average", avg, t.text)))
        })
        .child(div().flex().flex_col().gap(px(12.)).children(temps.into_iter().take(SHOWN).map(|(name, c)| {
            div()
                .flex()
                .flex_col()
                .gap(px(6.))
                .child(row(label(t, name).min_w_0().truncate(), num(format!("{c:.0}°C")).text_color(rgb(t.heat(c)))))
                // Scale: 110 C is the throttle ceiling on current Macs.
                .child(meter(t, c / 110. * 100., t.heat(c)))
        })))
        .when(count == 0, |p| p.child(empty(t, "No temperature sensors exposed on this machine.")))
}
