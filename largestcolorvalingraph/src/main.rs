use std::collections::HashMap;
  pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
let n = colors.len();
    let colors: Vec<char> = colors.chars().collect();

    let mut unique_colors: Vec<char> = Vec::new();
    let mut color_map: HashMap<char, usize> = HashMap::new();
    for &color in colors.iter() {
        if !color_map.contains_key(&color) {
            color_map.insert(color, unique_colors.len());
            unique_colors.push(color);
        }
    }

    let k = unique_colors.len();

    let mut adj:Vec<Vec<usize>> = vec![vec![]; n];
    for edge in edges.iter(){
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        adj[u].push(v); 
    }

    let  status: Vec<i32> = vec![0; n]; 
    let mut topo_order: Vec<usize> = Vec::with_capacity(n);
    let mut has_cycle = false;

    fn dfs(node: usize, adj: &Vec<Vec<usize>>, status: &mut Vec<i32>, topo_order: &mut Vec<usize>, has_cycle: &mut bool,){
        if *has_cycle {
            return;
        }
        status[node] = 1;
        for &neighbour in adj[node].iter() {
            if status[neighbour] == 1{
                *has_cycle = true;
                return;
            } 
            if status[neighbour] == 0 {
                dfs(neighbour, adj, status, topo_order, has_cycle);
            }
        }
        status[node] = 2;
        topo_order.push(node);
    }

    for node in 0..n{
        if status[node] == 0 {
            dfs(node, &adj, &mut vec![0; n], &mut topo_order, &mut has_cycle);
        }
        if has_cycle {
            return -1;
        }
    }
        let mut dp: Vec<Vec<i32>> = vec![vec![0; k]; n];
        let mut max_value = 0;

        for &node in topo_order.iter().rev() {
            let color_index = *color_map.get(&colors[node]).unwrap();
            dp[node][color_index] = 1;

            for &neighbour in adj[node].iter() {
                for c in 0..k {
                   let contribution = dp[neighbour][c] + if c == color_index { 1 } else { 0 };
                dp[node][c] = dp[node][c].max(contribution);
                }
            }

            for c in 0..k {
                max_value = max_value.max(dp[node][c]);
            }
            if max_value == n as i32 {
                return max_value;
            }

        }
    max_value
  }

fn main() {
    let colors = "abaca".to_string();
    let edges = vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3]];
    let result = largest_path_value(colors, edges);
    println!("Largest path value: {}", result);
}