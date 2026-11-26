//! The app mark (dial with needle), drawn live so it follows the active theme.
//! Same geometry as assets/icon.svg, on its 100-unit grid.

use gpui::{Bounds, Div, PathBuilder, canvas, div, fill, point, prelude::*, px, rgb, size};

use crate::{theme::Theme, ui::dial::arc};

pub fn mark(t: &Theme, side: f32) -> Div {
    let theme = *t;
    div().size(px(side)).flex_none().child(
        canvas(
            |_, _, _| {},
            move |b, _, window, _| {
                let s = f32::from(b.size.width) / 100.;
                let at = |x: f32, y: f32| point(b.origin.x + px(x * s), b.origin.y + px(y * s));
                window.paint_quad(
                    fill(Bounds::new(at(2., 2.), size(px(96. * s), px(96. * s))), rgb(theme.panel))
                        .corner_radii(px(22. * s))
                        .border_widths(px(1.))
                        .border_color(rgb(theme.line)),
                );
                let c = at(50., 54.);
                // 150 deg start, 240 deg sweep; the mark always reads ~68%.
                if let Some(p) = arc(c, 30. * s, 150., 390., 7. * s) {
                    window.paint_path(p, rgb(theme.track));
                }
                if let Some(p) = arc(c, 30. * s, 150., 313., 7. * s) {
                    window.paint_path(p, rgb(theme.accent));
                }
                let mut needle = PathBuilder::stroke(px(3. * s));
                needle.move_to(c);
                needle.line_to(at(63.7, 39.4));
                if let Ok(p) = needle.build() {
                    window.paint_path(p, rgb(theme.text));
                }
                let r = 4.5 * s;
                window.paint_quad(
                    fill(Bounds::new(point(c.x - px(r), c.y - px(r)), size(px(r * 2.), px(r * 2.))), rgb(theme.text))
                        .corner_radii(px(r)),
                );
            },
        )
        .size_full(),
    )
}
