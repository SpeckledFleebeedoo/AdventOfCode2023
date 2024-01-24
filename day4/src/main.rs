#[derive(Clone)]
struct Card{
    copies: i32,
    lucky_numbers: Vec<i32>,
    numbers: Vec<i32>,
    num_matches: i32,
}

fn main() {
    let mut total_score = 0;
    let file_path = "./src/input.txt";
    let contents = std::fs::read_to_string(file_path).expect("Failed to read file");
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut cards: Vec<Card> = Vec::new();

    for line in lines {
        let line_string = line.to_owned();
        let parts: Vec<&str> = line_string.split("|").collect();
        let winning_numbers: Vec<&str> = parts[0].trim().split_whitespace().collect();
        let card_numbers: Vec<&str> = parts[1].trim().split_whitespace().collect();
        let mut winning_numbers_int = Vec::new();
        let mut card_numbers_int: Vec<i32> = Vec::new();

        for n in &winning_numbers[2..] {
            winning_numbers_int.push(n.parse::<i32>().unwrap());
        }

        for n in card_numbers {
            card_numbers_int.push(n.parse::<i32>().unwrap());
        }

        let card = Card{copies: 1, lucky_numbers: winning_numbers_int, numbers: card_numbers_int, num_matches: 0};

        cards.push(card);
    }
    

    for i in 0..cards.len() {
        
        let lucky_numbers = cards[i].lucky_numbers.clone();

        for num in lucky_numbers {
            if cards[i].numbers.contains(&num) {
                cards[i].num_matches += 1;
            }
        }
        let score = match cards[i].num_matches {
            0 => 0,
            _ => 2_i32.pow((cards[i].num_matches - 1).try_into().unwrap()),
        };
        total_score += score;
        for _c in 0..cards[i].copies {
            if cards[i].num_matches != 0 {
                for m in 1..=cards[i].num_matches {
                    let index = i as i32 + m;
                    if index < cards.len() as i32 {
                        cards[index as usize].copies += 1;
                    }
                }
            }
        }
    }

    let mut total_cards = 0;
    for i in 0..cards.len() {
        total_cards += cards[i].copies;
    }

    println!("Total score: {}", total_score);
    println!("Total cards: {}", total_cards);
}