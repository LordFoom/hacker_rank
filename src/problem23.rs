fn hasCircularDependency(n: i32, dependencies: &[Vec<i32>]) -> bool {

    let mut adjacency_list = vec![Vec::new(); n as usize];
    for dependency in dependencies {
        let a = dependency[0] as usize;
        let b = dependency[1] as usize;

        adjacency_list[a].push(b);
    }

    let mut state = vec![0; n as usize];

    for node in 0..n as usize {
        if state[node]==0 && has_cycle(node, &adjacency_list,&mut state){
            return true;
        }
    }

    // println!("Adjacency list of edges");
    // for edge in adjacency_list{
    //     println!("{} ---> {}", edge[0], edge[1]);
    // }
    

    false

}

fn has_cycle(node: usize, adjacency_list: &[Vec<usize>], state: &mut [u8] ) -> bool{
    state[node]=1;
    for i in 0..adjacency_list[node].len() {
        let next = adjacency_list[node][i];
        if state[next]==1 {
            return true;
        }
        if state[next]==0 && has_cycle(next, adjacency_list, state) {
            return true;
        }
    }
    state[node]=2;
    false
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_just_print_stuff() {
        let v =vec![[1, 0], [2, 1], [3, 2]];
        hasCircularDependency(4,v);
    }
}

