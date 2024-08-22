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

    pub fn move_by(&self, x: f64, y: f64) -> C {
        C { x: self.x + x, y: self.y + y }
    }
}

pub struct BBox {
    min_pt: C,
    max_pt: C
}

impl BBox {
    pub fn from_values(x1: f64, y1: f64, x2: f64, y2: f64) -> BBox {
        let min_x = x1.min(x2);
        let min_y = y1.min(y2);
        let max_x = x1.max(x2);
        let max_y = y1.max(y2);
        BBox { min_pt: C::new(min_x, min_y), max_pt: C::new(max_x, max_y) }
    }

    pub fn from_coordinates(pt1: &C, pt2: &C) -> BBox {
        BBox::from_values(pt1.x, pt1.y, pt2.x, pt2.y)
    }

    pub fn intersects(&self, other: &BBox) -> bool { unimplemented!() }
    pub fn contains(&self, other: &BBox) -> bool { unimplemented!() }
    pub fn intersection(&self, other: &BBox) -> BBox { unimplemented!() }
    pub fn merge(&self, other: &BBox) -> BBox { unimplemented!() }
}

