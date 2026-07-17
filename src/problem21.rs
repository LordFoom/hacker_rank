/*Given n computers labeled 0 to n-1 and a list of bidirectional communication links,
 * find the number of connected components.

Example

Input

n = 4
links = [[0, 1], [2, 3]]

Output

2

Explanation

There are 4 computers: 0, 1, 2, and 3. Links connect 0-1 and 2-3.
Computers 0 and 1 form one group since they are directly connectable. Similarly, 2 and 3 form another group. There are no connections between the two groups, so the number of isolated communication groups is 2.

Input Format

    The first line contains an integer m denoting the length of links.
    The second line denotes the length of individual elements of array links.
    The next m lines contains individual elements of links.
    The last line contains the value of n.
    */
fn countIsolatedCommunicationGroups(links: &[Vec<i32>], n: i32) -> i32 {
    //build an ajacency list
    let mut graph = vec![Vec::new()];

    for link in links {
        let a = link[0] as usize;
        let b = link[1] as usize;
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut visited = vec![false; n as usize];
    let mut component_count = 0;
    for computer in 0..n {
        if visited[computer as usize] {
            continue;
        }
        component_count += 1;
        let mut stack = graph[computer as usize];
        while let Some(current) = stack.pop() {
            if visited[current as usize] {
                continue;
            }
            visited[current] = true;
            for &neighbor in &graph[current as usize] {
                if !visited[neighbor]{
                    stack.push(neighbor);
                }
            }
        }
    }
    component_count

}
