use beryllium::{InitFlags, SDL, GlProfile, SdlGlAttr, WindowPosition, WindowFlags, Event};
use polygon3d::{Polygon3D, Surface, World};

mod polygon3d;

fn main() -> Result<(), String> {

    // Setup stuff for Gl
    let sdl = SDL::init(InitFlags::Everything).expect("couldn't start SDL");

    sdl.gl_set_attribute(SdlGlAttr::MajorVersion, 3).unwrap();
    sdl.gl_set_attribute(SdlGlAttr::MinorVersion, 3).unwrap();
    sdl.gl_set_attribute(SdlGlAttr::Profile, GlProfile::Core).unwrap();

    let win = sdl.create_gl_window(
      "Hello Window",
      WindowPosition::Centered,
      800,
      600,
      WindowFlags::Shown,
    ).expect("couldn't make a window and context");

    
    // Setup stuff for the render system
    let mut world = World { objects: vec![], window: win };

    let cube = Polygon3D::new(
vec![
            Surface::new(vec![
                (0, 0, 0),
                (100, 0, 0),
                (0, 100, 0),
                (100, 100, 0)
            ]),
            Surface::new(vec![
                (0, 0, 0),
                (100, 0, 0),
                (0, 0, 100),
                (100, 0, 100)
            ]),
            Surface::new(vec![
                (0, 0, 0),
                (0, 100, 0),
                (0, 0, 100),
                (0, 100, 100)
            ]),
            Surface::new(vec![
                (100, 0, 0),
                (100, 100, 0),
                (100, 0, 100),
                (100, 100, 100)
            ]),
            Surface::new(vec![
                (0, 100, 0),
                (100, 100, 0),
                (0, 100, 100),
                (100, 100, 100)
            ]),
            Surface::new(vec![
                (0, 0, 100),
                (100, 0, 100),
                (0, 100, 100),
                (100, 100, 100)
            ])
        ]
    );

    world.add(cube);

    // main render loop
    'main_loop: loop {
        // handle events this frame
        while let Some(event) = sdl.poll_events().and_then(Result::ok) {
            match event {
                Event::Quit(_) => break 'main_loop,
                _ => ()
            }
        }
        // now the events are clear

        world.render();
    }

    Ok(())
}

struct Polygon {
    vertices: Vec<(u16, u16)>
}

impl Polygon {
    fn draw(self) {}
}
