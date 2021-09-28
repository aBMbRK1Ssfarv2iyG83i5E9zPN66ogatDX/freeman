fn main() {
    // adjacency list for the graph
    let my_graph: Vec<Vec<usize>> = vec![
        vec![2, 5],
        vec![1, 3, 6],
        vec![2, 7, 4],
        vec![3, 8],
        vec![1, 6],
        vec![5, 2, 7],
        vec![6, 3, 8],
        vec![7, 4],
    ];

    println!(
        "The Freeman's centralization for my_graph is {} ",
        freemans_centralization(&my_graph)
    );
}

// max node degree in a graph
fn max_degree(graph: &Vec<Vec<usize>>) -> usize {
    graph
        .iter()
        .map(|node_list: &Vec<usize>| node_list.len())
        .collect::<Vec<usize>>()
        .iter()
        .max()
        .unwrap()
        .clone()
}

// star graph of a given size
fn star(size: usize, star: &mut Vec<Vec<usize>>) {
    for i in 0..size {
        star.push(Vec::new());
        if i == 0 {
            for j in 0..size - 1 {
                star[i].push(j + 1);
            }
        } else {
            star[i].push(1);
        }
    }
}

fn freemans_centralization(graph: &Vec<Vec<usize>>) -> f32 {
    let (mut numerator, mut denominator) = (0, 0);

    let mut max = max_degree(graph);

    for node in graph {
        numerator += max - node.len();
    }

    let mut star_graph: Vec<Vec<usize>> = Vec::new();

    star(graph.len(), &mut star_graph);

    max = max_degree(&star_graph);

    for node in star_graph {
        denominator += max - node.len();
    }

    return numerator as f32 / denominator as f32;
}
