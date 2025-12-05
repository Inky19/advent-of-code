use std::{
    cmp::Ordering,
    fs,
    ops::RangeInclusive,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn run() {
    let file =
        fs::read_to_string("../data/2025/day_05/input.txt").expect("Could not read input.txt");
    pb1and2(file.lines());
}

struct OrdRangeInclusive<T: Ord>(RangeInclusive<T>);

fn pb1and2<'a>(mut lines: impl Iterator<Item = &'a str>) {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let mut ranges: Vec<OrdRangeInclusive<i64>> = vec![];
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let range: Vec<&str> = line.split('-').collect();
        let start = range[0].parse::<i64>().unwrap();
        let end = range[1].parse::<i64>().unwrap();
        ranges.push(OrdRangeInclusive::new(start, end));
    }

    ranges.sort();
    let mut ranges_it = ranges.iter();
    let first_range = ranges_it.next().unwrap();
    let mut ranges_unique: Vec<OrdRangeInclusive<i64>> = vec![OrdRangeInclusive::new(
        *first_range.0.start(),
        *first_range.0.end(),
    )];
    for range in ranges_it {
        let prev_range = ranges_unique.last().unwrap();
        if prev_range.0.end() >= range.0.start() {
            if prev_range.0.end() >= range.0.end() {
                continue;
            } else {
                ranges_unique.push(OrdRangeInclusive::new(
                    *prev_range.0.end() + 1,
                    *range.0.end(),
                ));
            }
        } else {
            ranges_unique.push(OrdRangeInclusive::new(*range.0.start(), *range.0.end()));
        }
    }

    let mut sum1 = 0;
    for line in &mut lines {
        let id = line.parse::<i64>().unwrap();
        for range in &ranges_unique {
            if range.0.contains(&id) {
                sum1 += 1;
                break;
            }
        }
    }

    let mut sum2 = 0;
    for range in &ranges_unique {
        sum2 += range.0.end() - range.0.start() + 1;
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("\n====== Problem 1 ======");
    println!("Number of fresh items: {}", sum1);
    println!("\n====== Problem 2 ======");
    println!("Number of fresh IDs: {}", sum2);
    println!(
        "Duration for both problems combined: {}s",
        (end - start).as_secs_f64()
    );
}

impl<T: Ord> Ord for OrdRangeInclusive<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ord = self.0.start().cmp(other.0.start());
        if ord == Ordering::Equal {
            return self.0.end().cmp(other.0.end());
        }
        return ord;
    }
}

impl<T: Ord> PartialOrd for OrdRangeInclusive<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ord = Some(self.0.start().cmp(other.0.start()));
        if ord.is_some() && ord.unwrap() == Ordering::Equal {
            return Some(self.0.end().cmp(other.0.end()));
        }
        return ord;
    }
}

impl<T: Ord> PartialEq for OrdRangeInclusive<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.start() == other.0.start() && self.0.end() == other.0.end()
    }
}

impl<T: Ord> Eq for OrdRangeInclusive<T> {}

impl<T: Ord> OrdRangeInclusive<T> {
    pub fn new(start: T, end: T) -> OrdRangeInclusive<T> {
        return OrdRangeInclusive(RangeInclusive::new(start, end));
    }
}
