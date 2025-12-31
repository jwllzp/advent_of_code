use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point(u64, u64);

impl Point {
    fn eq(&self, point: &Point) -> bool {
        (self.0 == point.0) && (self.1 == point.1)
    }

    fn is_aligned(&self, point: &Point) -> bool {
        self.0 == point.0 || self.1 == point.1
    }
}

#[derive(Debug)]
struct Rectangle<'a> {
    coords: (&'a Point, &'a Point),
    width: u64,
    height: u64,
    area: u64,
}

impl<'a> Rectangle<'a> {
    fn new(point_1: &'a Point, point_2: &'a Point) -> Self {
        let width = point_1.0.max(point_2.0) - point_1.0.min(point_2.0) + 1;
        let height = point_1.1.max(point_2.1) - point_1.1.min(point_2.1) + 1;
        let coords = (point_1, point_2);
        let area = match (width, height) {
            (1, h) => h,
            (w, 1) => w,
            (w, h) => w * h,
        };

        Rectangle {
            coords,
            width,
            height,
            area,
        }
    }

    fn get_corners(&self) -> Vec<&'a Point> {
        let p1 = self.coords.0;
        let p2 = self.coords.1;

        let (p3, p4) = match (p1.0.cmp(&p2.0), p1.1.cmp(&p2.1)) {
            (Ordering::Less, Ordering::Less) => todo!(),
            (Ordering::Less, Ordering::Equal) => todo!(),
            (Ordering::Less, Ordering::Greater) => todo!(),
            (Ordering::Equal, Ordering::Less) => todo!(),
            (Ordering::Equal, Ordering::Equal) => todo!(),
            (Ordering::Equal, Ordering::Greater) => todo!(),
            (Ordering::Greater, Ordering::Less) => todo!(),
            (Ordering::Greater, Ordering::Equal) => todo!(),
            (Ordering::Greater, Ordering::Greater) => todo!(),
        };

        vec![p1, p2, p3, p4]
    }
}

#[derive(Debug)]
struct Region {
    segments: Vec<Segment>,
}

#[derive(Debug)]
struct Segment {
    orientation: Orientation,
    start: u64,
    end: u64,
}

#[derive(Debug)]
enum Orientation {
    Horizontal(u64),
    Vertical(u64),
}

impl Region {
    fn new(points: &Vec<Point>) -> Self {
        let mut connected_pairs: HashSet<(&Point, &Point)> = HashSet::new();
        let mut segments: Vec<Segment> = Vec::new();

        for p1 in points {
            for p2 in points {
                if p1.eq(p2) || connected_pairs.contains(&(p1, p2)) {
                    continue;
                }

                if p1.is_aligned(p2) {
                    segments.push(Segment::new(p1, p2).unwrap());
                }

                connected_pairs.insert((p2, p1));
            }
        }

        Region { segments }
    }

    fn contains_rectangle(&self, rectancle: Rectangle) {
        todo!()
    }
}

#[derive(Debug)]
enum SegmentError {
    NonColinearPoints,
}

impl Segment {
    fn new(p1: &Point, p2: &Point) -> Result<Self, SegmentError> {
        if (p1.0 != p2.0) && (p1.1 != p2.1) {
            return Err(SegmentError::NonColinearPoints);
        }

        let orientation = if p1.0 == p2.0 {
            Orientation::Vertical(p1.0)
        } else {
            Orientation::Horizontal(p1.1)
        };

        let (start, end) = match orientation {
            Orientation::Horizontal(_) => (p1.0.min(p2.0), p1.0.max(p2.0)),
            Orientation::Vertical(_) => (p1.1.min(p2.1), p1.1.max(p2.1)),
        };

        Ok(Segment {
            orientation,
            start,
            end,
        })
    }
}

fn load_points(path: &str) -> Vec<Point> {
    let raw_input = std::fs::read_to_string(path).unwrap();
    raw_input
        .trim()
        .split("\n")
        .map(|l| {
            let mut coord_iter = l.split(",");
            let x = coord_iter.next().unwrap().parse::<u64>().unwrap();
            let y = coord_iter.next().unwrap().parse::<u64>().unwrap();
            Point(x, y)
        })
        .collect()
}

pub fn part_1(path: &str) -> u64 {
    let points = load_points(path);

    let mut max_area = 0;
    let mut rectangles: Vec<Rectangle> = Vec::new();
    for (idx, p1) in points[..points.len()].iter().enumerate() {
        for p2 in &points[(idx + 1)..] {
            let rectangle = Rectangle::new(p1, p2);
            max_area = max_area.max(rectangle.area);
            rectangles.push(rectangle);
        }
    }
    max_area
}

pub fn part_2(path: &str) -> u64 {
    let points = load_points(path);
    let region = Region::new(&points);
    dbg!(region);

    let mut max_area = 0;
    for (idx, p1) in points[..points.len()].iter().enumerate() {
        for p2 in &points[(idx + 1)..] {
            let rectangle = Rectangle::new(p1, p2);
            max_area = max_area.max(rectangle.area);
        }
    }
    max_area
}

#[cfg(test)]
mod d09 {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let output = part_1("src/days/inputs/09/example.txt");
        assert_eq!(50, output);
    }

    #[test]
    fn test_part_1() {
        let output = part_1("src/days/inputs/09/input.txt");
        assert_eq!(4748769124, output);
    }

    #[test]
    fn test_part_2_example() {
        let output = part_2("src/days/inputs/09/example.txt");
        assert_eq!(24, output);
    }

    #[test]
    fn test_part2() {
        todo!()
    }
}
