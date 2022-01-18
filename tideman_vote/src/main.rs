use std::env;
use std::io;
use std::io::Write;
use std::cmp;

struct Pair <'s> {
    c1: &'s str,
    c2: &'s str,
}

#[derive(Debug)]
struct Edge<'a> {
	source: &'a str,
	target: &'a str,
    weight: usize,
}

#[derive(Debug)]
struct Vertices <'b>{
	point: &'b str,
	edges: Vec<&'b str>,
}

fn main() {
    let args: Vec<String>= env::args().collect(); //Gets input args
    if args.len() < 3 { //Returns error if less than 3 args
      panic!(
        "Error please enter at least 2 candidates.
        Format: cargo run candidate1 candidate2"
      )
    }
    let candidates:Vec<&str> = args[1..]//Parse input to &str
      .iter()
      .map(|x| x as &str)
      .collect();

    println!("How many voters are participating?"); //Query user: # of voters
    let voter_number: usize =
      get_input_from_user()
      .parse()
      .expect("invalid input");

      println!("{} voters", voter_number); //User feedback
      println!(
        "The candidates are: {:?}\n
        Please select each candidate in order of preference with 1 being the highest choice.",
        candidates
      );

    let vote_talley = talley_votes(voter_number, &candidates);
    let edges = get_edges(vote_talley);


    let unlocked_edges: Vec<&Edge> = get_unlocked_edges(edges, candidates);
    println!("{:?}", unlocked_edges);
    declare_winner(unlocked_edges);
}


fn get_input_from_user() -> String {
    let input = io::stdin();
    let mut line = String::new();
    input
      .read_line(&mut line)
      .expect("Error reading input");
    line.trim().to_string()
}

//Talley votes
fn talley_votes (voter_number:usize, candidates:&Vec<&str>) -> Vec<Vec<String>> {
    let mut vote_talley: Vec<Vec<String>> = Vec::new();

    for i in 0..voter_number{
      println!("voter {}", i + 1);
      let mut vote: Vec<String> = Vec::new();
      for i in 0..candidates.len() {
        print!("preference {}: ", i + 1);
        io::stdout().flush().unwrap();
        let input = get_input_from_user();
        if !vec_contains(&candidates, &input) {
          panic!("Error: input must be a candidate - {:?}", candidates);
        }
        vote.push(input);
      }
      vote_talley.push(vote);
    }
    vote_talley
}

fn get_edges<'e> (vote_talley: Vec<Vec<String>>) -> Vec<Edge<'e>> {
    let candidates: &Vec<String> = &vote_talley[0];
    let mut pairs: Vec<Pair> = Vec::new();
    for (i, candidate) in candidates.iter().enumerate() {
        for j in i+1..candidates.len() {
            pairs.push(Pair { c1: candidate, c2: &candidates[j] });
        }
    }
    let mut edges: Vec<Edge> = Vec::new();

    for pair in pairs.iter() {
        let (mut count1, mut count2) = (0 as usize, 0 as usize);
        let (mut index1, mut index2) = (0 as usize, 0 as usize);
        for vote in vote_talley.iter() {
            for (i, candidate) in vote.iter().enumerate() {
                if candidate == pair.c1 {
                index1 = i;
                }
                if candidate == pair.c2 {
                index2 = i;
                }
            }
            if index1 < index2 {
                count1 += 1;
            } else if index2 < index1 {
                count2 += 1;
            }
        }
        if count1 > count2 {
            edges.push(Edge {
                source: pair.c1,
                target: pair.c2,
                weight: count1 - count2,
            })
        } else if count2 > count1 {
            edges.push(Edge {
                source: pair.c2,
                target: pair.c1,
                weight: count2 - count1,
            })
        }
    }
    edges.sort_by(|e1, e2| e2.weight.cmp(&e1.weight));
    edges
}

fn declare_winner (unlocked_edges: Vec<&Edge>) {
    let mut sources: Vec<&str> = Vec::new();
    let mut targets: Vec<&str> = Vec::new();
    for edge in unlocked_edges.iter() {
        if !sources.contains(&edge.source) {
            sources.push(edge.source);
        }
        if !targets.contains(&edge.target) {
            targets.push(edge.target);
        }
    }
    for source in sources.iter() {
        if !targets.contains(source) {
            println!("The winner is {}", source)
        }
    }
}

fn get_unlocked_edges<'e> (edges: &'e Vec<Edge<'e>>, points: &Vec<&str> ) -> Vec<&'e Edge<'e>> {
    let mut unlocked_edges: Vec<&Edge> = Vec::new();

    for edge in edges.iter() {
        unlocked_edges.push(edge);
        let vertices = get_vertices(&unlocked_edges, points);
        let mut visited: Vec<&str> = Vec::new();
        if is_cycle(&mut visited, &vertices, &edge.source) {
            unlocked_edges.pop();
        }
    }
    unlocked_edges
}

fn get_vertices <'d>(edges: &Vec<&'d Edge<'d>>, points: &'d Vec<&'d str>) -> Vec<Vertices<'d>> {
    let mut vertices : Vec<Vertices> = Vec::new();

    for point in points.iter() {
        let mut vec: Vec<&str> = Vec::new();
        for edge in edges.iter() {
            if point == &edge.source {
                vec.push(edge.target);
            }
        }
        vertices.push(
            Vertices {
                point:point,
                edges: vec,
            }
        )
    }
    vertices
}

fn is_cycle <'c>(visited: &mut Vec<&'c str>,  vertices: &Vec<Vertices<'c>>, point: &'c str) -> bool {
    if visited.contains(&point) {
        return true
    }
    visited.push(point);
    for vert in vertices.iter() {
        if vert.point == point {
            if vert.edges.len() == 0 {
                *visited = vec![visited[0]];
                continue
            } else {
                for edge in vert.edges.iter() {
                   if is_cycle(visited, vertices, edge) {
                       return true
                   }
                }
            }
        }
    }
    false
}

fn map_votes<'a> (vote_talley: &'a Vec<Vec<String>>) -> Vec<Edge> {
    let candidates: &Vec<String> = &vote_talley[0];
    let mut pairs: Vec<Pair> = Vec::new();
    for (i, candidate) in candidates.iter().enumerate() {
        for j in i+1..candidates.len() {
            pairs.push(Pair { c1: candidate, c2: &candidates[j] });
        }
    }
    let mut edges: Vec<Edge> = Vec::new();

    for pair in pairs.iter() {
        let (mut count1, mut count2) = (0 as usize, 0 as usize);
        let (mut index1, mut index2) = (0 as usize, 0 as usize);
        for vote in vote_talley.iter() {
            for (i, candidate) in vote.iter().enumerate() {
                if candidate == pair.c1 {
                index1 = i;
                }
                if candidate == pair.c2 {
                index2 = i;
                }
            }
            if index1 < index2 {
                count1 += 1;
            } else if index2 < index1 {
                count2 += 1;
            }
        }
        if count1 > count2 {
            edges.push(Edge {
                source: pair.c1,
                target: pair.c2,
                weight: count1 - count2,
            })
        } else if count2 > count1 {
            edges.push(Edge {
                source: pair.c2,
                target: pair.c1,
                weight: count2 - count1,
            })
        }
    }
    edges.sort_by(|e1, e2| e2.weight.cmp(&e1.weight));
    edges
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