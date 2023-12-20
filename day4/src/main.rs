type Data = [([u8; 10], [u8; 25]); 208];
type CompactData = [Game; 208];

#[derive(Clone, Debug, Copy)]
struct Game {
    winning_numbers: [bool; 256],
    numbers: [bool; 256],
}

fn data_to_compact_data(data: &Data) -> CompactData {
    let mut compact_data: CompactData = [Game {
        winning_numbers: [false; 256],
        numbers: [false; 256],
    }; 208];
    for (i, (winning_numbers, numbers)) in data.iter().enumerate() {
        for winning_number in winning_numbers.iter() {
            compact_data[i].winning_numbers[*winning_number as usize] = true;
        }
        for number in numbers.iter() {
            compact_data[i].numbers[*number as usize] = true;
        }
    }
    compact_data
}

fn to_string_data(data: &Data) -> String {
    let mut s = String::new();
    for (i, (winning_numbers, numbers)) in data.iter().enumerate() {
        s.push_str(&format!("{} | ", i + 1));
        for winning_number in winning_numbers.iter() {
            s.push_str(&format!("{} ", winning_number));
        }
        s.push_str("| ");
        for number in numbers.iter() {
            s.push_str(&format!("{} ", number));
        }
        s.push_str("\n");
    }
    s
}

fn main() {
    let mut data: Data = [([0; 10], [0; 25]); 208];
    let input = std::fs::read_to_string("input.txt").unwrap();
    for (i, line) in input.lines().enumerate() {
        let mut winning_numbers = [0; 10];
        let mut numbers = [0; 25];
        let mut line = line.trim().split(":");
        line.next();
        let mut line = line.next().unwrap().trim().split("|");
        let winning_numbers_str = line.next().unwrap().trim();
        let numbers_str = line.next().unwrap().trim();

        let mut index = 0;
        for winning_number_str in winning_numbers_str.split(" ") {
            match winning_number_str.parse::<u8>() {
                Ok(winning_number) => {
                    winning_numbers[index] = winning_number;
                    index += 1;
                }
                Err(_) => {}
            }
        }
        let mut index = 0;
        for number_str in numbers_str.split(" ") {
            match number_str.parse::<u8>() {
                Ok(number) => {
                    numbers[index] = number;
                    index += 1;
                }
                Err(_) => {}
            }
        }
        data[i] = (winning_numbers, numbers);
    }
    // println!("{}", to_string_data(&data));
    let compact_data = data_to_compact_data(&data);
    let mut total_points = 0;
    for game in 0..208 {
        let mut points = 0;
        for n in 0..256 {
            if compact_data[game].winning_numbers[n] && compact_data[game].numbers[n] {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }
        total_points += points;
    }
    println!("first star : {}", total_points);
}
