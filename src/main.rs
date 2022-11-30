use polygon3d::{Polygon3D, Surface3D};

mod polygon3d;

fn main() {
    let cube = Polygon3D::new(
        vec![
                Surface3D::new(vec![
                    (0, 0, 0),
                    (100, 0, 0),
                    (0, 100, 0),
                    (100, 100, 0)
                ]),
                Surface3D::new(vec![
                    (0, 0, 0),
                    (0, 100, 0),
                    (0, 0, 100),
                    (0, 100, 100)
                ]),
                Surface3D::new(vec![
                    (0, 0, 0),
                    (0, 100, 0),
                    (0, 0, 100),
                    (0, 100, 100)
                ]),
                Surface3D::new(vec![
                    (100, 0, 0),
                    (100, 100, 0),
                    (100, 0, 100),
                    (100, 100, 100)
                ]),
                Surface3D::new(vec![
                    (0, 100, 0),
                    (100, 100, 0),
                    (0, 100, 100),
                    (100, 100, 100)
                ]),
                Surface3D::new(vec![
                    (0, 0, 100),
                    (100, 0, 100),
                    (0, 100, 100),
                    (100, 100, 100)
                ])
            ]);

    cube.draw()
}
