use std::thread;
use std::time::Duration;

use flo_canvas::*;
use flo_draw::*;

use proj::geometry::point::{Point, Point3D};
use proj::window::screen::{FrameContext, Screen};

use std::f32::consts::PI;

static SCREEN: Screen = Screen {
    width: 800.0,
    height: 800.0,
};

static FPS: f32 = 60.0;
static COLOR: flo_canvas::Color = Color::Rgba(1., 0., 0., 1.0);

pub fn main() {
    with_2d_graphics(|| {
        let canvas = create_canvas_window("proj");

        let mut frame_ctx = FrameContext { dt: 0.0 };

        // let mut dz: f32 = 0.0;
        // let mut angle: f32 = 0.0;
        loop {
            canvas.draw(|gc| {
                gc.clear_canvas(Color::Rgba(0.1, 0.1, 0.1, 1.0));
                gc.canvas_height(SCREEN.height);
                //gc.center_region(0.0, 0.0, SCREEN.width, SCREEN.height);

                frame_ctx.calculate_deltatime(&FPS);
                draw_frame(gc, &frame_ctx);

                let sleep_time: Duration = Duration::from_millis((1000.0 / FPS) as u64);
                thread::sleep(sleep_time);
            });
        }
    });
}

fn draw_frame(gc: &mut CanvasGraphicsContext, fctx: &FrameContext) {
    #[rustfmt::skip]
    let mut vs: [Point3D; 8] = [
        Point3D { x: -0.5, y: -0.5, z: 1.5 },
        Point3D { x:  0.5, y: -0.5, z: 1.5 },
        Point3D { x:  0.5, y:  0.5, z: 1.5 },
        Point3D { x: -0.5, y:  0.5, z: 1.5 },

        Point3D { x: -0.5, y: -0.5, z: 2.5 },
        Point3D { x:  0.5, y: -0.5, z: 2.5 },
        Point3D { x:  0.5, y:  0.5, z: 2.5 },
        Point3D { x: -0.5, y:  0.5, z: 2.5 },
    ];

    #[rustfmt::skip]
    const FS: [usize; 36] = [
        // face avant
        0,1,2,
        0,2,3,

        // face arrière
        4,6,5,
        4,7,6,

        // gauche
        0,3,7,
        0,7,4,

        // droite
        1,5,6,
        1,6,2,

        // haut
        3,2,6,
        3,6,7,

        // bas
        0,4,5,
        0,5,1,
    ];

    for point3d in vs.iter_mut() {
        point3d.rotate_xz(&(2.0 * PI * fctx.dt));
    }

    for point3d in vs.iter() {
        // println!("Debug: {}", &point3d.project());
        SCREEN.draw_point(gc, &point3d.project(), 10.0, COLOR);
    }

    for i in 0..FS.len() {
        let a = &vs[FS[i]].project();
        let b = &vs[FS[(i + 1) % FS.len()]].project();

        SCREEN.draw_line(gc, &a, &b, COLOR);
    }
}
