extern crate regex;
use regex::Regex;
use std::collections::HashSet;
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

#[derive(Clone, Debug, Hash)]
struct GameResult {
  game_winner: usize,
  score: usize,
}

struct Game {}
impl Game {
  fn play(mut decks: Vec<Deck>) -> usize {
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

  fn play_recursive(mut decks: Vec<Deck>) -> GameResult {
    let mut deck_history = [
      HashSet::<VecDeque<Card>>::new(),
      HashSet::<VecDeque<Card>>::new(),
    ];

    let game_winner;

    loop {
      let history_repeated = [
        deck_history[0].contains(&decks[0].cards),
        deck_history[1].contains(&decks[1].cards),
      ];
      if history_repeated[0] && history_repeated[1] {
        game_winner = 0;
        break;
      }
      deck_history[0].insert(decks[0].cards.clone());
      deck_history[1].insert(decks[1].cards.clone());

      let topcard = [
        decks[0].cards.pop_front().unwrap(),
        decks[1].cards.pop_front().unwrap(),
      ];

      let (winner, loser) =
        if decks[0].cards.len() >= topcard[0].0 && decks[1].cards.len() >= topcard[1].0 {
          let result = {
            let mut recursive_decks = decks.clone();
            recursive_decks[0].cards.truncate(topcard[0].0);
            recursive_decks[1].cards.truncate(topcard[1].0);
            Game::play_recursive(recursive_decks)
          };
          // winner determined recursively.
          if result.game_winner == 0 {
            // decks[0] wins.
            (0, 1)
          } else {
            //decks[1] wins.
            (1, 0)
          }
        } else if topcard[0] > topcard[1] {
          // decks[0] wins.
          (0, 1)
        } else {
          // decks[1] wins.
          (1, 0)
        };

      decks[winner].cards.push_back(topcard[winner]);
      decks[winner].cards.push_back(topcard[loser]);

      if decks[loser].cards.is_empty() {
        game_winner = winner;
        break;
      }
    }
    let mut score = 0;
    let mut multiplier = 1;
    for c in decks[game_winner].cards.iter().rev() {
      score += multiplier * c.0;
      multiplier += 1;
    }
    return GameResult {
      game_winner: game_winner,
      score: score,
    };
  }
}

fn solve(input_all: String) {
  let lines = input_all.split_terminator("\n").collect::<Vec<_>>();
  let decks = DeckBuilder::from_lines(lines);
  let score = Game::play(decks.clone());
  println!("Part 1 {}", score);

  let result = Game::play_recursive(decks);
  println!("Part 2 {}", result.score);
}

fn main() -> anyhow::Result<()> {
  let input_all = std::fs::read_to_string("day22/input.txt")?;
  //let input_all = std::fs::read_to_string("day22/test.txt")?;
  solve(input_all);
  Ok(())
}
