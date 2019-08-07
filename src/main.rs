use nannou::prelude::*;
use std::f32;

fn main() {
    nannou::app(model).run()
}

struct Model {
    _window: WindowId,
}

fn model(app: &App) -> Model {
    let _window = app
    .new_window()
    .with_dimensions(800, 800)
    .with_title("Gosper")
    .view(view)
    .build()
    .unwrap();
    Model { _window }
}

fn view(_app: &App, _model: &Model, frame: &Frame) {


    // frame.clear(BLACK);
    let draw = _app.draw();

    draw.background().color(BLACK);

    let origo = Vector2::new(0.0, 0.0);
    let dir = Vector2::new(5.0, 5.0);
    let mut a = (origo, dir);

    gosper1(&draw, 4, 150.0, &mut a);

    pub fn gosper1(draw: &app::Draw, n: usize, len: f32, a: &mut(Vector2, Vector2)) -> (Vector2, Vector2) {
        // n is recursion depht, len is step length, a is (coordinate, direction)
        let U: f32 = (1.0/(3.0*3.0.sqrt())).atan();
        let V1: f32 = f32::consts::PI/6.0 - U;
        let V2: f32 = f32::consts::PI/2.0 - U;
        let V60 = f32::consts::PI/3.0;
        let V120 = f32::consts::PI - V60;

        if n == 0 {
            draw.line().start(a.0).end(a.0 + a.1).color(WHITE);
            a.0 += a.1;
        }
        else {
            a.1 = rot_vec(a.1, V1);
            gosper1(draw, n-1, len/7.0.sqrt(), a);
            a.1 = rot_vec(a.1, V60);
            gosper2(draw, n-1, len/7.0.sqrt(), a);
            a.1 = rot_vec(a.1, V120);
            gosper2(draw, n-1, len/7.0.sqrt(), a);
            a.1 = rot_vec(a.1, V60);
            gosper1(draw, n-1, len/7.0.sqrt(), a);
            a.1 = rot_vec(a.1, V120);
            gosper1(draw, n-1, len/7.0.sqrt(), a);
            gosper1(draw, n-1, len/7.0.sqrt(), a);
            a.1 = rot_vec(a.1, V60);
            gosper2(draw, n-1, len/7.0.sqrt(), a);
            a.1 = rot_vec(a.1, V2);
        }
        *a
    }

    pub fn gosper2(draw: &app::Draw, n: usize, len: f32, a: &mut(Vector2, Vector2)) -> (Vector2, Vector2) {
        // n is recursion depht, len is step length, a is (coordinate, direction)
        let U: f32 = (1.0/(3.0*3.0.sqrt())).atan();
        let V1: f32 = f32::consts::PI/6.0 - U;
        let V2: f32 = f32::consts::PI/2.0 - U;
        let V60 = f32::consts::PI/3.0;
        let V120 = f32::consts::PI - V60;

        if n == 0 {
            draw.line().start(a.0).end(a.0 + a.1).color(WHITE);
            a.0 += a.1;
        }
        else {
            a.1 = rot_vec(a.1, V2);
            gosper1(draw, n-1, len/7.0.sqrt(), a);
            a.1 = rot_vec(a.1, V60);
            gosper2(draw, n-1, len/7.0.sqrt(), a);
            gosper2(draw, n-1, len/7.0.sqrt(), a);
            a.1 = rot_vec(a.1, V120);
            gosper2(draw, n-1, len/7.0.sqrt(), a);
            a.1 = rot_vec(a.1, V60);
            gosper1(draw, n-1, len/7.0.sqrt(), a);
            a.1 = rot_vec(a.1, V120);
            gosper1(draw, n-1, len/7.0.sqrt(), a);
            a.1 = rot_vec(a.1, V60);
            gosper2(draw, n-1, len/7.0.sqrt(), a);
            a.1 = rot_vec(a.1, V1);
        }
        *a
    }

    pub fn rot_vec(a: Vector2, angle: f32) -> Vector2 {
        // Rotates vector a through angle radians
        let b = Vector2::new(angle.cos()*a[0] - angle.sin()*a[1], angle.sin()*a[0] + angle.cos()*a[1]);
        b
    }

    draw.to_frame(_app, frame).unwrap();
}
