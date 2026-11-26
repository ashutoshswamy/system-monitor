//! Instrument dial: 240 degree sweep, ten ticks, value arc in the signal color.

use std::f32::consts::PI;

use gpui::{Div, Path, PathBuilder, Pixels, Point, canvas, div, point, prelude::*, px, rgb};

use crate::theme::Theme;

const START: f32 = 150.;
const SWEEP: f32 = 240.;

// ponytail: arc as 2-degree line segments; swap for PathBuilder::arc_to if caps/joins ever look off.
pub fn arc(c: Point<Pixels>, r: f32, from: f32, to: f32, width: f32) -> Option<Path<Pixels>> {
    let mut p = PathBuilder::stroke(px(width));
    let steps = ((to - from) / 2.).ceil().max(1.) as usize;
    for i in 0..=steps {
        let a = (from + (to - from) * i as f32 / steps as f32) * PI / 180.;
        let pt = point(c.x + px(r * a.cos()), c.y + px(r * a.sin()));
        if i == 0 { p.move_to(pt) } else { p.line_to(pt) }
    }
    p.build().ok()
}

pub fn dial(t: &Theme, value: f32, color: u32, diameter: f32) -> Div {
    let theme = *t;
    div().size(px(diameter)).flex_none().child(
        canvas(
            |_, _, _| {},
            move |b, _, window, _| {
                let c = b.center();
                let r = f32::from(b.size.width) / 2. - 16.;
                if let Some(p) = arc(c, r, START, START + SWEEP, 10.) {
                    window.paint_path(p, rgb(theme.track));
                }
                let end = START + SWEEP * (value / 100.).clamp(0., 1.);
                if end > START + 0.5 {
                    if let Some(p) = arc(c, r, START, end, 10.) {
                        window.paint_path(p, rgb(color));
                    }
                }
                for i in 0..=10 {
                    let a = (START + SWEEP * i as f32 / 10.) * PI / 180.;
                    let major = i % 5 == 0;
                    let (r0, r1) = if major { (r + 8., r + 14.) } else { (r + 9., r + 12.) };
                    let mut tick = PathBuilder::stroke(px(1.));
                    tick.move_to(point(c.x + px(r0 * a.cos()), c.y + px(r0 * a.sin())));
                    tick.line_to(point(c.x + px(r1 * a.cos()), c.y + px(r1 * a.sin())));
                    if let Ok(p) = tick.build() {
                        window.paint_path(p, rgb(if major { theme.muted } else { theme.dim }));
                    }
                }
            },
        )
        .size_full(),
    )
}
