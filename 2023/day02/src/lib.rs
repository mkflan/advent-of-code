#[derive(Debug, Default)]
struct Draw {
    red_count: u32,
    green_count: u32,
    blue_count: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl Game {
    fn draws_within_requirements(&self) -> bool {
        self.draws
            .iter()
            .all(|draw| draw.red_count <= 12 && draw.green_count <= 13 && draw.blue_count <= 14)
    }
}

fn games(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|l| {
            let (game, result) = l.split_once(": ").unwrap();
            let game_id = game[5..].parse::<u32>().unwrap();
            let mut draws = Vec::<Draw>::new();

            for draw in result.split("; ") {
                let mut draw_stats = Draw::default();

                for color_draw in draw.split(", ") {
                    let (amount, color) = color_draw.split_once(" ").unwrap();
                    let amount = amount.parse::<u32>().unwrap();
                    let color = color.chars().next().unwrap();

                    if color == 'r' {
                        draw_stats.red_count = amount;
                    } else if color == 'g' {
                        draw_stats.green_count = amount;
                    } else {
                        draw_stats.blue_count = amount;
                    }
                }

                draws.push(draw_stats);
            }

            Game { id: game_id, draws }
        })
        .collect()
}

pub fn solve_part1(input: &str) -> u32 {
    games(input)
        .iter()
        .filter(|game| game.draws_within_requirements())
        .map(|game| game.id)
        .sum()
}

pub fn solve_part2(input: &str) -> u32 {
    games(input)
        .iter()
        .map(|game| {
            let mut rgb_highest = [0; 3];
            for draw in &game.draws {
                rgb_highest[0] = std::cmp::max(rgb_highest[0], draw.red_count);
                rgb_highest[1] = std::cmp::max(rgb_highest[1], draw.green_count);
                rgb_highest[2] = std::cmp::max(rgb_highest[2], draw.blue_count);
            }
            rgb_highest.into_iter().reduce(|acc, c| acc * c).unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_sample() {
        let sample_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(8, solve_part1(sample_input));
        assert_eq!(2286, solve_part2(sample_input));
    }
}
