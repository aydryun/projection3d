use std::thread;
use std::time::Duration;

use flo_canvas::*;
use flo_draw::*;

use proj::geometry::point::{Point, Point3D};
use proj::window::screen::{Screen, FrameContext};

use std::f32::consts::PI;

static SCREEN: Screen = Screen {
    width: 800.0,
    height: 800.0,
};

static FPS: f32 = 60.0;
static COLOR: flo_canvas::Color = Color::Rgba(1., 0., 0., 1.0);

pub fn main() {

    with_2d_graphics(||  {
        let canvas = create_canvas_window("proj");

        let mut frame_ctx = FrameContext {
          dt: 0.0
        };

        // let mut dz: f32 = 0.0;
        // let mut angle: f32 = 0.0;
        loop {
            canvas.draw(|gc| {
                gc.clear_canvas(Color::Rgba(0.1, 0.1, 0.1, 1.0));
                gc.canvas_height(SCREEN.height);
                gc.center_region(0.0, 0.0, SCREEN.width, SCREEN.height);

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
    let mut vs: [Point3D; 4] = [
        Point3D { x:  0.6, y:  0.6, z:  0.6 + 3.0},
        Point3D { x: -0.6, y: -0.4, z:  0.5 + 3.0},
        Point3D { x: -0.5, y:  0.7, z: -0.4 + 3.0},
        Point3D { x:  0.4, y: -0.7, z: -0.5 + 3.0},
    ];

    #[rustfmt::skip]
    const FS: [usize; 12] = [
        0,1,2,
        0,3,1,
        0,2,3,
        1,3,2,
    ];

    for point3d in vs.iter_mut() {
      point3d.rotate_xz(&(2.0 * PI * fctx.dt));
    }

    for point3d in vs.iter() {
        // println!("Debug: {}", &point3d.project());
        SCREEN.draw_point(gc, &point3d.project(), 10.0, COLOR);
    }

    for i in 0..FS.len()  {
      let a = &vs[FS[i]].project();
      let b = &vs[FS[(i+1)%FS.len()]].project();

      SCREEN.draw_line(gc, &a, &b, COLOR);
    }
}
