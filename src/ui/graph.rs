//! Rolling history graph.

use gpui::{Bounds, Div, PathBuilder, canvas, div, fill, point, prelude::*, px, rgb, size};

use crate::{metrics::HISTORY, theme::Theme, ui::kit::MONO};

/// Newest sample at the right edge, quarter gridlines, soft area under the first series,
/// a dot on each live head, and a time axis below.
pub fn graph(t: &Theme, series: Vec<(Vec<f32>, u32)>, max: f32, height: f32) -> Div {
    let axis = div()
        .flex()
        .justify_between()
        .font_family(MONO)
        .text_size(px(10.))
        .text_color(rgb(t.dim))
        .child("2 min")
        .child("1 min")
        .child("now");
    div().flex().flex_col().gap(px(6.)).child(plot(t, series, max, height)).child(axis)
}

/// The graph without its time axis, for small multiples.
pub fn plot(t: &Theme, series: Vec<(Vec<f32>, u32)>, max: f32, height: f32) -> Div {
    let theme = *t;
    div().h(px(height)).w_full().child(
        canvas(
            |_, _, _| {},
            move |b, _, window, _| {
                for i in 0..=4 {
                    let base = i == 4;
                    let y = b.origin.y + b.size.height * (i as f32 / 4.) - px(if base { 1. } else { 0. });
                    let c = theme.wash(if base { 0x1c } else { 0x0c });
                    window.paint_quad(fill(Bounds::new(point(b.origin.x, y), size(b.size.width, px(1.))), c));
                }
                let step = b.size.width / (HISTORY - 1) as f32;
                for (n, (data, color)) in series.into_iter().enumerate() {
                    if data.len() < 2 {
                        continue;
                    }
                    let x0 = b.right() - step * (data.len() - 1) as f32;
                    let pts: Vec<_> = data
                        .iter()
                        .enumerate()
                        .map(|(i, v)| point(x0 + step * i as f32, b.bottom() - b.size.height * (v / max).clamp(0., 1.)))
                        .collect();
                    if n == 0 {
                        let mut area = PathBuilder::fill();
                        area.move_to(point(x0, b.bottom()));
                        pts.iter().for_each(|p| area.line_to(*p));
                        area.line_to(point(b.right(), b.bottom()));
                        area.close();
                        if let Ok(p) = area.build() {
                            window.paint_path(p, theme.tint(color, if theme.dark { 0x22 } else { 0x2a }));
                        }
                    }
                    let mut stroke = PathBuilder::stroke(px(1.5));
                    stroke.move_to(pts[0]);
                    pts[1..].iter().for_each(|p| stroke.line_to(*p));
                    if let Ok(p) = stroke.build() {
                        window.paint_path(p, rgb(color));
                    }
                    let head = pts[pts.len() - 1];
                    window.paint_quad(
                        fill(Bounds::new(point(head.x - px(3.), head.y - px(3.)), size(px(6.), px(6.))), rgb(color))
                            .corner_radii(px(3.)),
                    );
                }
            },
        )
        .size_full(),
    )
}
