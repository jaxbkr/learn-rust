#![allow(unused)]

use std::sync::MutexGuard;

// Struct
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Methods

impl Point {
    // associated function == static methods
    fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    // Methods == work on instances of points

    fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn dist_from_origin(&self) -> f32 {
        return (self.x * self.x + self.y * self.y).sqrt();
    }
}

struct Point3D(f32, f32, f32);

struct Empty;

// nested
#[derive(Debug)]
struct Circle {
    center: Point,
    radius: u32,
}

fn main() {
    let p = Point { x: 1.0, y: 2.0 };
    println!("point.x = {}, point.y = {}", p.x, p.y);

    let p3_d = Point3D(1.0, 2.0, 3.0);
    println!(
        "point3D.x = {}, point3D.y = {}, point3D.z = {}",
        p3_d.0, p3_d.1, p3_d.2
    );

    let empty = Empty;

    let circle = Circle {
        center: Point { x: 1.0, y: 2.0 },
        radius: 1,
    };
    println!("{:?}", circle);

    // shortcuts

    let x = 1.0;
    let y = 1.0;

    let p = Point { x, y };

    // copy fields

    let p0 = Point { x: 1.0, y: 1.0 };
    let p1 = Point { x: 2.0, ..p0 };

    // update a struct

    let mut p = Point { x: 0.0, y: 0.0 };
    p.x += 1.0;
    p.y += 2.0;
    println!("{:?}", p);

    // Using Methods

    let mut p = Point::zero();
    println!("{:?}", p);
    p.move_to(3.0, 5.0);
    println!("{:?}", p);
    let d = p.dist_from_origin();
    println!("distance from origin = {:?}", d)
}
