#[derive(Debug)]
struct Edge<'a> {
	source: &'a str,
	target: &'a str,
}

#[derive(Debug)]
struct Vertices <'b>{
	point: &'b str,
	edges: Vec<&'b str>,
}

fn main() {

	let points: Vec<&str> = vec!["A", "B", "C", "D"];

	let edges: Vec<Edge> =
		vec![
			Edge{source: "A", target: "B"},
			Edge{source: "A", target: "C"},
			Edge{source: "D", target: "A"},
			Edge{source: "C", target: "B"},
			Edge{source: "C", target: "D"},
			Edge{source: "D", target: "B"}
		];

	//Make Vertices
	let mut map: Vec<Vertices> = Vec::new();

	for point in points.iter() {
		let mut vec: Vec<&str> = Vec::new();
		for edge in edges.iter() {
			if point == &edge.source {
				vec.push(edge.target);
			}
		}
		map.push(
			Vertices {
				point: point,
				edges: vec,
			}
		)
	};

	let mut visited: Vec<&str> = Vec::new();

	fn is_cycle <'c>(visited: &mut Vec<&'c str>,  map: & Vec<Vertices<'c>>, point: &'c str) -> bool {
        println!("point {:?}", point);
        println!("visited {:?}", visited);
        if visited.contains(&point) {
            return true
        }
        visited.push(point);
		for vert in map.iter() {
            if vert.point == point {
                if vert.edges.len() == 0 {
                    *visited = vec![visited[0]];
                    continue
                } else {
                    println!("{:?}", vert.edges);
                    for edge in vert.edges.iter() {
                       if is_cycle(visited, map, edge) {
                           return true
                       }
                    }
                }
            }
        }
        false
	}

	let test: bool = is_cycle(&mut visited, &map, &map[2].point);



	println!("{:?}", test);
}

