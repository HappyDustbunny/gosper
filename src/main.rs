use nannou::prelude::*;

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

    let begin = Vector2::new(0.0, 0.0);
    let end = Vector2::new(-395.0, -395.0);
    draw.line().start(begin).end(end).color(WHITE);

    let begin = Vector2::new(-390.0, -395.0);
    let end = Vector2::new(395.0, -395.0);
    draw.line().start(begin).end(end).color(WHITE);

    let begin = Vector2::new(395.0, -395.0);
    let end = Vector2::new(5.0, 0.0);
    draw.line().start(begin).end(end).color(WHITE);

    draw.to_frame(_app, frame).unwrap();
}
