// #[macro_use]
extern crate anyhow;
//extern crate itertools;
//use itertools::Itertools;
extern crate regex;
use regex::Regex;
use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
struct Card(usize);

#[derive(Clone, Debug, Hash)]
struct Deck {
  player: String,
  cards: VecDeque<Card>,
}

struct DeckBuilder {}
impl DeckBuilder {
  fn from_lines(lines: Vec<&str>) -> Vec<Deck> {
    let mut decks: Vec<Deck> = Vec::new();
    let player_re = Regex::new(r"Player ([0-9]+):").unwrap();

    let mut line_it = lines.iter(); // iterator over `&&str` of lines.
    while let Some(line) = line_it.next() {
      let player = {
        let caps = player_re.captures(line).expect("Illformed player line");
        caps.get(1).unwrap().as_str().to_string()
      };
      let mut cards: VecDeque<Card> = VecDeque::new();
      while let Some(line) = line_it.next() {
        if line.is_empty() {
          break;
        }
        cards.push_back(Card(line.parse().expect("Illformed card line")));
      }
      decks.push(Deck {
        player: player,
        cards: cards,
      });
    }
    decks
  }
}

struct Game {}
impl Game {
  fn play(decks: &mut Vec<Deck>) -> usize {
    loop {
      let (winner, loser) = if decks[0].cards[0].0 > decks[1].cards[0].0 {
        // decks[0] wins.
        (0, 1)
      } else {
        // decks[1] wins.
        (1, 0)
      };

      let mut winner_cards = Vec::new();
      winner_cards.push(decks[winner].cards.pop_front().unwrap());
      winner_cards.push(decks[loser].cards.pop_front().unwrap());
      for c in winner_cards {
        decks[winner].cards.push_back(c);
      }
      //println!("--\n{:?}", decks);

      if decks[loser].cards.is_empty() {
        let mut score = 0;
        let mut multiplier = 1;
        for c in decks[winner].cards.iter().rev() {
          score += multiplier * c.0;
          multiplier += 1;
        }
        return score;
      }
    }
  }
}

fn solve(input_all: String) -> anyhow::Result<()> {
  let lines = input_all.split_terminator("\n").collect::<Vec<_>>();
  let mut decks = DeckBuilder::from_lines(lines);
  let score = Game::play(&mut decks);
  println!("Part 1 {}", score);
  Ok(())
}

fn main() -> anyhow::Result<()> {
  let input_all = std::fs::read_to_string("day22/input.txt")?;
  //let input_all = std::fs::read_to_string("day22/test.txt")?;
  solve(input_all)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {}
}
