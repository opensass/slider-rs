#[derive(Clone, PartialEq, Default)]
pub enum Cursor {
    #[default]
    Round,
    Square,
    Custom(&'static str),
}

impl Cursor {
    pub fn to_style(&self, disabled: bool, color: &Color) -> String {
        let base_color = if disabled {
            "#aaa"
        } else {
            color.to_color_code()
        };
        let base_style = match self {
            Cursor::Round => format!(
                "width: 16px; height: 16px; background: {}; border-radius: 50%;",
                base_color
            ),
            Cursor::Square => format!(
                "width: 16px; height: 16px; background: {}; border-radius: 0;",
                base_color
            ),
            Cursor::Custom(s) => s.to_string(),
        };
        format!(
            "{} cursor: pointer; transition: background 0.3s;",
            base_style
        )
    }
}

#[derive(Clone, PartialEq, Default)]
pub enum Size {
    #[default]
    None,
    Md,
    Sm,
    Lg,
    Custom(&'static str),
}

impl Size {
    pub fn to_style(&self) -> &'static str {
        match self {
            Size::None => "",
            Size::Sm => "height: 10px; width: 120px;",
            Size::Md => "height: 20px; width: 220px;",
            Size::Lg => "height: 35px; width: 320px;",
            Size::Custom(s) => s,
        }
    }
}

#[derive(Clone, PartialEq, Default)]
pub enum Color {
    #[default]
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
    Info,
    Light,
    Dark,
    Muted,
    White,
    Custom(&'static str),
}

impl Color {
    pub fn to_color_code(&self) -> &'static str {
        match self {
            Color::Primary => "#0d6efd",
            Color::Secondary => "#6c757d",
            Color::Success => "#198754",
            Color::Warning => "#ffc107",
            Color::Danger => "#dc3545",
            Color::Info => "#0dcaf0",
            Color::Light => "#f8f9fa",
            Color::Dark => "#212529",
            Color::Muted => "#6c757d",
            Color::White => "#ffffff",
            Color::Custom(s) => s,
        }
    }
}

#[derive(Clone, PartialEq, Default)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

impl Orientation {
    pub fn is_vertical(&self) -> bool {
        matches!(self, Orientation::Vertical)
    }

    pub fn to_style(&self) -> &'static str {
        if self.is_vertical() {
            "writing-mode: vertical-lr;"
        } else {
            ""
        }
    }

    pub fn to_orient(&self) -> Option<&'static str> {
        if self.is_vertical() {
            Some("vertical")
        } else {
            None
        }
    }
}

#[derive(Clone, PartialEq, Default)]
pub enum Width {
    #[default]
    Auto,
    Px(u32),
    Percent(u8),
    Custom(&'static str),
}

impl Width {
    pub fn to_style(&self) -> String {
        match self {
            Width::Auto => "width: auto;".to_string(),
            Width::Px(p) => format!("width: {}px;", p),
            Width::Percent(p) => format!("width: {}%;", p),
            Width::Custom(s) => format!("width: {};", s),
        }
    }
}

#[derive(Clone, PartialEq, Default)]
pub enum Height {
    #[default]
    Auto,
    Px(u32),
    Custom(&'static str),
}

impl Height {
    pub fn to_style(&self) -> String {
        match self {
            Height::Auto => "height: auto;".to_string(),
            Height::Px(p) => format!("height: {}px;", p),
            Height::Custom(s) => format!("height: {};", s),
        }
    }
}
