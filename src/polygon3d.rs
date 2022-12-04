use beryllium::GlWindow;

pub struct Polygon3D {
    pub surfaces: Vec<Surface>
}

pub struct World {
    pub(crate) objects: Vec<Polygon3D>,
    pub(crate) window: GlWindow,
    pub(crate) camera: (u16, u16, u16)
}

impl World {
    pub fn add(&mut self, object: Polygon3D) {
        self.objects.push(object)
    }

    pub fn render(&self) {
        for object in &self.objects {
            object.draw(&self)
        }
    }
}

impl Polygon3D {
    pub fn new(surfaces: Vec<Surface>) -> Self {
        Self { surfaces }
    }

    pub fn draw(&self, world: &World) {
        for surface in &self.surfaces {
            surface.draw(world)
        }
    }
}

pub struct Surface {
    vertices: Vec<(u16, u16, u16)>
}

impl Surface {
    pub fn new(vertices: Vec<(u16, u16, u16)>) -> Self {
        Self { vertices }
    }

    pub fn draw(&self, _world: &World) {}
}
