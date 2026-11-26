//! System sampling and rolling history. No UI here.

use std::collections::VecDeque;

use sysinfo::{Components, Disk, Disks, Networks, ProcessRefreshKind, ProcessesToUpdate, System};

use crate::format::pct;

pub const HISTORY: usize = 120;

#[derive(Default)]
pub struct History(pub VecDeque<f32>);

impl History {
    pub fn push(&mut self, v: f32) {
        if self.0.len() == HISTORY {
            self.0.pop_front();
        }
        self.0.push_back(v);
    }
    pub fn now(&self) -> f32 {
        self.0.back().copied().unwrap_or(0.)
    }
    pub fn values(&self) -> Vec<f32> {
        self.0.iter().copied().collect()
    }
    pub fn avg(&self) -> f32 {
        if self.0.is_empty() { 0. } else { self.0.iter().sum::<f32>() / self.0.len() as f32 }
    }
    pub fn peak(&self) -> f32 {
        self.0.iter().copied().fold(0., f32::max)
    }
}

pub struct Proc {
    pub name: String,
    pub pid: u32,
    pub cpu: f32,
    pub mem: u64,
}

pub struct Metrics {
    pub sys: System,
    // None when the platform refuses the source; panels show their empty state.
    pub comps: Option<Components>,
    pub disks: Option<Disks>,
    pub nets: Option<Networks>,
    pub cpu: History,
    /// One history per logical core, same order as `sys.cpus()`.
    pub cores: Vec<History>,
    pub mem: History,
    pub rx: History,
    pub tx: History,
    pub rd: History,
    pub wr: History,
    pub procs: Vec<Proc>,
    pub proc_count: usize,
    pub host: String,
    pub os: String,
    pub kernel: String,
    pub brand: String,
    pub physical: usize,
}

impl Metrics {
    pub fn new() -> Self {
        let mut sys = System::new().expect("system info unavailable");
        sys.refresh_cpu_all();
        let brand = sys.cpus().first().map(|c| c.brand().trim().to_string()).unwrap_or_default();
        let mut m = Self {
            sys,
            comps: Components::new_with_refreshed_list().ok(),
            disks: Disks::new_with_refreshed_list().ok(),
            nets: Networks::new_with_refreshed_list().ok(),
            cpu: History::default(),
            cores: Vec::new(),
            mem: History::default(),
            rx: History::default(),
            tx: History::default(),
            rd: History::default(),
            wr: History::default(),
            procs: Vec::new(),
            proc_count: 0,
            host: System::host_name().unwrap_or_else(|_| "This Mac".into()),
            os: System::long_os_version().unwrap_or_default(),
            kernel: format!("Darwin {}", System::kernel_version().unwrap_or_default()),
            brand,
            physical: System::physical_core_count().unwrap_or(0),
        };
        m.sample();
        m
    }

    // ponytail: runs on the UI thread (~10 ms with processes); move to background_executor if frames hitch.
    pub fn sample(&mut self) {
        self.sys.refresh_cpu_all();
        self.sys.refresh_memory();
        self.sys.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::nothing().with_cpu().with_memory(),
        );
        self.comps.as_mut().map(|c| c.refresh(false));
        self.disks.as_mut().map(|d| d.refresh(false));
        self.nets.as_mut().map(|n| n.refresh(false));

        self.cpu.push(self.sys.global_cpu_usage());
        let cpus = self.sys.cpus();
        self.cores.resize_with(cpus.len(), History::default);
        self.cores.iter_mut().zip(cpus).for_each(|(h, c)| h.push(c.usage()));
        self.mem.push(pct(self.sys.used_memory(), self.sys.total_memory()));

        // Deltas since the last refresh; sampling runs once per second, so these are bytes/s.
        let (rx, tx) = self.net_totals(|n| (n.received(), n.transmitted()));
        self.rx.push(rx as f32);
        self.tx.push(tx as f32);
        let (rd, wr) = self
            .volumes()
            .iter()
            .fold((0, 0), |(r, w), d| (r + d.usage().read_bytes, w + d.usage().written_bytes));
        self.rd.push(rd as f32);
        self.wr.push(wr as f32);

        // Every process, unsorted; panels sort per view.
        self.procs = self
            .sys
            .processes()
            .values()
            .map(|p| Proc {
                name: p.name().to_string_lossy().into_owned(),
                pid: p.pid().as_u32(),
                cpu: p.cpu_usage(),
                mem: p.memory(),
            })
            .collect();
        self.proc_count = self.procs.len();
    }

    /// Sum over non-loopback interfaces.
    pub fn net_totals(&self, f: impl Fn(&sysinfo::NetworkData) -> (u64, u64)) -> (u64, u64) {
        self.nets
            .iter()
            .flat_map(|n| n.iter())
            .filter(|(name, _)| !name.to_string_lossy().starts_with("lo"))
            .map(|(_, n)| f(n))
            .fold((0, 0), |(a, b), (x, y)| (a + x, b + y))
    }

    /// Real volumes only: drop macOS system sub-mounts and mirrors of the same container.
    pub fn volumes(&self) -> Vec<&Disk> {
        let mut seen = Vec::new();
        self.disks
            .iter()
            .flat_map(|d| d.iter())
            .filter(|d| {
                let m = d.mount_point().to_string_lossy();
                d.total_space() > 0 && !m.starts_with("/System/Volumes") && !m.starts_with("/private/var")
            })
            .filter(|d| {
                let key = (d.name().to_os_string(), d.total_space());
                !seen.contains(&key) && {
                    seen.push(key);
                    true
                }
            })
            .collect()
    }

    /// Sensors with a valid reading, hottest first.
    pub fn temps(&self) -> Vec<(String, f32)> {
        let mut t: Vec<_> = self
            .comps
            .iter()
            .flat_map(|c| c.iter())
            .filter_map(|c| c.temperature().filter(|t| t.is_finite() && *t > 0.).map(|t| (c.label().to_string(), t)))
            .collect();
        t.sort_by(|a, b| b.1.total_cmp(&a.1));
        t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn history_rolls() {
        let mut h = History::default();
        (0..HISTORY + 5).for_each(|i| h.push(i as f32));
        assert_eq!(h.0.len(), HISTORY);
        assert_eq!(h.0[0], 5.);
        assert_eq!(h.now(), (HISTORY + 4) as f32);
        let h2 = History(VecDeque::from([10., 30.]));
        assert_eq!((h2.avg(), h2.peak()), (20., 30.));
    }
}
