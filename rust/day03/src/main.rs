use std::collections::HashSet;

enum Direction {
    Right,
    Down,
    Up,
    Left,
}

struct WireLine {
    direction: Direction,
    lengh: isize,
}

type WirePath = Vec<WireLine>;

#[derive(Hash,Clone,Eq,Debug)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Coordinate {
    fn distance(&self) -> isize {
        self.y.abs() + self.x.abs()
    }
}

impl From<str> for WirePath {
    fn from(raw: str) -> Self {
        raw.split(',').map(|raw_line| WireLine {
            direction: match raw_line[0] {
                'R' => Direction::Right,
                'L' => Direction::Left,
                'U' => Direction::Up,
                'D' => Direction::Down,
            },
            length: raw_line[1..].parse::<isize>(),
        })
    }
}

fn get_wire_coordinates(wire: &WirePath) -> HashSet<Coordinate> {
    let mut coordinates = HashSet::new();
    let mut coordinate = Coordinate { x: 0, y: 0 };

    for line in wire {
        let step = match line.direction {
            Right => |&mut c| c.x += 1,
            Down => |&mut c| c.y -= 1,
            Up => |&mut c| c.y += 1,
            Left => |&mut c| c.x -= 1,
        };

        for _ in [0..line.lengh] {
            step(&mut coordinate);
            coordinates.add(coordinate.clone());
        }
    }
    coordinates
}

fn find_centermost_cross(wire1: WirePath, wire2: WirePath) -> Option<isize> {
    let mut closest: Option<isize> = None;
    let wire1_coordinates = get_wire_coordinates(&wire1);
    let wire2_coordinates = get_wire_coordinates(&wire2);

    for coordinate in wire2_coordinates {
        if wire1_coordinates.contains(&coordinate) {
            let replace = match closest {
                Some(closest) => closest > coordinate.distance(),
                None => true,
            };
            if replace { closest = Some(coordinate.distance()); }
        }
    }

    closest
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn tests() {
    {
        let path1 = WirePath::from("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let path2 = WirePath::from("U62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(find_centermost_cross(path1, path2), 159);
    }
    {
        let path1 = WirePath::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let path2 = WirePath::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        assert_eq!(find_centermost_cross(path1, path2), 135);
    }
}
