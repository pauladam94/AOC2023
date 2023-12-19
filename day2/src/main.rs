type Games = Vec<Game>;
type Game = Vec<MiniGame>;
type MiniGame = Vec<Entry>;

const RED: usize = 12;
const GREEN: usize = 13;
const BLUE: usize = 14;

fn possible_games_number(games: &Games) -> Vec<usize> {
    let mut possible_games_number: Vec<usize> = vec![];
    for (i, game) in games.iter().enumerate() {
        let n_game = i + 1;
        if let Some(()) = is_game_possible(game) {
            possible_games_number.push(n_game);
        }
    }
    possible_games_number
}

fn power_set_games(games: &Games) -> usize {
    let mut power_set_games: usize = 0;
    for game in games.iter() {
        power_set_games += power_game(game);
    }
    power_set_games
}

fn power_game(game: &Game) -> usize {
    let mut max_blue = 0;
    let mut max_red = 0;
    let mut max_green = 0;
    for mini_game in game.iter() {
        for entry in mini_game.iter() {
            use Color::*;
            match entry.color {
                Blue => {
                    if entry.number > max_blue {
                        max_blue = entry.number;
                    }
                }
                Red => {
                    if entry.number > max_red {
                        max_red = entry.number;
                    }
                }
                Green => {
                    if entry.number > max_green {
                        max_green = entry.number;
                    }
                }
            }
        }
    }
    max_blue * max_red * max_green
}

fn is_game_possible(game: &Game) -> Option<()> {
    for mini_game in game.iter() {
        for entry in mini_game.iter() {
            use Color::*;
            match entry.color {
                Blue => {
                    if entry.number > BLUE {
                        return None;
                    }
                }
                Red => {
                    if entry.number > RED {
                        return None;
                    }
                }
                Green => {
                    if entry.number > GREEN {
                        return None;
                    }
                }
            }
        }
    }
    Some(())
}

struct Entry {
    number: usize,
    color: Color,
}
enum Color {
    Blue,
    Red,
    Green,
}

impl std::str::FromStr for Color {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blue" => Ok(Color::Blue),
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Color::Blue => "blue",
            Color::Red => "red",
            Color::Green => "green",
        };
        write!(f, "{}", s)
    }
}

fn main() {
    let mut games: Games = vec![];
    let input = std::fs::read_to_string("input.txt").unwrap();
    for (i, line) in input.lines().enumerate() {
        let mut game: Game = vec![];
        let n_game = i + 1;
        line.to_string();
        // println!("{} | line : \n{}", n_game, line);
        let line = line.split(":").collect::<Vec<&str>>();
        let line = line[1].split(";");
        for mini_game_str in line {
            let mut mini_game: MiniGame = vec![];
            for entry_str in mini_game_str.split(", ") {
                let mut entry: Entry = Entry {
                    number: 0,
                    color: Color::Blue,
                };
                let entry_str = entry_str.trim();
                if let Some((number, color)) = entry_str.split_once(" ") {
                    let number = number.parse::<usize>().unwrap();
                    let color = color.parse::<Color>().unwrap();
                    // println!("{} | {} | {}", n_game, number, color);
                    entry.number = number;
                    entry.color = color;
                }

                mini_game.push(entry);
            }
            // println!("next_entry");
            game.push(mini_game);
        }
        games.push(game);
    }
    // On somme tous les élements du vec possible_games_number(&games)
    // println!("{}", possible_games_number(&games).iter().sum::<usize>());
    println!("{}", power_set_games(&games));
}
