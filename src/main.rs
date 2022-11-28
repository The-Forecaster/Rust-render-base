fn main() {
    println!("Hello, world!");
}

struct Coordinate {
    x: u32,
    y: u32
}

struct Quad {
    x1: Coordinate,
    x2: Coordinate,
    y1: Coordinate,
    y2: Coordinate
}

struct Cube {
    start: Coordinate,
    length: u32,
    width: u32
}

