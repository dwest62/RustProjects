use std::env;
use std::io;
use std::io::Write;
use std::cmp;
use std::collections::HashMap;

#[derive(Debug)]
struct Pair {
  y: String,
  x: String,
  winner: String,
  loser: String,
  margin: usize,
}

impl Default for Pair {
  fn default() -> Pair {
    Pair {
      y: String::from("unknown"),
      x: String::from("unknown"),
      winner: String::from("unknown"),
      loser: String::from("unknown"),
      margin: 0,
    }
  }
}

impl Pair {
  fn get_pairs (candidates:Vec<String>) -> Vec<Pair> {
    let mut pairs: Vec<Pair>= Vec::new();
    for (i,x) in candidates.iter().enumerate() {
      for y in candidates[(i+1)..].iter() {
        let pair = Pair {
          x: x.to_string(),
          y: y.to_string(),
          ..Default::default()
        };
        pairs.push(pair);
      }
    }
    pairs
  }
}

fn main() {
  let candidates = get_args_from_user();

  println!("How many voters are participating?"); //Query user: # of voters
  let input = get_input_from_user();
  let voter_number= parse_string_to_usize(input);
  println!("{} voters", voter_number); //User feedback

  println!(
    "The candidates are: {:?}\nPlease select each candidate in order of preference with 1 being the highest choice.", candidates
  ); //Query user for votes
  let vote_talley=talley_votes(voter_number, &candidates);
  println!("vote: {:?}", vote_talley); //User feedback

  //Get a vector of candidate pairs based on votes and candidates sorted by margin of victory
  let candidate_pairs = get_candidate_pairs(vote_talley, candidates);

  let mut locked_pairs: Vec<&Pair> = Vec::new();

  for pair in candidate_pairs.iter() {
    if locked_pairs.len() == 0 {
      locked_pairs.push(pair);
    } else {
      let mut path: Vec<String> = Vec::new();
    }
  }
  fn makes_cycle <'a>(lp: &mut Vec<&'a Pair>, loser: &'a String, path: &mut Vec<&'a String>) -> bool {
    for locked_pair in lp.iter(){
      if locked_pair.winner.eq(loser) {
        path.push(loser);
        if has_duplicates(path) {
          return true
        }
      }
    }
    false
  }

  fn has_duplicates (vec: &Vec<&String>) -> bool {
    let mut map = HashMap::new();
    for el in vec.iter(){
      let count = map.entry(el).or_insert(0);
      *count += 1;
    }
    for (_, value) in map {
      if value > 1 {
        return true;
      }
    }
    return false;
  }
}

//Talley votes
fn talley_votes (voter_number:usize, candidates:&Vec<String>) -> Vec<Vec<String>> {
  let mut vote_talley: Vec<Vec<String>> = Vec::new();

  for i in 0..voter_number{
    println!("voter {}", i + 1);
    let mut vote: Vec<String> = Vec::new();
    for i in 0..candidates.len() {
      print!("preference {}: ", i + 1);
      io::stdout().flush().unwrap();
      let input = get_input_from_user();
      if !vec_contains(&candidates,&input) {
        panic!("Error: input must be a candidate - {:?}", candidates);
      }
      vote.push(input);
    }
    vote_talley.push(vote);
  }
  vote_talley
}

//Check if vector contains element
fn vec_contains <V: cmp::PartialEq<E>, E>(vector: &Vec<V>, el: &E) -> bool {
  for e in vector.iter() {
    if e == el {
      return true
    }
  }
  false
}

//Pair candidates and talley the results
fn get_candidate_pairs (vote_talley: Vec<Vec<String>>, candidates:Vec<String>) -> Vec<Pair> {
  let mut pairs: Vec<Pair> = Pair::get_pairs(candidates);
  //Get pair results
  for pair in pairs.iter_mut() {
    let mut diff_x_y: isize = 0;
    for vote in vote_talley.iter(){
      let mut x:isize = 0;
      let mut y:isize = 0;
      for (j, el) in vote.iter().enumerate() {
        if el == &pair.x {
          x = vote.len() as isize - j as isize;
        } else if el == &pair.y {
          y = vote.len() as isize - j as isize;
        }
      }
      diff_x_y = diff_x_y + (x - y);
      println!("x {}{} y {}{} diff {}", x, pair.x, y, pair.y, diff_x_y);
    }
    if diff_x_y > 0 {
      pair.winner = pair.x.clone();
      pair.loser = pair.y.clone();
      pair.margin = diff_x_y.abs() as usize;
    } else if diff_x_y < 0 {
      pair.winner = pair.y.clone();
      pair.loser = pair.x.clone();
      pair.margin = diff_x_y.abs() as usize;
    } else {
      pair.winner = String::from("tied");
    }
    println!("winner  {} margin {}", pair.winner, pair.margin)
  }
  pairs.sort_by(|p1, p2| p2.margin.cmp(&p1.margin));
  pairs
}

//Get args from user
fn get_args_from_user() -> Vec<String> {
  let mut args: Vec<String> = env::args().collect();
  args.remove(0);
  if args.len() < 2 {
    panic!(
      "Error please enter at least 2 arguments.
        Format: cargo run candidate1 candidate2"
    )
  }
  args
}

//Get input from user
fn get_input_from_user() -> String {
  let input = io::stdin();
  let mut line = String::new();
  input
    .read_line(&mut line)
    .expect("Error reading input");
  line.trim().to_string()
}

//Parse String to usize
fn parse_string_to_usize (text: String) -> usize {
  let result: usize =
  text
    .trim()
    .parse()
    .expect("Error parsing text to usize");
  result
}

/*fn get_unlocked_pairs (pairs: &Vec<Pair>) -> Vec<&Pair> {
  let mut unlocked_pairs: Vec<&Pair> = Vec::new();
  for e in pairs.iter() {
    if e.unlocked == true {
      unlocked_pairs.push(e);
    }
  }
  unlocked_pairs
}

candidate_pairs[0].unlocked = true;

let unlocked_pairs= get_unlocked_pairs(&candidate_pairs);*/