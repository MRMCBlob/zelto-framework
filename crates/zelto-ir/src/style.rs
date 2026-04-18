#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    pub color: Option<Color>,
    pub background: Option<Color>,
    pub font_size: Option<f32>,
    pub font_weight: Option<FontWeight>,
    pub padding: Option<Edges>,
    pub margin: Option<Edges>,
    pub width: Option<Dimension>,
    pub height: Option<Dimension>,
    pub border_radius: Option<f32>,
    pub border_color: Option<Color>,
    pub border_width: Option<f32>,
    pub flex_direction: Option<FlexDirection>,
    pub align_items: Option<AlignItems>,
    pub justify_content: Option<JustifyContent>,
    pub gap: Option<f32>,
    pub flex_grow: Option<f32>,
    pub flex_shrink: Option<f32>,
    pub opacity: Option<f32>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            color: None,
            background: None,
            font_size: None,
            font_weight: None,
            padding: None,
            margin: None,
            width: None,
            height: None,
            border_radius: None,
            border_color: None,
            border_width: None,
            flex_direction: None,
            align_items: None,
            justify_content: None,
            gap: None,
            flex_grow: None,
            flex_shrink: None,
            opacity: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    pub const WHITE: Self = Self::rgb(255, 255, 255);
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const TRANSPARENT: Self = Self::rgba(0, 0, 0, 0);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Edges {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Edges {
    pub const fn all(v: f32) -> Self {
        Self { top: v, right: v, bottom: v, left: v }
    }
    pub const fn xy(x: f32, y: f32) -> Self {
        Self { top: y, right: x, bottom: y, left: x }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dimension {
    Px(f32),
    Percent(f32),
    Auto,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Thin,
    Light,
    Regular,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}

impl FontWeight {
    pub fn to_numeric(self) -> u16 {
        match self {
            Self::Thin => 100,
            Self::Light => 300,
            Self::Regular => 400,
            Self::Medium => 500,
            Self::SemiBold => 600,
            Self::Bold => 700,
            Self::ExtraBold => 800,
            Self::Black => 900,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignItems {
    Start,
    Center,
    End,
    Stretch,
    Baseline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JustifyContent {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}
