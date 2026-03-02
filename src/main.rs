use flo_canvas::*;
use flo_draw::*;
use flo_draw::binding::*;

struct Game {
    width: f32,
    height: f32,
}

struct Point {
    x: f32,
    y: f32,
}

static GAME: Game = Game {
    width: 800.0,
    height: 800.0,
};

pub fn main() {

    with_2d_graphics(|| {
        let canvas = create_canvas_window("Hello, triangle");

        canvas.draw(|gc| {
            // clear(gc);
            gc.clear_canvas(Color::Rgba(0.1, 0.1, 0.1, 1.0));
            gc.canvas_height(GAME.height);
            gc.center_region(0.0, 0.0, GAME.width, GAME.height);

            gc.new_path();

            // gc.rect(-400.0, -400.0, 000.0, 00.0);
            // gc.rect(0.0, 0.0, 400.0, 400.0);

            // gc.fill_color(Color::Rgba(0.0, 0.0, 0.8, 1.0));
            // gc.fill();

            draw_point(gc, &screen(&Point { x: 0.0, y: 0.0 }), 10.0);
        });
    });
}

fn clear(gc: &mut CanvasGraphicsContext) {
    gc.clear_canvas(Color::Rgba(0.0, 0.4, 0.4, 1.0));
}

fn draw_point(gc: &mut CanvasGraphicsContext, point: &Point, size: f32) {
    gc.new_path();
    println!("point: x={}, y={}", point.x, point.y);
    println!("x1: {}, x2: {}, y1: {}, y2: {}", point.x - size / 2.0, point.x - size / 2.0, point.y + size / 2.0, point.y + size / 2.0);
    gc.rect(
        point.x - (size / 2.0),
        point.y - (size / 2.0),
        point.x + (size / 2.0),
        point.y + (size / 2.0),
    );


    gc.fill_color(Color::Rgba(1.0, 0.0, 0.0, 1.0));
    gc.fill();
}

fn screen(point: &Point) -> Point {
    Point {
        x: (point.x + 1.0)/2.0*GAME.width,
        y: (1.0 - (point.y + 1.0)/2.0)*GAME.height,
    }
}
