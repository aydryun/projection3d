use std::thread;
use std::time::Duration;

use flo_canvas::*;
use flo_draw::*;

struct Game {
    width: f32,
    height: f32,
}

struct Point {
    x: f32,
    y: f32,
}

struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

static GAME: Game = Game {
    width: 800.0,
    height: 800.0,
};
static FPS: f32 = 1.0;

pub fn main() {
    with_2d_graphics(|| {
        let canvas = create_canvas_window("Hello, triangle");

        let mut dz: f32 = 0.0;
        loop {
            canvas.draw(|gc| {
                // clear(gc);

                // gc.new_path();

                // gc.rect(-400.0, -400.0, 000.0, 00.0);
                // gc.rect(0.0, 0.0, 400.0, 400.0);

                // gc.fill_color(Color::Rgba(0.0, 0.0, 0.8, 1.0));
                // gc.fill();

                // draw_point(gc, &screen(&project(&Point3D { x: 0.0, y: 0.0, z: 1.0 })), 10.0);

                gc.clear_canvas(Color::Rgba(0.1, 0.1, 0.1, 1.0));
                gc.canvas_height(GAME.height);
                gc.center_region(0.0, 0.0, GAME.width, GAME.height);

                draw_frame(gc, &mut dz);
            });
        }
    });
}

fn clear(gc: &mut CanvasGraphicsContext) {
    // gc.clear_canvas(Color::Rgba(0.0, 0.4, 0.4, 1.0));
    gc.clear_canvas(Color::Rgba(0.1, 0.1, 0.1, 1.0));
    gc.canvas_height(GAME.height);
    gc.center_region(0.0, 0.0, GAME.width, GAME.height);
}

fn draw_point(gc: &mut CanvasGraphicsContext, point: &Point, size: f32) {
    gc.new_path();
    println!("point: x={}, y={}", point.x, point.y);
    gc.rect(
        point.x - (size / 2.0),
        point.y - (size / 2.0),
        point.x + (size / 2.0),
        point.y + (size / 2.0),
    );

    gc.fill_color(Color::Rgba(1.0, 0.0, 0.0, 1.0));
    gc.fill();
}

fn draw_frame(gc: &mut CanvasGraphicsContext, dz: &mut f32) {
    const DELTA_TIME: f32 = 1.0 / FPS;

    *dz += 1.0 * DELTA_TIME;
    println!("dz: {}", *dz);

    const vs: [Point3D; 8] = [
      Point3D {x: 0.5, y: 0.5, z: 1.0},
      Point3D {x: -0.5, y: 0.5, z: 1.0},
      Point3D {x: 0.5, y: -0.5, z: 1.0},
      Point3D {x: -0.5, y: -0.5, z: 1.0},
      Point3D {x: 0.5, y: 0.5, z: 10.0},
      Point3D {x: -0.5, y: 0.5, z: 10.0},
      Point3D {x: 0.5, y: -0.5, z: 10.0},
      Point3D {x: -0.5, y: -0.5, z: 10.0}
    ];

    for point in vs.iter() {
      draw_point(gc, &screen(&project(point)), 10.0);
    }

    let sleep_time: Duration = Duration::from_millis((1000.0 / FPS) as u64);
    thread::sleep(sleep_time);
}

fn screen(point: &Point) -> Point {
    Point {
        x: (point.x + 1.0) / 2.0 * GAME.width,
        y: (1.0 - (point.y + 1.0) / 2.0) * GAME.height,
    }
}

fn project(point3D: &Point3D) -> Point {
    Point {
        x: point3D.x / point3D.z,
        y: point3D.y / point3D.z,
    }
}
