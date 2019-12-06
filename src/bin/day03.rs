/// Probably the ugliest piece of code I ever wrote. Let's pretend it never existed.
use adventofcode::files;

#[derive(Debug)]
struct Segment {
    a: (i32, i32),
    b: (i32, i32),
    direction: char,
}

impl Segment {
    fn does_intersect(&self, other_segment: &Segment) -> bool {
        if (self.is_vertical() && other_segment.is_vertical())
            || (!self.is_vertical() && !other_segment.is_vertical())
        {
            return false;
        }
        let horizontal: &Segment;
        let vertical: &Segment;

        match self.is_vertical() {
            true => {
                vertical = self;
                horizontal = other_segment;
            }
            false => {
                vertical = other_segment;
                horizontal = self;
            }
        };

        if horizontal.a.0 <= vertical.a.0
            && horizontal.b.0 >= vertical.a.0
            && horizontal.a.1 <= vertical.b.1
            && horizontal.a.1 >= vertical.a.1
        {
            true
        } else {
            false
        }
    }

    fn intersection(&self, other_segment: &Segment) -> (i32, i32) {
        if (self.is_vertical() && other_segment.is_vertical())
            || (!self.is_vertical() && !other_segment.is_vertical())
        {
            return (0, 0);
        }
        let horizontal: &Segment;
        let vertical: &Segment;

        match self.is_vertical() {
            true => {
                vertical = self;
                horizontal = &other_segment;
            }
            false => {
                vertical = &other_segment;
                horizontal = self;
            }
        };

        if horizontal.a.0 <= vertical.a.0
            && horizontal.b.0 >= vertical.a.0
            && horizontal.a.1 <= vertical.b.1
            && horizontal.a.1 >= vertical.a.1
        {
            (vertical.a.0, horizontal.a.1)
        } else {
            (0, 0)
        }
    }

    fn step_adjustement(&self, other_segment: &Segment) -> (i32, i32) {
        if (self.is_vertical() && other_segment.is_vertical())
            || (!self.is_vertical() && !other_segment.is_vertical())
        {
            return (0, 0);
        }
        let horizontal: &Segment;
        let vertical: &Segment;

        match self.is_vertical() {
            true => {
                vertical = self;
                horizontal = &other_segment;
            }
            false => {
                vertical = &other_segment;
                horizontal = self;
            }
        };

        if horizontal.a.0 <= vertical.a.0
            && horizontal.b.0 >= vertical.a.0
            && horizontal.a.1 <= vertical.b.1
            && horizontal.a.1 >= vertical.a.1
        {
            let vertical_step = match vertical.direction {
                'U' => horizontal.a.1 - vertical.a.1,
                _ => vertical.b.1 - horizontal.b.1,
            };

            let horizontal_step = match horizontal.direction {
                'R' => vertical.a.0 - horizontal.a.0,
                _ => horizontal.b.0 - vertical.b.0,
            };

            (horizontal_step, vertical_step)
        } else {
            (0, 0)
        }
    }

    fn is_vertical(&self) -> bool {
        self.a.0 == self.b.0
    }

    fn length(&self) -> i32 {
        self.b.0 - self.a.0 + self.b.1 - self.a.1
    }
}

fn create_segment_from_path(origin: (i32, i32), path: &String) -> (Segment, i32, i32) {
    let (x, y) = origin;
    let direction = path.chars().next().unwrap();
    let step: i32 = path[1..].to_owned().parse().unwrap();
    match direction {
        'U' => (
            Segment {
                a: (x, y),
                b: (x, y + step),
                direction: direction,
            },
            x,
            y + step,
        ),
        'D' => (
            Segment {
                a: (x, y - step),
                b: (x, y),
                direction: direction,
            },
            x,
            y - step,
        ),
        'R' => (
            Segment {
                a: (x, y),
                b: (x + step, y),
                direction: direction,
            },
            x + step,
            y,
        ),
        'L' => (
            Segment {
                a: (x - step, y),
                b: (x, y),
                direction: direction,
            },
            x - step,
            y,
        ),
        _ => (
            Segment {
                a: (0, 0),
                b: (0, 0),
                direction: direction,
            },
            0,
            0,
        ),
    }
}

fn path_to_segments(paths: &Vec<String>) -> Vec<Segment> {
    let mut coords = (0, 0);
    paths
        .iter()
        .map(|path| {
            let (segment, x, y) = create_segment_from_path(coords, path);
            coords = (x, y);

            segment
        })
        .collect()
}

fn find_distance_for_intersection(
    first_segments: Vec<Segment>,
    second_segments: Vec<Segment>,
) -> i32 {
    let mut distance = std::i32::MAX;

    for i in 0..first_segments.len() {
        for j in 0..second_segments.len() {
            if first_segments[i].does_intersect(&second_segments[j]) {
                let intersection = first_segments[i].intersection(&second_segments[j]);
                let intersection_distance = intersection.0.abs() + intersection.1.abs();
                if distance > intersection_distance && intersection_distance != 0 {
                    distance = intersection_distance;
                }
            }
        }
    }

    distance
}

fn find_steps_for_intersection(first_segments: Vec<Segment>, second_segments: Vec<Segment>) -> i32 {
    let mut steps = std::i32::MAX;

    for i in 0..first_segments.len() {
        for j in 0..second_segments.len() {
            if first_segments[i].does_intersect(&second_segments[j]) {
                let first_step = first_segments
                    .iter()
                    .take(i)
                    .fold(0, |acc, segment| acc + segment.length());
                let second_step = second_segments
                    .iter()
                    .take(j)
                    .fold(0, |acc, segment| acc + segment.length());
                let step_adjustement = first_segments[i].step_adjustement(&second_segments[j]);
                let step = first_step + second_step + step_adjustement.0 + step_adjustement.1;
                if step < steps && step > 0 {
                    steps = step;
                }
            }
        }
    }

    steps
}

fn main() {
    let lines = files::read_file_line_per_line("inputs/day03.txt");
    let first_wire: Vec<String> = lines[0].split(',').map(|path| path.to_string()).collect();
    let second_wire: Vec<String> = lines[1].split(',').map(|path| path.to_string()).collect();

    let distance = find_distance_for_intersection(
        path_to_segments(&first_wire),
        path_to_segments(&second_wire),
    );
    let steps = find_steps_for_intersection(
        path_to_segments(&first_wire),
        path_to_segments(&second_wire),
    );

    println!("Distance: {}", distance);
    println!("Steps: {}", steps);
}
