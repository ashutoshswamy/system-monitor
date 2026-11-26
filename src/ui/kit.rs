//! Small themed building blocks shared by every panel.

use gpui::{BoxShadow, Div, FontWeight, Rgba, SharedString, Styled, div, point, prelude::*, px, relative, rgb};

use crate::theme::Theme;

pub const MONO: &str = "Menlo";
pub const SANS: &str = "Avenir Next";

pub fn label(t: &Theme, text: impl Into<SharedString>) -> Div {
    div().text_size(px(11.)).text_color(rgb(t.muted)).child(text.into())
}

pub fn num(text: impl Into<SharedString>) -> Div {
    div().font_family(MONO).text_size(px(12.)).child(text.into())
}

pub fn empty(t: &Theme, msg: &'static str) -> Div {
    div().py(px(8.)).text_size(px(12.)).text_color(rgb(t.dim)).child(msg)
}

pub fn panel(t: &Theme, title: &'static str, aside: impl Into<SharedString>) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(16.))
        .p(px(20.))
        .min_w_0()
        .bg(rgb(t.panel))
        .border_1()
        .border_color(rgb(t.line))
        .rounded(px(10.))
        .shadow(vec![BoxShadow {
            color: t.shadow(),
            offset: point(px(0.), px(8.)),
            blur_radius: px(24.),
            spread_radius: px(-8.),
        }])
        .child(
            div()
                .flex()
                .justify_between()
                .items_baseline()
                .gap(px(12.))
                .child(div().text_size(px(13.)).font_weight(FontWeight::SEMIBOLD).child(title))
                .child(label(t, aside).min_w_0().truncate()),
        )
}

pub fn readout(t: &Theme, value: impl Into<SharedString>, unit: &'static str, color: u32, size: f32) -> Div {
    div()
        .flex()
        .items_baseline()
        .gap(px(5.))
        .child(
            div()
                .font_family(MONO)
                .text_size(px(size))
                .line_height(px(size * 1.1))
                .text_color(rgb(color))
                .child(value.into()),
        )
        .child(div().text_size(px(12.)).text_color(rgb(t.muted)).child(unit))
}

pub fn meter(t: &Theme, pct: f32, color: u32) -> Div {
    div()
        .h(px(4.))
        .w_full()
        .rounded(px(2.))
        .bg(rgb(t.track))
        .child(div().h_full().rounded(px(2.)).bg(rgb(color)).w(relative((pct / 100.).clamp(0., 1.))))
}

pub fn row(left: impl IntoElement, right: impl IntoElement) -> Div {
    div().flex().justify_between().items_baseline().gap(px(12.)).child(left).child(right)
}

pub fn swatch(color: Rgba) -> Div {
    div().flex_none().size(px(8.)).rounded(px(2.)).bg(color)
}

pub fn legend(t: &Theme, color: Rgba, name: &'static str, value: String) -> Div {
    div()
        .flex()
        .items_center()
        .gap(px(8.))
        .child(swatch(color))
        .child(label(t, name).flex_1())
        .child(num(value))
}

pub fn stat(t: &Theme, name: &'static str, value: String) -> Div {
    div().flex().items_baseline().gap(px(6.)).child(label(t, name)).child(num(value))
}

/// flex-grow weights so rows share one column rhythm regardless of content width.
pub trait Grow: Styled + Sized {
    fn grow(mut self, weight: f32) -> Self {
        let s = self.style();
        s.flex_grow = Some(weight);
        s.flex_shrink = Some(1.);
        s.flex_basis = Some(relative(0.).into());
        self
    }
}
impl Grow for Div {}
