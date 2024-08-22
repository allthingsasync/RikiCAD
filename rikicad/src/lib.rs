mod kernel {
    pub struct C {
        x: f64,
        y: f64
    }

    impl C {
        pub fn new(x: f64, y: f64) -> C {
            C { x, y }
        }

        pub fn scale(&self, factor: f64) -> C {
            C { x: self.x * factor, y: self.y * factor }
        }

        pub fn scale_xy(&self, x_factor: f64, y_factor: f64) -> C {
            C { x: self.x * x_factor, y: self.y * y_factor }
        }
    }

    pub struct BBox {
        min_pt: C,
        max_pt: C
    }

    impl BBox {
        pub fn from_values(x1: f64, y1: f64, x2: f64, y2: f64) -> BBox {
            unimplemented!()
        }

        pub fn from_coordinates(pt1: &C, pt2: &C) -> BBox {
            unimplemented!()
        }

        pub fn intersects(&self, other: &BBox) -> bool { unimplemented!() }
        pub fn contains(&self, other: &BBox) -> bool { unimplemented!() }
        pub fn intersection(&self, other: &BBox) -> BBox { unimplemented!() }
        pub fn merge(&self, other: &BBox) -> BBox { unimplemented!() }
    }

    pub struct MouseEvent {}
    pub struct ScrollEvent {}
    pub struct KeyEvent {}
    pub trait IPainter {
        fn draw_line(x1: u16, y1: u16, x2: u16, y1: u16);
        fn draw_circle(x1: u16, y1: u16, radius: u16);
    }

    pub struct Document {}
    pub struct EntityStore {}
    pub struct EntityContainer {}
    pub struct GeoLine {}
    pub struct GeoCircle {}
    pub struct Meta {}
    pub struct Layer {}
    pub struct LineWidth {}
    pub struct LineType {}

    pub enum EntityGeometry {
        Line(GeoLine),
        Circle(GeoCircle),
    }
}
