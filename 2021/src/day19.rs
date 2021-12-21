use std::collections::HashSet;

#[derive(Debug, Clone, Eq, Hash)]
struct Beacon {
    x: i16,
    y: i16,
    z: i16,
}

impl PartialEq for Beacon {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

#[derive(Debug)]
struct Scanner {
    id: usize,
    beacons: Vec<Beacon>,
    rotation: (i16, i16, i16),
    dimentions: (usize, usize, usize),
    transform: (i16, i16, i16),
}

impl Beacon {
    fn rotate(&self, (tx, ty, tz): (i16, i16, i16), (dx, dy, dz): (usize, usize, usize)) -> Beacon {
        let ps = [self.x, self.y, self.z];
        let ps_ = [ps[dx], ps[dy], ps[dz]];
        Beacon {
            x: ps_[0] * tx,
            y: ps_[1] * ty,
            z: ps_[2] * tz,
        }
    }

    fn transform(&self, (tx, ty, tz): (i16, i16, i16)) -> Beacon {
        Beacon {
            x: self.x + tx,
            y: self.y + ty,
            z: self.z + tz,
        }
    }

    fn diff(&self, other: &Beacon) -> (i16, i16, i16) {
        (other.x - self.x, other.y - self.y, other.z - self.z)
    }
}

impl Scanner {
    fn manhattan_dist(&self, other: &Scanner) -> u16 {
        let (x, y, z) = self.transform;
        let (xx, yy, zz) = other.transform;
        ((x - xx).abs() + (y - yy).abs() + (z - zz).abs()) as u16
    }

    fn find_overlap(&self, other: &Scanner) -> Option<Scanner> {
        let self_beacons: HashSet<Beacon> = self.beacons.iter().map(|b| b.clone()).collect();
        for scanner in rotations(other).iter() {
            for beacon in self.beacons.iter() {
                for reference_pt in scanner.beacons.iter() {
                    let diff = reference_pt.diff(&beacon);
                    let transformed_scanner = Scanner {
                        id: scanner.id,
                        beacons: scanner.beacons.iter().map(|b| b.transform(diff)).collect(),
                        rotation: scanner.rotation,
                        dimentions: scanner.dimentions,
                        transform: diff,
                    };
                    let matching = transformed_scanner
                        .beacons
                        .iter()
                        .filter(|b| self_beacons.contains(b));
                    if matching.count() >= 12 {
                        return Some(transformed_scanner);
                    }
                }
            }
        }
        None
    }
}

const MULTS: [(i16, i16, i16); 8] = [
    (1, 1, 1),
    (-1, 1, 1),
    (-1, -1, 1),
    (1, -1, 1),
    (1, 1, -1),
    (-1, 1, -1),
    (1, -1, -1),
    (-1, -1, -1),
];

const DIMS: [(usize, usize, usize); 6] = [
    (0, 1, 2),
    (1, 2, 0),
    (2, 0, 1),
    (0, 2, 1),
    (1, 0, 2),
    (2, 1, 0),
];

fn rotations(scanner: &Scanner) -> Vec<Scanner> {
    DIMS.iter()
        .flat_map(|(dx, dy, dz)| {
            MULTS.iter().map(|(xm, ym, zm)| {
                let beacons = scanner
                    .beacons
                    .iter()
                    .map(|beacon| beacon.rotate((*xm, *ym, *zm), (*dx, *dy, *dz)))
                    .collect();
                Scanner {
                    id: scanner.id,
                    beacons,
                    rotation: (*xm, *ym, *zm),
                    dimentions: (*dx, *dy, *dz),
                    transform: scanner.transform,
                }
            })
        })
        .collect()
}

fn parse_input(input: &str) -> Vec<Scanner> {
    let mut scanners = vec![Scanner {
        id: 0,
        beacons: vec![],
        rotation: (1, 1, 1),
        dimentions: (0, 1, 2),
        transform: (0, 0, 0),
    }];
    for line in input.lines() {
        if line.is_empty() {
            scanners.push(Scanner {
                id: scanners.len(),
                beacons: vec![],
                rotation: (1, 1, 1),
                dimentions: (0, 1, 2),
                transform: (0, 0, 0),
            });
            continue;
        }
        if line.starts_with("---") {
            continue;
        }
        let mut pieces = line.split(',');
        let x = pieces.next().unwrap().parse::<i16>().unwrap();
        let y = pieces.next().unwrap().parse::<i16>().unwrap();
        let z = pieces.next().unwrap().parse::<i16>().unwrap();
        let last_inx = scanners.len() - 1;
        scanners[last_inx].beacons.push(Beacon { x, y, z });
    }
    scanners
}

fn combine_scanners(scanners: Vec<Scanner>) -> Vec<Scanner> {
    let mut scanners_ = scanners.into_iter();
    let mut processed_ids: HashSet<usize> = HashSet::new();
    processed_ids.insert(0);
    let mut tried_comparisons: HashSet<(usize, usize)> = HashSet::new();
    let mut processed = vec![scanners_.next().unwrap()];
    let all_but_first: Vec<Scanner> = scanners_.collect();
    let mut num_processed = 1;
    while num_processed > 0 {
        num_processed = 0;
        let to_process: Vec<&Scanner> = all_but_first
            .iter()
            .filter(|s| !processed_ids.contains(&s.id))
            .collect();
        for scanner in to_process.iter() {
            for i in 0..processed.len() {
                let processed_scanner = &processed[i];
                if tried_comparisons.contains(&(processed_scanner.id, scanner.id)) {
                    continue;
                }
                match processed_scanner.find_overlap(&scanner) {
                    None => {
                        tried_comparisons.insert((processed_scanner.id, scanner.id));
                    }
                    Some(scan) => {
                        processed.push(scan);
                        processed_ids.insert(scanner.id);
                        num_processed += 1;
                        break;
                    }
                }
            }
        }
    }
    processed
}

pub fn day19_part1(input: &str) -> String {
    let scanners = parse_input(input);
    let combined = combine_scanners(scanners);
    let mut seen = HashSet::new();
    for scanner in combined {
        for beacon in scanner.beacons {
            seen.insert(beacon);
        }
    }
    format!("{}", seen.len())
}

pub fn day19_part2(input: &str) -> String {
    let scanners = parse_input(input);
    let combined = combine_scanners(scanners);
    let mut max_dist = 0;
    for (i, scanner) in combined.iter().enumerate() {
        for j in i..combined.len() {
            let dist = scanner.manhattan_dist(&combined[j]);
            if dist > max_dist {
                max_dist = dist;
            }
        }
    }
    format!("{}", max_dist)
}
