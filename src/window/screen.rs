use flo_canvas::{CanvasGraphicsContext, Color, GraphicsContext, GraphicsPrimitives};

use crate::geometry::point::Point;

pub struct FrameContext {
    pub delta_time: f32,
}

pub struct Screen {
    pub width: f32,
    pub height: f32,
}

impl FrameContext {
    pub fn calculate_deltatime(&mut self, fps: &f32) {
        self.delta_time = 1.0 / fps
    }

    pub fn new(fps: &f32) -> FrameContext {
        FrameContext {
            delta_time: 1.0 / fps,
        }
    }
}

impl Screen {
    /*     pub fn draw(&self, gc: &mut CanvasGraphicsContext, point: Point, color: Color) {

      gc.draw
        Point {
            x: point.x / &self.width,
            y: point.y / &self.height,
        }
    } */

    /*
    pub fn new(width: f32, height: f32) -> Screen {
        Screen {
            width: width,
            height: height,
            aspect_ratio: width / height,
        }
    }
    */

    pub fn draw_point(
        &self,
        gc: &mut CanvasGraphicsContext,
        point: &Point,
        size: f32,
        color: Color,
    ) {
        let screen_point = self.get_screen_coordinates(point);
        gc.new_path();
        //println!("point: x={}, y={}", point.x, point.y);
        gc.rect(
            screen_point.x - (size / 2.0),
            screen_point.y - (size / 2.0),
            screen_point.x + (size / 2.0),
            screen_point.y + (size / 2.0),
        );

        gc.fill_color(color);
        gc.fill();
    }

    pub fn draw_line(
        &self,
        gc: &mut CanvasGraphicsContext,
        point_1: &Point,
        point_2: &Point,
        color: Color,
    ) {
        let screen_point_1 = self.get_screen_coordinates(point_1);
        let screen_point_2 = self.get_screen_coordinates(point_2);

        gc.new_path();
        gc.move_to(screen_point_1.x, screen_point_1.y);
        gc.line_to(screen_point_2.x, screen_point_2.y);
        gc.stroke_color(color);
        gc.stroke();
    }

    pub fn get_screen_coordinates(&self, point: &Point) -> Point {
        // println!("gsc : x: {}, y: {}", (point.x * self.width), (point.y * self.height));
        Point {
            //x: (point.x + 1.0) / 2.0 * self.width,
            //y: (point.y + 1.0) / 2.0 * self.height,
            x: point.x * self.width,
            y: point.y * self.height,
        }
    }
}
