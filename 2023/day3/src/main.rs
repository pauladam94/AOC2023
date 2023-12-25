use std::collections::{HashMap, HashSet};

const NOT_SYMBOL: [char; 11] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];
type Data = [[char; 140]; 140];

fn number_adjacent_symbol(i: usize, j: usize, max_i: usize, max_j: usize, data: &Data) -> usize {
    let mut number_adjacent_symbol = 0;
    if i + 1 < max_i - 1 && !NOT_SYMBOL.contains(&data[i + 1][j]) {
        number_adjacent_symbol += 1;
    }
    if i > 0 && !NOT_SYMBOL.contains(&data[i - 1][j]) {
        number_adjacent_symbol += 1;
    }
    if j + 1 < max_j - 1 && !NOT_SYMBOL.contains(&data[i][j + 1]) {
        number_adjacent_symbol += 1;
    }
    if j > 0 && !NOT_SYMBOL.contains(&data[i][j - 1]) {
        number_adjacent_symbol += 1;
    }
    if i + 1 < max_i - 1 && j + 1 < max_j - 1 && !NOT_SYMBOL.contains(&data[i + 1][j + 1]) {
        number_adjacent_symbol += 1;
    }
    if i + 1 < max_i - 1 && j > 0 && !NOT_SYMBOL.contains(&data[i + 1][j - 1]) {
        number_adjacent_symbol += 1;
    }
    if i > 0 && j + 1 < max_j - 1 && !NOT_SYMBOL.contains(&data[i - 1][j + 1]) {
        number_adjacent_symbol += 1;
    }
    if i > 0 && j > 0 && !NOT_SYMBOL.contains(&data[i - 1][j - 1]) {
        number_adjacent_symbol += 1;
    }
    number_adjacent_symbol
}

fn adjacent_to_star(
    i: usize,
    j: usize,
    max_i: usize,
    max_j: usize,
    data: &Data,
) -> HashSet<Location> {
    let mut locations: HashSet<Location> = HashSet::new();
    if i + 1 < max_i - 1 && data[i + 1][j] == '*' {
        locations.insert(Location { i: i + 1, j });
    }
    if i > 0 && data[i - 1][j] == '*' {
        locations.insert(Location { i: i - 1, j });
    }
    if j + 1 < max_j - 1 && data[i][j + 1] == '*' {
        locations.insert(Location { i, j: j + 1 });
    }
    if j > 0 && data[i][j - 1] == '*' {
        locations.insert(Location { i, j: j - 1 });
    }
    if i + 1 < max_i - 1 && j + 1 < max_j - 1 && data[i + 1][j + 1] == '*' {
        locations.insert(Location { i: i + 1, j: j + 1 });
    }
    if i + 1 < max_i - 1 && j > 0 && data[i + 1][j - 1] == '*' {
        locations.insert(Location { i: i + 1, j: j - 1 });
    }
    if i > 0 && j + 1 < max_j - 1 && data[i - 1][j + 1] == '*' {
        locations.insert(Location { i: i - 1, j: j + 1 });
    }
    if i > 0 && j > 0 && data[i - 1][j - 1] == '*' {
        locations.insert(Location { i: i - 1, j: j - 1 });
    }
    locations
}

fn main() {
    let mut data: Data = [['.'; 140]; 140];
    let input = std::fs::read_to_string("input.txt").unwrap();
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            data[i][j] = char;
        }
    }
    first_star(&data);
    second_star(&data);
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Location {
    i: usize,
    j: usize,
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.i, self.j)
    }
}

fn second_star(data: &Data) {
    let mut hash_stars: HashMap<Location, Vec<usize>> = HashMap::new();
    let mut current_number = 0;
    let mut stars_location: HashSet<Location> = HashSet::new();
    for i in 0..140 {
        for j in 0..140 {
            match data[i][j].to_string().parse::<usize>() {
                Ok(number) => {
                    current_number = current_number * 10 + number;
                    let adjacent_stars = adjacent_to_star(i, j, 140, 140, &data);
                    for stars in adjacent_stars {
                        stars_location.insert(stars);
                    }
                }
                Err(_) => {
                    for stars in stars_location {
                        if hash_stars.contains_key(&stars) {
                            let mut vec = hash_stars.get(&stars).unwrap().to_vec();
                            vec.push(current_number);
                            hash_stars.insert(stars, vec);
                        } else {
                            hash_stars.insert(stars, vec![current_number]);
                        }
                    }
                    current_number = 0;
                    stars_location = HashSet::new();
                }
            }
        }
    }

    let mut final_sum = 0;
    for (location, numbers) in hash_stars {
        if numbers.len() == 2 {
            println!(
                "Location : {} {} | {} | {:?}",
                location.i, location.j, numbers[0], numbers[1]
            );
            final_sum = final_sum + numbers[0] * numbers[1];
        }
    }
    println!("second star : {}", final_sum);
}

fn first_star(data: &Data) {
    let mut final_sum = 0;
    let mut current_number = 0;
    let mut to_add_number = false;
    for i in 0..140 {
        for j in 0..140 {
            match data[i][j].to_string().parse::<usize>() {
                Ok(number) => {
                    current_number = current_number * 10 + number;
                    if number_adjacent_symbol(i, j, 140, 140, &data) >= 1 {
                        to_add_number = true;
                    }
                }
                Err(_) => {
                    if to_add_number {
                        final_sum = final_sum + current_number;
                        to_add_number = false;
                        // println!("{}", current_number);
                    }

                    current_number = 0;
                }
            }
        }
    }
    println!("first star : {}", final_sum);
}
