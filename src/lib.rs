pub type Point = [u16; 2]; // point should be ordered x, y

pub type Quad = [Point; 4]; // point 1, point 2, point 3, point 4

pub type Rect = (Point, [u16; 2]); // Top left, width, height
