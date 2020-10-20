use std::fs::File;
use std::io::prelude::*;

// The file, poker.txt, contains one-thousand random hands dealt to two players. Each line of the file contains ten cards (separated by a single space): the first five are Player 1's cards and the last five are Player 2's cards. You can assume that all hands are valid (no invalid characters or repeated cards), each player's hand is in no specific order, and in each hand there is a clear winner.
// How many hands does Player 1 win?

const CLUBS: u8 = 0;
const DIAMONDS: u8 = 1;
const SPADES: u8 = 2;
const HEARTS: u8 = 3;
struct Card {
    suit: u8, // clubs=0, diamonds=1, spades=2, hearts=3
    rank: u8, // 2, 3, 4, 5, 6, 7, 8, 9, 10, Jack, Queen, King, Ace
}

struct Hand {
    cards: [Card; 5],
}

//                                   straight flush ranks    multiples (rank, num)
fn evaluate_hand(hand: &Hand) -> (bool, bool, Vec<u8>, Vec<(u8, u8)>) {
    let mut straight: bool = true;
    let mut flush: bool = true;
    let mut ranks: Vec<u8> = Vec::new();
    let mut multiples: Vec<(u8, u8)> = Vec::new();
    let suit: u8 = hand.cards[0].suit;
    let mut numbers: [u8; 13] = [0; 13]; // keep a track of how manu of each rank are present;

    // gather the card ranks and check for a flush
    for card in hand.cards.iter() {
        ranks.push(card.rank);
        if flush {
            if card.suit != suit {
                flush = false;
            }
        }
    }
    ranks.sort();

    // gather multiples and check for a straight
    for i in 0..ranks.len() {
        numbers[ranks[i] as usize] += 1;
        if straight && i >= 1 {
            if ranks[i] != ranks[i - 1] + 1 {
                straight = false
            }
        }
    }

    if !straight && !flush {
        // build multiples vector
        for i in 0..numbers.len() {
            if numbers[i] > 1 {
                multiples.push((i as u8, numbers[i]));
            }
        }
        multiples.sort();
    }

    (straight, flush, ranks, multiples)
}

// check for two pairs
// chekc for one pair
// check for high card
impl Hand {
    fn beats(&self, aponent: Hand) -> bool {
        let (me_straight, me_flush, me_ranks, me_multiples) = evaluate_hand(self);
        let (them_straight, them_flush, them_ranks, them_multiples) = evaluate_hand(&aponent);
        let mut j: usize;

        let mut me_straight_flush_rank = 0;
        let mut them_straight_flush_rank = 0;

        // if I have a straight flush
        if me_straight && me_flush {
            me_straight_flush_rank = me_ranks[0];
        }
        // or they have a straight flush
        if them_straight && them_flush {
            them_straight_flush_rank = them_ranks[0];
        }
        // biggest straight flush wins
        if me_straight_flush_rank != 0 || them_straight_flush_rank != 0 {
            return me_straight_flush_rank > them_straight_flush_rank;
        }

        let mut me_4_kind_rank = 0;
        let mut them_4_kind_rank = 0;

        // if I have 4 of a kind
        if me_multiples.len() == 1 && me_multiples[0].1 == 4 {
            me_4_kind_rank = me_multiples[0].0;
        }
        // if they have 4 of a kind
        if them_multiples.len() == 1 && them_multiples[0].1 == 4 {
            them_4_kind_rank = them_multiples[0].0;
        }
        // biggest 4 of a kind wins
        if me_4_kind_rank != 0 || them_4_kind_rank != 0 {
            return me_4_kind_rank > them_4_kind_rank;
        }

        let mut me_full_house = false; // rank of the 3 of a kind card
        let mut them_full_house = false; // rank of the 3 of a kind card
                                         // if I have a full house
        if me_multiples.len() == 2 {
            if me_multiples[1].1 == 3 {
                me_full_house = true;
            }
        }
        // if they have a full house
        if them_multiples.len() == 2 {
            if them_multiples[1].1 == 3 {
                them_full_house = true;
            }
        }
        if me_full_house {
            if them_full_house {
                return me_multiples[1].0 > them_multiples[1].0;
            }
            return true;
        } else if them_full_house {
            return false;
        }
        // check for flushes
        if me_flush {
            if them_flush {
                j = me_ranks.len();
                for _i in 0..me_ranks.len() {
                    j -= 1;
                    if me_ranks[j] != them_ranks[j] {
                        return me_ranks[j] > them_ranks[j];
                    }
                }
                return false; // I guess it is a draw so player 1 does not win ??
            }
            return true;
        } else if them_flush {
            return false;
        }
        // check for straights
        if me_straight {
            if them_straight {
                if me_ranks[0] != them_ranks[0] {
                    return me_ranks[0] > them_ranks[0];
                }
                return false; // another draw
            }
            return true; // I win
        } else if them_straight {
            return false; // they win
        }
        // check for 3 of a kind
        if me_multiples.len() == 1 && me_multiples[0].1 == 3 {
            if them_multiples.len() < 1 || them_multiples[0].1 < 3 {
                return true;
            }
            // both have 3 of a kind, highest rank wins
            return me_multiples[0].0 > them_multiples[0].0;
        } else if them_multiples.len() == 1 && them_multiples[0].1 == 3 {
            return false; // they win
        }

        // check for 2 pairs
        if me_multiples.len() == 2 {
            if them_multiples.len() == 2 {
                if me_multiples[0].0 != them_multiples[0].0 {
                    return me_multiples[0].0 > them_multiples[0].0;
                } else if me_multiples[1].0 != them_multiples[1].0 {
                    return me_multiples[1].0 > them_multiples[1].0;
                }
                // go for high card
                j = me_ranks.len();
                for _i in 0..me_ranks.len() {
                    j -= 1;
                    if me_ranks[j] != them_ranks[j] {
                        return me_ranks[j] > them_ranks[j];
                    }
                }
                return false; // draw
            } else {
                return true; // I win
            }
        } else if them_multiples.len() == 2 {
            return false; // they win
        }

        // check for a pair
        if me_multiples.len() > 0 {
            if them_multiples.len() > 0 {
                if me_multiples[0].0 != them_multiples[0].0 {
                    return me_multiples[0].0 > them_multiples[0].0;
                }
                // go for high card
                j = me_ranks.len();
                for _i in 0..me_ranks.len() {
                    j -= 1;
                    if me_ranks[j] != them_ranks[j] {
                        return me_ranks[j] > them_ranks[j];
                    }
                }
                return false; // draw
            }
            return true;
        } else if them_multiples.len() > 0 {
            return false;
        }

        // go for high card
        j = me_ranks.len();
        for _i in 0..me_ranks.len() {
            j -= 1;
            if me_ranks[j] != them_ranks[j] {
                return me_ranks[j] > them_ranks[j];
            }
        }

        false
    }
}

fn parse_line(line: &str) -> (Hand, Hand) {
    //    println!( "{}", line );
    let hand_text = line.split_whitespace();

    let mut cards: Vec<Card> = Vec::new();

    for card_text in hand_text {
        let bytes = card_text.as_bytes();
        let suit: u8 = match bytes[1] {
            0x43 => CLUBS,    // 'C'
            0x44 => DIAMONDS, // 'D'
            0x53 => HEARTS,   // 'S'
            _ => SPADES,
        }; // 'H'
        let rank: u8 = match bytes[0] {
            0x32 => 0,
            0x33 => 1,
            0x34 => 2,
            0x35 => 3,
            0x36 => 4,
            0x37 => 5,
            0x38 => 6,
            0x39 => 7,
            0x54 => 8,
            0x4A => 9,
            0x51 => 10,
            0x4B => 11,
            0x41 => 12,
            _ => 13,
        };
        cards.push(Card { suit, rank });
    }

    let player1: Hand = Hand {
        cards: [
            cards.remove(0),
            cards.remove(0),
            cards.remove(0),
            cards.remove(0),
            cards.remove(0),
        ],
    };
    let player2: Hand = Hand {
        cards: [
            cards.remove(0),
            cards.remove(0),
            cards.remove(0),
            cards.remove(0),
            cards.remove(0),
        ],
    };

    (player1, player2)
}

pub fn p54() -> u64 {
    let mut player1_wins = 0;

    // open the file
    let mut file = File::open("p054_poker.txt").expect("file not found");

    // read the file into a string
    let mut s = String::new();
    file.read_to_string(&mut s).expect("could not read file");

    let lines = s.lines();

    for line in lines {
        // parse the data
        let (player1, player2) = parse_line(line);

        // work out who wins
        if player1.beats(player2) {
            player1_wins += 1;
        }
    }

    // return player 1 count
    player1_wins
}
