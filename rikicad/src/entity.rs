use crate::base::C;

pub struct GeoLine {
    start: C,
    end: C
}

pub struct GeoCircle {
    center: C,
    radius: f64
}

pub enum EntityGeometry {
    Line(GeoLine),
    Circle(GeoCircle),
}

pub struct EntityContainer {}
