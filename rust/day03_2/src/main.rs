use std::collections::HashMap;
use std::fs;

enum Direction {
    Right,
    Down,
    Up,
    Left,
}

struct WireLine {
    direction: Direction,
    length: isize,
}

type WirePath = Vec<WireLine>;

#[derive(Hash, Clone, PartialEq, Eq, Debug)]
struct Coordinate {
    x: isize,
    y: isize,
}

fn create_path(raw: &str) -> WirePath {
    raw.split(',').map(|raw_line| WireLine {
        direction: match raw_line.chars().nth(0).unwrap() {
            'R' => Direction::Right,
            'L' => Direction::Left,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => panic!("Unexpectec character"),
        },
        length: raw_line[1..].parse::<isize>().unwrap(),
    }).collect()
}

fn get_wire_coordinates(wire: &WirePath) -> Vec<Coordinate> {
    let mut coordinates = Vec::new();
    let mut coordinate = Coordinate { x: 0, y: 0 };

    for line in wire {
        let step: Box<dyn Fn(&mut Coordinate)> = match line.direction {
            Direction::Right => Box::new(|c | c.x += 1),
            Direction::Down => Box::new(|c | c.y -= 1),
            Direction::Up => Box::new(|c | c.y += 1),
            Direction::Left => Box::new(|c | c.x -= 1),
        };

        for _ in 0..line.length {
            step(&mut coordinate);
            coordinates.push(coordinate.clone());
        }
    }
    coordinates
}

fn find_centermost_cross(wire1: WirePath, wire2: WirePath) -> Option<usize> {
    let mut closest: Option<usize> = None;
    let mut wire1_coordinates = HashMap::new();
    let wire2_coordinates = get_wire_coordinates(&wire2);

    for (i, coordinate) in get_wire_coordinates(&wire1).into_iter().enumerate() {
        wire1_coordinates.insert(coordinate, i + 1);
    }

    for (i, coordinate) in wire2_coordinates.iter().enumerate() {
        match wire1_coordinates.get(&coordinate) {
            Some(wire1_distance) => {
                let distance = *wire1_distance + i + 1;
                let replace = match closest {
                    Some(closest) => closest > distance,
                    None => true,
                };
                if replace {
                    closest = Some(distance);
                }
            }
            None => (),
        }
    }

    closest
}

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    let mut lines = file.lines();
    let path1 = create_path(lines.next().unwrap());
    let path2 = create_path(lines.next().unwrap());
    let distance = find_centermost_cross(path1, path2);
    match distance {
        Some(distance) => println!("Distance: {}", distance),
        None => println!("Failed to find distance"),
    }
}

#[test]
fn tests() {
    {
        let path1 = create_path("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let path2 = create_path("U62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(find_centermost_cross(path1, path2).unwrap(), 610);
    }
    {
        let path1 = create_path("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let path2 = create_path("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        assert_eq!(find_centermost_cross(path1, path2).unwrap(), 410);
    }
}
