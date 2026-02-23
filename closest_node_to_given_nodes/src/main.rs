pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32{
    fn compute_distances(start: i32, edges: &Vec<i32>) -> Vec<i32> {
        let n = edges.len();
        let mut dist = vec![-1; n];
        let mut current = start;
        let mut distance = 0;

        while current != -1 && dist[current as usize] == -1 {
            dist[current as usize] = distance;
            distance += 1;
            current = edges[current as usize];
        }
        dist
    }

    let n = edges.len();
    let dist1 = compute_distances(node1, &edges);
    let dist2 = compute_distances(node2, &edges);

    let mut min_max_dist = i32::MAX;
    let mut result = -1;

    for i in 0..n {
        let d1 = dist1[i];
        let d2 = dist2[i];

        if d1 != -1 && d2 != -1{
            let max_dist = d1.max(d2);
            if max_dist <  min_max_dist{
                min_max_dist = max_dist;
                result = i as i32;
            }
        }
    }
    result
}