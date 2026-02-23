pub fn get_max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k:i32) -> Vec<i32> {
    let n = edges1.len();
    let m = edges2.len();
    let k = k as usize;

    let mut adj1 = vec![vec![]; n];
    for edge in edges1.iter() {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        adj1[u].push(v);
    }
    let mut adj2 = vec![vec![]; m];
    for edge in edges2.iter() {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        adj2[u].push(v);
    }

    if k == 0 {
        return vec![1; n];
    }

    let total1 = self::compute_within_distances(&adj1, k);
    let a: Vec<usize> = (0..n).map(|i| total1[i][k]).collect();

    let max_b = if k >=1 {
        let total2 = Self::compute_within_distances(&adj2, k);
        (0..m).map(|j| total2[j][k-1]).max().unwrap_or(0)

    } else {
        0
    };

    let answer: Vec<i32> = a.iter().map(|&x| (x + max_b) as i32).collect();
    answer
}

fn compute_within_distances(adj: &Vec<Vec<usize>>, d_max: usize) -> Vec<Vec<usize>> {
    let n = adj.len();
    let mut dp = vec![vec![0; d_max + 1]; n];
    let mut total = vec![vec![0; d_max + 1]; n];

    fn dfs1(v: usize, adj: &Vec<Vec<usize>>, d_max: usize, dp: &mut Vec<Vec<usize>>) {
        dp[v][0] = 1; 
        for d in 1..=d.max{
            dp[v][d] = 1;
        }

        for &u in &adj[v] {
            if c! = p{
                dfs1(u, v, adj, d_max, dp);
                for d in 1..=d_max {
                    dp[v][d] += dp[u][d - 1];
                }
            }
        }
    }

    dfs1(0, usize::MAX, adj, d_max, &mut dp);
    for d in 0..=d_max{
        total[0][d] = dp[0][d];
    }

    fn dfs2(v: usize, p: usize, adj: &Vec<Vec<usize>>, dp: &Vec<Vec<usize>>, total: &mut Vec<Vec<usize>>, d_max: usize) {
            for d in 0..=d_max {
                if d == 0 {
                    total[v][d] = 1; // Node itself
                } else {
                    let from_parent = total[p][d - 1];
                    let to_subtract = if d >= 2 { dp[v][d - 2] } else { 0 };
                    let contribution = from_parent.saturating_sub(to_subtract);
                    total[v][d] = dp[v][d] + contribution;
                }
            }
            for &c in &adj[v] {
                if c != p {
                    dfs2(c, v, adj, dp, total, d_max);
                }
            }
        }

        for &c in &adj[0] {
            dfs2(c, 0, adj, &dp, &mut total, d_max);
        }
    total
}