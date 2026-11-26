use std::cmp::Ordering;

use gpui::{Context, Div, div, prelude::*, px, rgb};

use crate::{
    app::Dashboard,
    format::bytes,
    metrics::{Metrics, Proc},
    theme::Theme,
    ui::kit::{empty, meter, num, panel},
};

#[derive(Clone, Copy, PartialEq)]
pub enum Col {
    Name,
    Pid,
    Cpu,
    Mem,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Sort {
    pub col: Col,
    pub desc: bool,
}

impl Default for Sort {
    fn default() -> Self {
        Sort { col: Col::Cpu, desc: true }
    }
}

impl Sort {
    /// Same column flips direction; a new column starts at its natural order (names A-Z, numbers biggest first).
    pub fn click(self, col: Col) -> Sort {
        if col == self.col { Sort { col, desc: !self.desc } } else { Sort { col, desc: col != Col::Name } }
    }

    fn cmp(self, a: &Proc, b: &Proc) -> Ordering {
        let o = match self.col {
            Col::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            Col::Pid => a.pid.cmp(&b.pid),
            Col::Cpu => a.cpu.total_cmp(&b.cpu),
            Col::Mem => a.mem.cmp(&b.mem),
        };
        // PID breaks ties so rows don't shuffle between refreshes.
        let o = if self.desc { o.reverse() } else { o };
        o.then(a.pid.cmp(&b.pid))
    }

    fn by(self) -> &'static str {
        match self.col {
            Col::Name => "name",
            Col::Pid => "PID",
            Col::Cpu => "CPU",
            Col::Mem => "memory",
        }
    }
}

/// Sorted process table. `limit: None` shows every process.
pub fn render(m: &Metrics, t: &Theme, sort: Sort, limit: Option<usize>, cx: &mut Context<Dashboard>) -> Div {
    let cores = m.sys.cpus().len().max(1) as f32;
    let mut procs: Vec<&Proc> = m.procs.iter().collect();
    procs.sort_by(|a, b| sort.cmp(a, b));
    procs.truncate(limit.unwrap_or(usize::MAX));

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
    let right = |d: Div, w: f32| d.w(px(w)).flex().justify_end();
    let head = |i: usize, col: Col, text: &'static str| {
        let on = sort.col == col;
        div()
            .id(("sort", i))
            .flex()
            .gap(px(3.))
            .cursor_pointer()
            .text_size(px(10.))
            .text_color(rgb(if on { t.text } else { t.dim }))
            .hover(|s| s.text_color(rgb(t.text)))
            .on_click(cx.listener(move |d, _, _, cx| {
                d.sort = d.sort.click(col);
                cx.notify();
            }))
            .child(text)
            .when(on, |d| d.child(if sort.desc { "▾" } else { "▴" }))
    };

    // ponytail: one div row per process, fine for a few hundred; switch to uniform_list if thousands hitch.
    let rows = procs.iter().enumerate().map(|(i, p)| {
        // Process CPU is per-core percent; the bar shows share of the whole machine.
        let c = t.level(p.cpu / cores * 100.);
        line(i + 1)
            .child(div().flex_1().min_w_0().truncate().text_size(px(12.)).child(p.name.clone()))
            .child(right(num(p.pid.to_string()), 56.).text_color(rgb(t.muted)))
            .child(
                right(div().items_center().gap(px(10.)), 132.)
                    .child(div().w(px(60.)).child(meter(t, p.cpu / cores * 100., c)))
                    .child(right(num(format!("{:.1}%", p.cpu)), 60.)),
            )
            .child(right(num(bytes(p.mem as f64)), 76.).text_color(rgb(t.muted)))
    });

    let aside = if limit.is_some() {
        format!("top {} of {} by {}", procs.len(), m.proc_count, sort.by())
    } else {
        format!("{} by {}", m.proc_count, sort.by())
    };
    panel(t, "Processes", aside)
        .gap(px(8.))
        .child(
            line(0)
                .h(px(20.))
                .child(div().flex_1().child(head(0, Col::Name, "Name")))
                .child(right(div(), 56.).child(head(1, Col::Pid, "PID")))
                .child(right(div(), 132.).child(head(2, Col::Cpu, "CPU")))
                .child(right(div(), 76.).child(head(3, Col::Mem, "Memory"))),
        )
        .child(div().flex().flex_col().children(rows))
        .when(m.procs.is_empty(), |p| p.child(empty(t, "Process list unavailable.")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_and_toggles() {
        let p = |name: &str, pid, cpu, mem| Proc { name: name.into(), pid, cpu, mem };
        let v = [p("b", 2, 5., 100), p("A", 1, 9., 50), p("c", 3, 5., 300)];
        let order = |s: Sort| {
            let mut r: Vec<_> = v.iter().collect();
            r.sort_by(|a, b| s.cmp(a, b));
            r.iter().map(|p| p.pid).collect::<Vec<_>>()
        };
        assert_eq!(order(Sort::default()), [1, 2, 3]); // CPU desc, ties by PID
        let name = Sort::default().click(Col::Name);
        assert!(!name.desc);
        assert_eq!(order(name), [1, 2, 3]); // case-insensitive A, b, c
        assert_eq!(order(name.click(Col::Name)), [3, 2, 1]);
        assert_eq!(order(Sort::default().click(Col::Mem)), [3, 2, 1]);
    }
}
