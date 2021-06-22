use nannou::{prelude::*};


// t is the frame index. increment it for each tick.
// x is a value from 0 to 128
// y is a value from 0 to 64
fn get_pixel_color (t: u32, x: u32, y: u32) -> bool {
    jumping_blob(t as f32 / 30.0, (x as f32 / 64. - 0.5, y as f32 / 64.))
}

fn jumping_blob(t: f32, o: (f32, f32)) -> bool {
    let mut p = o;
    let radius = 0.18;
    let smoothing = 0.15;
    let dist = 0.26;
    p.0 -= 0.5;
    p.1 -= 0.5;
    p.1 *= -1.0;
    p = p_r(p, PI / 2.0);
    let q = p;
    p = p_r(p, -t);
    let s = f_op_difference_round(
        f_op_union_round(
            q.0,
            length((p.0 + dist, p.1)) - radius,
            smoothing,
        ),
        length((p.0 - dist, p.1)) - radius,
        smoothing,
    );
    return s >= 0.0;
}
fn p_r(p: (f32, f32), a: f32) -> (f32, f32) {
    (
        a.cos() * p.0 + a.sin() * p.1,
        a.cos() * p.1 - a.sin() * p.0,
    )
}
fn length(l: (f32, f32)) -> f32 {
    (l.0 * l.0 + l.1 * l.1).sqrt()
}
fn f_op_union_round(a: f32, b: f32, r: f32) -> f32 {
    r.max(a.min(b))
        - length(((r - a).max(0.), (r - b).max(0.)))
}
fn f_op_intersection_round(a: f32, b: f32, r: f32) -> f32 {
    (-r).min(a.max(b))
        + length(((r + a).max(0.), (r + b).max(0.)))
}
fn f_op_difference_round(a: f32, b: f32, r: f32) -> f32 {
    f_op_intersection_round(a, -b, r)
}

// all this part is to make the playground running on desktop

fn main() {
    nannou::app(model).update(update).run()
}
struct Model {
    _window: window::Id,
    i: u32
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { _window, i: 0 }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.i = model.i + 1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(CORNFLOWERBLUE);
    let w = 128u32;
    let h = 64u32;
    let sz = 4;
    for x in 0..w {
        for y in 0..h {
            draw.rect()
                .x_y((x*sz) as f32 - (w * sz / 2) as f32, (y*sz) as f32  - (h * sz / 2) as f32)
                .w(sz as f32)
                .h(sz as f32)
                .color(if get_pixel_color(model.i,x,y) { BLACK } else { WHITE });
        }
    }
    draw.to_frame(app, &frame).unwrap();
}