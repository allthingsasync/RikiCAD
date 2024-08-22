pub struct Meta {
    layer: Layer,
    line_width: LineWidth,
    line_type: LineType,
    color: Color
}

pub struct Layer {
    name: String,
    line_width: LineWidth,
    line_type: LineType,
    color: Color
}

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

pub struct LineWidth {}
pub struct LineType {}
