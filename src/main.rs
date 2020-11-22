use nannou::prelude::*;
use std::collections::VecDeque;

#[derive(Clone, Copy)]
struct Vertex {
    position: Point2,
    velocity: Vector2,
}
impl Vertex {
    fn update(&mut self, app: &App) {
        self.position += self.velocity;

        let bounds = app.window_rect();

        if self.position.x < bounds.left()
            || self.position.x > bounds.right()
            || self.position.y < bounds.bottom()
            || self.position.y > bounds.top()
        {
            self.velocity = -self.velocity
        }
    }
}

struct Path {
    points: Vec<Vertex>,
    color: rgb::Srgb<u8>,
    history: VecDeque<Vec<Vertex>>,
}
impl Path {
    fn new(app: &App) -> Path {
        let bounds = app.window_rect();
        let n_points = random_range(3, 9);

        let mut points = Vec::with_capacity(n_points as usize);
        for _ in 0..n_points {
            let x = random_range(bounds.left(), bounds.right());
            let y = random_range(bounds.bottom(), bounds.top());
            let direction = random_range(-10.0, 10.0);
            let speed = random_range(-10.0, 10.0);
            points.push(Vertex {
                position: pt2(x, y),
                velocity: pt2(direction, speed),
            })
        }
        points.push(points[0].clone());
        let colors = [SNOW, DEEPPINK, TURQUOISE, YELLOWGREEN, LIGHTGRAY];
        let color = colors[random_range(0, colors.len())];
        Path {
            points,
            color,
            history: VecDeque::with_capacity(5),
        }
    }
    fn update(&mut self, app: &App) {
        self.history.push_back(self.points.clone());
        if self.history.len() > 5 {
            self.history.pop_back();
        }
        for pt in &mut self.points {
            pt.update(app);
        }
    }
    fn display(&self, drawing: &Draw) {
        for path in &self.history {
            let points: Vec<Point2> = path.iter().map(|v| v.position).collect();

            drawing
                .polyline()
                .weight(1.0)
                .color(self.color)
                .points(points);
        }
        let points: Vec<Point2> = self.points.iter().map(|v| v.position).collect();

        drawing
            .polyline()
            .weight(2.0)
            .points(points)
            .color(self.color);
    }
}

struct Model {
    path: Vec<Path>,
}
impl Model {
    fn display(&self, drawing: &Draw) {
        for line in &self.path {
            line.display(drawing)
        }
    }
}
fn model(app: &App) -> Model {
    let bounds = app.window_rect();
    let mut paths: Vec<Path> = Vec::new();

    for _ in 0..2 {
        paths.push(Path::new(app));
    }
    /* let x0 = random_range(bounds.left(), bounds.right());
    let y0 = random_range(bounds.bottom(), bounds.top());
    let v0 = random_range(-3.0, 3.0);

    let x1 = random_range(bounds.left(), bounds.right());
    let y1 = random_range(bounds.bottom(), bounds.top());
    let v1 = random_range(-3.0, 3.0);

    let line = Path {
        points: vec![
            Vertex {
                position: pt2(x0, y0),
                velocity: pt2(v0, v0),
            },
            Vertex {
                position: pt2(x1, y1),
                velocity: pt2(v1, v1),
            },
        ],
    };
    lines.push(line); */
    Model { path: paths }
}
fn update(app: &App, model: &mut Model, update: Update) {
    for line in &mut model.path {
        line.update(app);
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
    /*  let line = model.lines[0]; */
    frame.clear(BLACK);
    let drawing = app.draw();

    model.display(&drawing);
    drawing.to_frame(app, &frame).unwrap();
}
fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}
