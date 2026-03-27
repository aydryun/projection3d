use std::thread;
use std::time::Duration;

use flo_canvas::*;
use flo_draw::*;

use proj::geometry::axis::Axis;
use proj::window::screen::{FrameContext, Screen};

use proj::obj::loader::Obj;

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

        let mut frame_ctx = FrameContext { delta_time: 0.0 };

        let mut model = Obj::new("./assets/skateboard.obj");
        model.load_obj_file().unwrap();

        //offset model 10z
        for point in model.vertices.iter_mut() {
            point.translate(10.0, Axis::Z);
        }

        loop {
            canvas.draw(|gc| {
                gc.clear_canvas(Color::Rgba(0.1, 0.1, 0.1, 1.0));
                gc.canvas_height(SCREEN.height);
                //gc.center_region(0.0, 0.0, SCREEN.width, SCREEN.height);

                frame_ctx.calculate_deltatime(&FPS);
                draw_frame(gc, &frame_ctx, &mut model);

                let sleep_time: Duration = Duration::from_millis((1000.0 / FPS) as u64);
                thread::sleep(sleep_time);
            });
        }
    });
}

fn draw_frame(gc: &mut CanvasGraphicsContext, fctx: &FrameContext, model: &mut Obj) {
    // Calcule le centre du modèle
    let mut center_x = 0.0;
    let mut center_z = 0.0;
    for point3d in model.vertices.iter() {
        center_x += point3d.x;
        center_z += point3d.z;
    }
    center_x /= model.vertices.len() as f32;
    center_z /= model.vertices.len() as f32;

    // Tourne tous les points autour du centre du modèle
    for point3d in model.vertices.iter_mut() {
        point3d.translate(-center_x, Axis::X);
        point3d.translate(-center_z, Axis::Z);
        point3d.rotate_xz(&(0.1 * PI * fctx.delta_time));
        point3d.translate(center_x, Axis::X);
        point3d.translate(center_z, Axis::Z);
    }

    // for point3d in model.vertices.iter() {
    //     SCREEN.draw_point(gc, &point3d.project(), 10.0, COLOR);
    // }

    for face in &model.faces {
        for i in 0..face.len() {
            let a = &model.vertices[face[i] - 1].project();
            let b = &model.vertices[face[(i + 1) % face.len()] - 1].project();
            SCREEN.draw_line(gc, &a, &b, COLOR);
        }
    }
}
