fn hasCircularDependency(n: i32, dependencies: &[Vec<i32>]) -> bool {

    let mut adjacency_list = vec![Vec::new(); n as usize];
    for dependency in dependencies {
        let a = dependency[a];
        let b = dependency[b];

        adjacency_list[a].push(b);
        adjacency_list[b].push(a);
    }
    

    false

}

