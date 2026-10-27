use std::f64::consts::PI;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn distance_to(&self, other: &Point) -> f64 {
        let dx2 = (self.x - other.x).powi(2);
        let dy2 = (self.y - other.y).powi(2);

        (dx2 + dy2).sqrt()
    }
}

#[derive(Debug)]
enum Shape {
    Circle {
        center: Point,
        radius: f64,
    },
    Rectangle {
        top_left: Point,
        width: f64,
        height: f64,
    },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius, .. } => PI * radius.powi(2),
            Shape::Rectangle { width, height, .. } => width * height,
        }
    }

    fn scale(&mut self, factor: f64) {
        match self {
            Shape::Circle { radius, .. } => *radius *= factor,
            Shape::Rectangle { width, height, .. } => {
                *width *= factor;
                *height *= factor;
            }
        }
    }

    fn contains(&self, p: &Point) -> bool {
        match self {
            Shape::Circle { center, radius } => {
                (p.x - center.x).powi(2) + (p.y - center.y).powi(2) <= radius.powi(2)
            }
            Shape::Rectangle {
                top_left,
                width,
                height,
            } => {
                p.x >= top_left.x
                    && p.x <= top_left.x + width
                    && p.y >= top_left.y
                    && p.y <= top_left.y + height
            }
        }
    }

    fn largest(shapes: &[Shape]) -> Option<&Shape> {
        if shapes.is_empty() {
            return None;
        }

        let mut largest = &shapes[0];
        let mut largest_area = largest.area();

        for shape in &shapes[1..] {
            let area = shape.area();

            if area > largest_area {
                largest = shape;
                largest_area = area;
            }
        }

        Some(largest)
    }
}

fn main() {
    let p1 = Point::origin();
    let p2 = Point { x: 5.0, y: 5.0 };

    println!("Distance between p1 and p2: {}", p1.distance_to(&p2));

    let r1 = Shape::Rectangle {
        top_left: Point { x: 0.0, y: 0.0 },
        width: 5.0,
        height: 5.0,
    };

    let r2 = Shape::Rectangle {
        top_left: Point { x: 0.0, y: 0.0 },
        width: 10.0,
        height: 10.0,
    };

    let circle = Shape::Circle {
        center: Point { x: 2.0, y: 4.0 },
        radius: 20.0,
    };

    let mut shapes = [r1, r2, circle];

    for shape in &shapes {
        println!("{shape:?}: area {}", shape.area());
    }

    if let Some(shape) = Shape::largest(&shapes) {
        println!("largest: {shape:?}");
    }

    shapes[1].scale(4.0);

    if let Some(shape) = Shape::largest(&shapes) {
        println!("largest: {shape:?}");
    }

    println!("r2 contains p2: {}", shapes[1].contains(&p2));
}
