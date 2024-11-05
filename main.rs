use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Rectangle {
    a: Point,
    b: Point,
}

impl Rectangle {
    fn area(&self) -> i32 {
        let width = (self.b.x - self.a.x).abs();
        let height = (self.b.y - self.a.y).abs();
        width * height
    }

    fn intersect(&self, other: &Rectangle) -> Option<Rectangle> {
        let x1 = self.a.x.max(other.a.x);
        let y1 = self.a.y.min(other.a.y);
        let x2 = self.b.x.min(other.b.x);
        let y2 = self.b.y.max(other.b.y);

        if x1 < x2 && y2 < y1 {
            Some(Rectangle {
                a: Point { x: x1, y: y1 },
                b: Point { x: x2, y: y2 },
            })
        } else {
            None
        }
    }
}

fn area_occupied(rectangles: &Vec<Rectangle>) -> i32 {
    let mut total_area = 0;
    let mut overlap_areas = HashSet::new();

    for (i, rect) in rectangles.iter().enumerate() {
        total_area += rect.area();

        for other in &rectangles[i + 1..] {
            if let Some(intersection) = rect.intersect(other) {
                overlap_areas.insert(intersection);
            }
        }
    }

    for overlap in overlap_areas {
        total_area -= overlap.area();
    }

    total_area
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}

fn main() {
    area_occupied_test();
    println!("Test passed!");
}
