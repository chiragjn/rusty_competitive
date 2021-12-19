use lazy_static::lazy_static;
use std::{
    collections::HashSet,
    io::{self, BufRead, Stdin},
};

struct InputUtils {
    stream: Stdin,
}

impl Default for InputUtils {
    fn default() -> Self {
        Self {
            stream: io::stdin(),
        }
    }
}

impl Iterator for InputUtils {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.stream
            .lock()
            .lines()
            .next()
            .map(|line| line.unwrap().trim().to_string())
    }
}

lazy_static! {
    static ref ROTATIONS: [[[i64; 3]; 3]; 24] = [
        [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
        [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
        [[1, 0, 0], [0, -1, 0], [0, 0, -1]],
        [[1, 0, 0], [0, 0, 1], [0, -1, 0]],
        [[0, -1, 0], [1, 0, 0], [0, 0, 1]],
        [[0, 0, 1], [1, 0, 0], [0, 1, 0]],
        [[0, 1, 0], [1, 0, 0], [0, 0, -1]],
        [[0, 0, -1], [1, 0, 0], [0, -1, 0]],
        [[-1, 0, 0], [0, -1, 0], [0, 0, 1]],
        [[-1, 0, 0], [0, 0, -1], [0, -1, 0]],
        [[-1, 0, 0], [0, 1, 0], [0, 0, -1]],
        [[-1, 0, 0], [0, 0, 1], [0, 1, 0]],
        [[0, 1, 0], [-1, 0, 0], [0, 0, 1]],
        [[0, 0, 1], [-1, 0, 0], [0, -1, 0]],
        [[0, -1, 0], [-1, 0, 0], [0, 0, -1]],
        [[0, 0, -1], [-1, 0, 0], [0, 1, 0]],
        [[0, 0, -1], [0, 1, 0], [1, 0, 0]],
        [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
        [[0, 0, 1], [0, -1, 0], [1, 0, 0]],
        [[0, -1, 0], [0, 0, -1], [1, 0, 0]],
        [[0, 0, -1], [0, -1, 0], [-1, 0, 0]],
        [[0, -1, 0], [0, 0, 1], [-1, 0, 0]],
        [[0, 0, 1], [0, 1, 0], [-1, 0, 0]],
        [[0, 1, 0], [0, 0, -1], [-1, 0, 0]]
    ];
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Clone)]
struct Scanner {
    position: Point,
    beacons: Vec<Point>,
}

struct ResolvedResult {
    aligned: usize,
    index: usize,
    scanner: Scanner,
}

fn read_scanners(lines: &mut Box<dyn Iterator<Item = String>>) -> Vec<Scanner> {
    let mut scanners = vec![];
    for line in lines {
        if line.starts_with("---") {
            scanners.push(Scanner {
                position: Point { x: 0, y: 0, z: 0 },
                beacons: vec![],
            });
        } else if line.trim().is_empty() {
            continue;
        } else {
            let parts: Vec<i64> = line
                .trim()
                .split(',')
                .map(|p| {
                    p.parse::<i64>()
                        .expect("Failed to parse beacon position to i64")
                })
                .collect();
            assert!(
                parts.len() == 3,
                "found more than three integers on a single line"
            );
            scanners.last_mut().unwrap().beacons.push(Point {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            });
        }
    }
    scanners
}

fn rotate_point(point: &Point, rotation: &[[i64; 3]; 3]) -> Point {
    Point {
        x: point.x * rotation[0][0] + point.y * rotation[0][1] + point.z * rotation[0][2],
        y: point.x * rotation[1][0] + point.y * rotation[1][1] + point.z * rotation[1][2],
        z: point.x * rotation[2][0] + point.y * rotation[2][1] + point.z * rotation[2][2],
    }
}

fn rotate(scanner: &Scanner, rotation: &[[i64; 3]; 3]) -> Scanner {
    Scanner {
        position: scanner.position,
        beacons: scanner
            .beacons
            .iter()
            .map(|p| rotate_point(p, rotation))
            .collect(),
    }
}

fn translate_point(point: &Point, offset: &Point) -> Point {
    Point {
        x: point.x + offset.x,
        y: point.y + offset.y,
        z: point.z + offset.z,
    }
}

fn translate(scanner: &Scanner, offset: &Point) -> Scanner {
    Scanner {
        position: translate_point(&scanner.position, offset),
        beacons: scanner
            .beacons
            .iter()
            .map(|p| translate_point(p, offset))
            .collect(),
    }
}

fn count_aligned(resolved_beacon_positions: &HashSet<Point>, scanner: &Scanner) -> usize {
    let set = HashSet::from_iter(scanner.beacons.iter().copied());
    resolved_beacon_positions.intersection(&set).count()
}

fn l1_distance(point_a: &Point, point_b: &Point) -> u64 {
    ((point_a.x - point_b.x).abs() + (point_a.y - point_b.y).abs() + (point_a.z - point_b.z).abs())
        as _
}

fn solve(mut lines: Box<dyn Iterator<Item = String>>) -> u64 {
    let scanners = read_scanners(&mut lines);
    if scanners.is_empty() {
        return 0;
    }
    let mut resolved: Vec<bool> = vec![false; scanners.len()];
    let mut rotated_scanners: Vec<Vec<Scanner>> = vec![];
    for scanner in scanners {
        rotated_scanners.push(ROTATIONS.iter().map(|r| rotate(&scanner, r)).collect());
    }
    let mut resolved_scanner_positions: Vec<Point> = vec![];
    let mut resolved_beacon_positions: HashSet<Point> = HashSet::new();
    let mut resolved_result = Some(ResolvedResult {
        aligned: rotated_scanners[0][0].beacons.len(),
        index: 0,
        scanner: rotated_scanners[0][0].clone(),
    });
    while let Some(rr) = resolved_result {
        resolved[rr.index] = true;
        resolved_scanner_positions.push(rr.scanner.position);
        println!("Resolved scanner {} @ {:?}", rr.index, rr.scanner.position);
        resolved_beacon_positions.extend(HashSet::<Point>::from_iter(
            rr.scanner.beacons.iter().copied(),
        ));
        resolved_result = None;
        for i in 0..rotated_scanners.len() {
            if !resolved[i] {
                for rotated_scanner in rotated_scanners[i].iter() {
                    for resolved_beacon_position in resolved_beacon_positions.iter() {
                        // While this gives us much more accuracy and probability of success
                        // it might be an overkill to check with all resolved points so far
                        for unresolved_beacon_position in rotated_scanner.beacons.iter() {
                            let offset = Point {
                                x: -unresolved_beacon_position.x + resolved_beacon_position.x,
                                y: -unresolved_beacon_position.y + resolved_beacon_position.y,
                                z: -unresolved_beacon_position.z + resolved_beacon_position.z,
                            };
                            let moved_scanner = translate(rotated_scanner, &offset);
                            let aligned = count_aligned(&resolved_beacon_positions, &moved_scanner);
                            if resolved_result.is_none()
                                || aligned > resolved_result.as_ref().unwrap().aligned
                            {
                                resolved_result = Some(ResolvedResult {
                                    aligned,
                                    index: i,
                                    scanner: moved_scanner,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    let mut max_distance: u64 = 0;
    for i in 0..resolved_scanner_positions.len() {
        for j in (i + 1)..resolved_scanner_positions.len() {
            max_distance = std::cmp::max(
                max_distance,
                l1_distance(
                    &resolved_scanner_positions[i],
                    &resolved_scanner_positions[j],
                ),
            );
        }
    }
    max_distance
}

fn main() {
    let iu = InputUtils::default();
    let boxed_iter = Box::new(iu);
    println!("{}", solve(boxed_iter));
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test() {
        let test_input = r#"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14"#;
        let it = test_input
            .split('\n')
            .into_iter()
            .map(|part| part.to_string());
        assert_eq!(solve(Box::new(it)), 3621);
    }
}
