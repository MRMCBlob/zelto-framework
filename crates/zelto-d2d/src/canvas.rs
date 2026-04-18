use zelto_ir::style::Color;

/// High-level draw context passed to `<Canvas>` draw functions.
/// Wraps Direct2D operations — implemented in Phase 3.
pub struct D2DCanvas {
    pub width: f32,
    pub height: f32,
}

impl D2DCanvas {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    /// Fill a rectangle.
    pub fn fill_rect(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        // TODO Phase 3: ID2D1RenderTarget::FillRectangle
        let _ = (x, y, w, h, color);
    }

    /// Stroke a rectangle outline.
    pub fn stroke_rect(&self, x: f32, y: f32, w: f32, h: f32, color: Color, stroke: f32) {
        // TODO Phase 3: ID2D1RenderTarget::DrawRectangle
        let _ = (x, y, w, h, color, stroke);
    }

    /// Fill an ellipse.
    pub fn fill_ellipse(&self, cx: f32, cy: f32, rx: f32, ry: f32, color: Color) {
        // TODO Phase 3: ID2D1RenderTarget::FillEllipse
        let _ = (cx, cy, rx, ry, color);
    }

    /// Draw text using DirectWrite.
    pub fn draw_text(&self, text: &str, x: f32, y: f32, size: f32, color: Color) {
        // TODO Phase 3: IDWriteTextLayout + DrawTextLayout
        let _ = (text, x, y, size, color);
    }
}
