use std::collections::HashSet;
use std::collections::VecDeque;

pub type Deck = VecDeque<usize>;

#[derive(Debug)]
pub struct PuzzleInput {
    deck1: Deck,
    deck2: Deck,
}

#[aoc_generator(day22)]
pub fn parser(input: &str) -> PuzzleInput {
    let mut deck1 = Deck::new();
    let mut deck2 = Deck::new();

    let mut deck_ptr = &mut deck1;

    for line in input.lines() {
        if line.len() == 0 {
            continue;
        }

        match line.parse::<usize>() {
            Ok(num) => deck_ptr.push_back(num),
            Err(_) => {
                if line.starts_with("Player 2") {
                    deck_ptr = &mut deck2;
                }
            }
        }
    }

    PuzzleInput { deck1, deck2 }
}

fn run_simple_round(deck1: &mut Deck, deck2: &mut Deck) {
    let card1 = deck1.pop_front().unwrap();
    let card2 = deck2.pop_front().unwrap();

    if card1 > card2 {
        deck1.push_back(card1);
        deck1.push_back(card2);
    } else {
        deck2.push_back(card2);
        deck2.push_back(card1);
    }
}

#[aoc(day22, part1)]
pub fn day22_part1(data: &PuzzleInput) -> usize {
    let mut deck1 = data.deck1.clone();
    let mut deck2 = data.deck2.clone();

    while deck1.len() > 0 && deck2.len() > 0 {
        run_simple_round(&mut deck1, &mut deck2);
    }

    if deck1.len() > 0 {
        score_deck(&deck1)
    } else {
        score_deck(&deck2)
    }
}


/**
 * Runs recursive combat on two decks, mutating them and returning
 * the number of the winner
 */
fn run_recursive_combat(deck1: &mut Deck, deck2: &mut Deck, depth: usize) -> bool {
    let mut game_states_seen = HashSet::<(Deck, Deck)>::new();

    // Play until somebody runs out of cards
    while !deck1.is_empty() && !deck2.is_empty() {
        let current_state = (deck1.clone(), deck2.clone());

        if game_states_seen.contains(&current_state) {
            return true;
        }
        game_states_seen.insert(current_state);

        // This is a game we haven't played before
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();
    
         let player1_winner = if card1 > deck1.len() || card2 > deck2.len() {
            card1 > card2
         } else {
            // We can recurse
            let mut deck1_copy = deck1.clone().into_iter().take(card1).collect();
            let mut deck2_copy = deck2.clone().into_iter().take(card2).collect();

            run_recursive_combat(&mut deck1_copy, &mut deck2_copy, depth + 1)
        };

        if player1_winner {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }

    deck2.is_empty()
}

#[aoc(day22, part2)]
pub fn day22_part2(data: &PuzzleInput) -> usize {

    let mut deck1 = data.deck1.clone();
    let mut deck2 = data.deck2.clone();

    // let mut deck1 = vec![9,2,6,3,1];
    // let mut deck2 = vec![5,8,4,7,10];

    // let mut deck1 = vec![43,19];
    // let mut deck2 = vec![2,29,14];

    let player1_winner = run_recursive_combat(&mut deck1, &mut deck2, 1);
    // let player1_winner = rec_combat(&mut deck1, &mut deck2);
    
    if player1_winner {
        score_deck(&deck1)
    } else {
        score_deck(&deck2)
    }
}

fn score_deck(deck: &Deck) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, card)| (i+1)*card)
        .sum()
}
