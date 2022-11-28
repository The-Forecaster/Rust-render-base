fn main() {
    println!("Hello, world!");
}

struct Coordinate {
    x: u16,
    y: u16
}

impl Coordinate {
    fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

struct Quad {
    x: Coordinate,
    y: Coordinate,
    z: Coordinate,
    a: Coordinate
}

impl Quad {
    fn new(x: Coordinate, y: Coordinate, z: Coordinate, a: Coordinate) -> Self {
        Self { x, y, z, a }
    }

    fn rect(start: Coordinate, width: u16, height: u16) -> Self {
        Self { x: Coordinate::new(start.x, start.y), y: Coordinate::new(start.x + height, start.y), z: Coordinate::new(start.x, start.y + width), a: Coordinate::new(&start.x + height, start.y + width)}
    }
}

struct Square {
    start: Coordinate,
    length: u16
}
