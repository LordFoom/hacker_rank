fn hasCircularDependency(n: i32, dependencies: &[Vec<i32>]) -> bool {

    let mut adjacency_list = vec![Vec::new(); n as usize];
    for dependency in dependencies {
        let a = dependency[0];
        let b = dependency[1];

        adjacency_list[a].push(b);
    }

    let mut state = vec![0; n as usize];

    for node in 0..n as usize {
        if state[node]=0 && has_cycle(node, &adjecency_list,&mut state){

        }
    }
    

    false

}

fn has_cycle(node: usize, adjacency_list: &[Vec<usize>], state: &mut [u8] ){
    state[node]=1;
}

