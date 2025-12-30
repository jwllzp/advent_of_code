#[derive(Debug)]
struct Point(u64, u64);

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
        todo!()
    }

    #[test]
    fn test_part2() {
        todo!()
    }
}
