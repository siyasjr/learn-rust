// Problem: There are num_courses labeled 0 to n-1. Given prerequisites, determine if you can finish all courses (detect cycles in a graph).


use std::collections::VecDeque;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut indegree = vec![0; num_courses as usize];
        let mut graph = vec![vec![]; num_courses as usize];
        for p in &prerequisites {
            graph[p[1] as usize].push(p[0] as usize);
            indegree[p[0] as usize] += 1;
        }
        let mut queue = VecDeque::new();
        for i in 0..num_courses as usize {
            if indegree[i] == 0 { queue.push_back(i); }
        }
        let mut count = 0;
        while let Some(node) = queue.pop_front() {
            count += 1;
            for &neighbor in &graph[node] {
                indegree[neighbor] -= 1;
                if indegree[neighbor] == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
        count == num_courses
    }
}
