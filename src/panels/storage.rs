use gpui::{Div, div, prelude::*, px, rgb};
use sysinfo::DiskKind;

use crate::{
    format::{bytes, pct},
    metrics::Metrics,
    theme::Theme,
    ui::kit::{empty, label, meter, num, panel, row},
};

pub fn render(m: &Metrics, t: &Theme) -> Div {
    let vols: Vec<_> = m
        .volumes()
        .into_iter()
        .map(|d| {
            let (total, free) = (d.total_space(), d.available_space());
            let used = pct(total - free, total);
            let mount = d.mount_point().to_string_lossy().to_string();
            let name = if mount == "/" { "Macintosh HD".into() } else { mount.rsplit('/').next().unwrap_or(&mount).to_string() };
            let kind = match d.kind() {
                DiskKind::SSD => " SSD",
                DiskKind::HDD => " HDD",
                DiskKind::Unknown(_) => "",
            };
            let tags = format!(
                "{}{kind}{}",
                d.file_system().to_string_lossy().to_uppercase(),
                if d.is_removable() { "  removable" } else { "" }
            );
            div()
                .flex()
                .flex_col()
                .gap(px(8.))
                .child(row(
                    div()
                        .flex()
                        .items_baseline()
                        .gap(px(10.))
                        .min_w_0()
                        .child(div().text_size(px(13.)).truncate().child(name))
                        .child(label(t, tags).text_color(rgb(t.dim))),
                    num(format!("{used:.0}%")).text_color(rgb(t.level(used))),
                ))
                .child(meter(t, used, t.level(used)))
                .child(row(
                    label(t, format!("{} used", bytes((total - free) as f64))),
                    label(t, format!("{} free of {}", bytes(free as f64), bytes(total as f64))),
                ))
        })
        .collect();
    let none = vols.is_empty();
    panel(t, "Storage", format!("{} volumes", vols.len()))
        .gap(px(20.))
        .children(vols)
        .when(none, |p| p.child(empty(t, "No mounted volumes found.")))
}
