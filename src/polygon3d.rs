use std::sync::Arc;

pub struct Polygon3D {
    pub surfaces: Vec<Surface3D>
}

impl Polygon3D {
    pub fn new(surfaces: Vec<Surface3D>) -> Self {
        Self { surfaces }
    }

    pub fn draw(self) {
        for surface in self.surfaces {
            surface.draw()
        }
    }
}

pub struct Surface3D {
    vertices: Vec<(u16, u16, u16)>
}

impl Surface3D {
    pub fn new(vertices: Vec<(u16, u16, u16)>) -> Self {
        Self { vertices }
    }

    pub fn draw(self) {
        for vertex in self.vertices {
            Arc::new(vertex);
        }
    }
}
