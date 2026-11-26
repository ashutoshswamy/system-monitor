//! Color themes. Every theme keeps the same roles: neutrals, one signal accent, one critical color.

use gpui::{Hsla, Rgba, WindowAppearance, rgba};

#[derive(Clone, Copy)]
pub struct Theme {
    pub name: &'static str,
    pub dark: bool,
    pub bg: u32,
    pub sidebar: u32,
    pub panel: u32,
    pub line: u32,
    pub track: u32,
    pub text: u32,
    pub muted: u32,
    pub dim: u32,
    pub accent: u32,
    pub hot: u32,
}

impl Theme {
    /// Color with alpha (0-255), for fills under lines and secondary segments.
    pub fn tint(&self, hex: u32, alpha: u8) -> Rgba {
        rgba(hex << 8 | alpha as u32)
    }

    /// Hairlines and zebra rows: white on dark themes, black on light ones.
    pub fn wash(&self, alpha: u8) -> Rgba {
        self.tint(if self.dark { 0xffffff } else { 0x000000 }, alpha)
    }

    pub fn shadow(&self) -> Hsla {
        gpui::hsla(0., 0., 0., if self.dark { 0.35 } else { 0.07 })
    }

    /// Signal color for a percentage: accent normally, critical at 90% and up.
    pub fn level(&self, pct: f32) -> u32 {
        if pct >= 90. { self.hot } else { self.accent }
    }

    pub fn heat(&self, celsius: f32) -> u32 {
        if celsius >= 85. { self.hot } else { self.accent }
    }
}

pub const GRAPHITE: Theme = Theme {
    name: "Graphite",
    dark: true,
    bg: 0x0b0c0e,
    sidebar: 0x0f1012,
    panel: 0x141619,
    line: 0x23262b,
    track: 0x1e2125,
    text: 0xebe8e1,
    muted: 0x9196a0,
    dim: 0x5f646d,
    accent: 0xf2a33a,
    hot: 0xef6a55,
};

pub const DAYLIGHT: Theme = Theme {
    name: "Daylight",
    dark: false,
    bg: 0xeef0f3,
    sidebar: 0xe6e9ed,
    panel: 0xfcfcfd,
    line: 0xd9dde3,
    track: 0xe4e7ec,
    text: 0x16181b,
    muted: 0x5b626d,
    dim: 0x868d98,
    accent: 0xb86206,
    hot: 0xc93a24,
};

pub const MIDNIGHT: Theme = Theme {
    name: "Midnight",
    dark: true,
    bg: 0x090d13,
    sidebar: 0x0c1119,
    panel: 0x10161f,
    line: 0x1d2633,
    track: 0x18202b,
    text: 0xe3e9f2,
    muted: 0x8c98aa,
    dim: 0x566275,
    accent: 0x52b8e6,
    hot: 0xf06a6a,
};

pub const MOSS: Theme = Theme {
    name: "Moss",
    dark: true,
    bg: 0x0b0e0c,
    sidebar: 0x0e1210,
    panel: 0x131815,
    line: 0x222b25,
    track: 0x1b221e,
    text: 0xe5ebe4,
    muted: 0x919d93,
    dim: 0x5b665e,
    accent: 0x93cf6c,
    hot: 0xef7b55,
};

pub const DUSK: Theme = Theme {
    name: "Dusk",
    dark: true,
    bg: 0x0d0b12,
    sidebar: 0x110e17,
    panel: 0x17131f,
    line: 0x272133,
    track: 0x201b2a,
    text: 0xebe6f2,
    muted: 0x9a91aa,
    dim: 0x635b72,
    accent: 0xb48cf2,
    hot: 0xf2708a,
};

pub const TIDE: Theme = Theme {
    name: "Tide",
    dark: true,
    bg: 0x081010,
    sidebar: 0x0b1414,
    panel: 0x0f1a1a,
    line: 0x1c2b2b,
    track: 0x162323,
    text: 0xe1eeec,
    muted: 0x8aa19e,
    dim: 0x536a67,
    accent: 0x3fd1b8,
    hot: 0xf0755a,
};

pub const EMBER: Theme = Theme {
    name: "Ember",
    dark: true,
    bg: 0x100b0a,
    sidebar: 0x150f0d,
    panel: 0x1b1412,
    line: 0x2e221e,
    track: 0x261c19,
    text: 0xf2e8e3,
    muted: 0xa8968e,
    dim: 0x6e5e57,
    accent: 0xff8a4c,
    hot: 0xff4f7b,
};

pub const SAND: Theme = Theme {
    name: "Sand",
    dark: false,
    bg: 0xf3efe7,
    sidebar: 0xebe5da,
    panel: 0xfdfbf7,
    line: 0xddd5c7,
    track: 0xe8e1d4,
    text: 0x1f1a14,
    muted: 0x6b6152,
    dim: 0x958a7a,
    accent: 0x2f7d6d,
    hot: 0xc23b22,
};

pub const FROST: Theme = Theme {
    name: "Frost",
    dark: false,
    bg: 0xedf2f7,
    sidebar: 0xe3eaf2,
    panel: 0xfbfcfe,
    line: 0xd3dde8,
    track: 0xdfe7f0,
    text: 0x121a24,
    muted: 0x546273,
    dim: 0x7f8c9c,
    accent: 0x2a6fdb,
    hot: 0xd23c3c,
};

pub const THEMES: [Theme; 9] = [GRAPHITE, DAYLIGHT, MIDNIGHT, MOSS, DUSK, TIDE, EMBER, SAND, FROST];

#[derive(Clone, Copy, PartialEq)]
pub enum Choice {
    /// Follow macOS appearance: Daylight when light, Graphite when dark.
    System,
    Fixed(usize),
}

impl Choice {
    pub fn resolve(self, appearance: WindowAppearance) -> Theme {
        match self {
            Choice::Fixed(i) => THEMES[i],
            Choice::System => match appearance {
                WindowAppearance::Light | WindowAppearance::VibrantLight => DAYLIGHT,
                WindowAppearance::Dark | WindowAppearance::VibrantDark => GRAPHITE,
            },
        }
    }
}
