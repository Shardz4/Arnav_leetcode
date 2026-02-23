pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
    fn dfs(
        node: usize,
        parent: isize,
        depth: usize,
        children: &Vec<Vec<usize>>,
        color: &mut Vec<usize>,
    ) -> i32 {
        let mut res =  - (depth %2) as i32;
        color[node] = depth % 2;
        for &child in &children[node] {
            if child as isize == parent {
                continue;
            }
            res += dfs(child, node as isize, depth + 1, children, color);
        }
        res
    }

    fn build (edges: &Vec<Vec<i32>>, color: &mut Vec<usize>) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut children = vec![vec![]; n];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edges[1] as usize;
            children[u].push(v);
            children[v].push(u);
        }
        let res = dfs(0, -1, 0, &children, color);
        vec![res, (n as i32) - res]
    }

    let n1 = edges1.len() +1;
    let n2 = edges2.len() + 1;
    let mut color1 = vec![0; n1];
    let mut color2 = vec![0; n2];
    let count1 = build(&edges1, &mut color1);
    let count2 = build(&edges2, &mut color2);
    let mut result = vec![0; n1];
    for i in 0..n1 {
        result[i] = count1[color1[i]] + count2[0].max(count2[1]);
    }
    result
}

fn main() {
    let edges1 = vec![vec![0, 1], vec![0, 2], vec![1, 3]];
    let edges2 = vec![vec![0, 1], vec![0, 2], vec![1, 3]];
    let result = max_target_nodes(edges1, edges2);
    println!("{:?}", result); // Output: [4, 4, 4, 4]
}