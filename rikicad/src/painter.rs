pub trait IPainter {
    fn draw_line(x1: u16, y1: u16, x2: u16, y1: u16);
    fn draw_circle(x1: u16, y1: u16, radius: u16);
}