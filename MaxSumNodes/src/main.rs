use std::collections::HashMap;

pub fn maximum_value_sum(nums:Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
    let n = nums.len();
    let k = k as i32;
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n];

    // Build the adjacency list
    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        adj[u].push(v);
        adj[v].push(u);
    }

    // dp[x][c] = maximum sum of subtree rooted at x with parity c (1: odd, 0: even)

    let mut dp: Vec<[i32;2]> = vec![[0; 2]; n];
    let mut visited = vec![false; n];

    fn dfs(node: usize, parent: usize, adj: &Vec<Vec<usize>>, nums: &Vec<i32>, k: i32, dp: &mut Vec<[i32;2]>, visited: &mut Vec<bool>) {
        visited[node] = true;
        let children: Vec<usize> = adj[node].iter().filter(|&&x| x != parent).cloned().collect();

        if children.is_empty() {
            dp[node][0] = nums[node];
            dp[node][1] = nums[node] ^ k;
            return;
        }

        for c in 0..2{
            let mut bas_sum = 0;
            let mut child_parities = vec![0; children.len()];
            let total_combinations = 1 << children.len();

            for i in 0..total_combinations{
                let mut sum = 0;
                let mut edge_count = 0;

                for (j, &child) in children.iter().enumerate() {
                    
                    let cy if i & (1 << j) != 0 {
                        child_parities[j] = 1;
                    } else {
                        child_parities[j] = 0;
                    }
                    dfs(child, node, adj, nums, k, dp, visited);
                    sum += dp[child][child_parities[j]];
                    edge_count += child_parities[j];
            }

            for p in 0..2{
                let total_parity = edge_count + p%2;
                if total_parity == c{
                    let node_value = if total_parity == 0 {
                        nums[node]
                    } else {
                        nums[node] ^ k
                    };
                    dp[node][c] = dp[node][c].max(sum + node_value);
                }
            }
            }
        }
    }
    dfs(0, n, &adj, &nums, k, &mut dp, &mut visited);

    dp[0][0].max(dp[0][1]).into()
}