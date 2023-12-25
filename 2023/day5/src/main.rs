#[derive(Default)]
struct DataString {
    seeds: Vec<u64>,
    maps: [Vec<MapString>; 7],
}

impl std::fmt::Display for DataString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Seeds: ")?;
        for seed in self.seeds.iter() {
            write!(f, "{} ", seed)?;
        }
        write!(f, "\n")?;
        for (i, map) in self.maps.iter().enumerate() {
            writeln!(f, "Map {}: ", i + 1)?;
            for map in map.iter() {
                writeln!(f, "{} | {} | {}", map.first, map.second, map.range)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Default)]
struct Data {
    seeds: Vec<Range>,
    maps: [Vec<Map>; 7],
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Seeds: ")?;
        for seed in self.seeds.iter() {
            write!(f, "{} - {}", seed.begin, seed.end)?;
        }
        write!(f, "\n")?;
        for (i, map) in self.maps.iter().enumerate() {
            writeln!(f, "Map {}: ", i + 1)?;
            for map in map.iter() {
                writeln!(
                    f,
                    "{} - {} | {} - {}",
                    map.first.begin, map.first.end, map.second.begin, map.second.end
                )?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Default, Clone)]
struct RangeString {
    first: u64,
    second: u64,
    range: u64,
}

impl RangeString {
    const ZERO: Self = Self {
        first: 0,
        second: 0,
        range: 0,
    };
}

#[derive(Clone)]
struct Range {
    begin: u64,
    end: u64,
}
impl Range {
    const ZERO: Self = Self { begin: 0, end: 0 };

    fn new(begin: u64, end: u64) -> Self {
        Self { begin, end }
    }

    #[inline]
    fn inn(&self, value: u64) -> bool {
        value >= self.begin && value <= self.end
    }
}
struct Map {
    first: Range,
    second: Range,
}

#[derive(Default)]
struct MapString {
    first: u64,
    second: u64,
    range: u64,
}

impl Map {
    const ZERO: Self = Self {
        first: Range::ZERO,
        second: Range::ZERO,
    };
}

fn main() {
    let mut data = DataString::default();
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut number_map = 0;
    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            for j in line.split(" ") {
                match j.parse::<u64>() {
                    Ok(seed) => {
                        data.seeds.push(seed);
                    }
                    Err(_) => {}
                }
            }
        } else if line != "" {
            let mut range = MapString::default();
            let mut not_title_line = false;

            for (index, j) in line.split(" ").enumerate() {
                // println!("{}", j);
                match j.parse::<u64>() {
                    Ok(seed) => {
                        not_title_line = true;
                        if index == 0 {
                            range.second = seed;
                        } else if index == 1 {
                            range.first = seed;
                        } else if index == 2 {
                            range.range = seed;
                        }
                    }
                    Err(_) => {
                        number_map += 1;
                        break;
                    }
                }
            }
            if not_title_line {
                data.maps[number_map - 1].push(range);
            }
        }
    }
    // println!("{}", data);
    println!("first star : {}", first_star(&data));
    let mut data = convert_data(data);
    // println!("{}", data);
    println!("second star : {}", second_star(&data));
}

fn first_star(data: &DataString) -> u64 {
    let mut min_location = u64::MAX;
    for seed in data.seeds.iter() {
        seed_location(seed, data, &mut min_location);
    }
    min_location
}

fn seed_location(seed: &u64, data: &DataString, min_location: &mut u64) {
    let mut current_value = *seed;
    for i in 0..7 {
        for range in data.maps[i].iter() {
            if current_value >= range.first && current_value <= range.first + range.range - 1 {
                current_value = range.second + (current_value - range.first);
                break;
            }
        }
    }
    if current_value < *min_location {
        *min_location = current_value;
    }
}

fn convert_data(data: DataString) -> Data {
    let mut new_data = Data::default();
    for i in 0..(data.seeds.len() / 2) {
        new_data.seeds.push(Range {
            begin: data.seeds[2 * i],
            end: data.seeds[2 * i] + data.seeds[2 * i + 1] - 1,
        });
    }
    for i in 0..7 {
        for map in data.maps[i].iter() {
            new_data.maps[i].push(Map {
                first: Range {
                    begin: map.first,
                    end: map.first + map.range - 1,
                },
                second: Range {
                    begin: map.second,
                    end: map.second + map.range - 1,
                },
            });
        }
    }
    new_data
}

fn second_star(data: &Data) -> u64 {
    let mut min_location = u64::MAX;
    for seed in data.seeds.iter() {
        println!("seed : {} - {}", seed.begin, seed.end);
        let mut ranges: Vec<Range> = vec![seed.clone()];
        for j in 0..7 {
            println!("map : {}", j);
            let mut new_ranges: Vec<Range> = Vec::new();
            let mut index = 0;
            while index < ranges.len() {
                let range = ranges[index].clone();
                for map in data.maps[j].iter() {
                    if map.first.begin <= range.begin && map.first.end > range.end {
                        new_ranges.push(Range::new(
                            map.second.begin + (range.begin - map.first.begin),
                            map.second.begin + (range.end - map.first.begin),
                        ));
                        continue;
                    }

                    // Map in Range
                    if range.inn(map.first.begin) {
                        if range.inn(map.first.end) {
                            new_ranges.push(Range::new(
                                map.second.begin + (range.begin - map.first.begin),
                                map.second.begin + (range.end - map.first.begin),
                            ));
                            continue;
                        } else {
                            new_ranges.push(Range::new(
                                map.second.begin + (range.begin - map.first.begin),
                                map.second.begin + (map.first.end - map.first.begin),
                            ));
                            continue;
                        }
                    } else {
                        if range.inn(map.first.end) {
                            new_ranges.push(Range::new(
                                map.second.begin,
                                map.second.begin + (range.end - map.first.begin),
                            ));
                            continue;
                        } else {
                            new_ranges.push(Range::new(
                                map.second.begin,
                                map.second.begin + (map.first.end - map.first.begin),
                            ));
                            continue;
                        }
                    }

                    if map.first.begin >= range.begin && map.first.end <= range.end {
                        new_ranges.push(Range::new(map.second.begin, map.second.end));
                        continue;
                    }

                    if map.first.begin <= range.begin && map.first.end >= range.end {
                        new_ranges.push(Range::new(
                            map.second.begin + (range.begin - map.first.begin),
                            map.second.begin + (range.end - map.first.begin),
                        ));
                    }
                }
                index += 1;
            }
            ranges = new_ranges;
        }

        for range in ranges.iter() {
            if range.begin < min_location {
                min_location = range.begin;
            }
        }
    }
    min_location
}
