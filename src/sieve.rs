pub fn primes(limit : i32) -> Vec<i32> {

    let mut primes : Vec<i32> = Vec::new();
    let mut composites : Vec<i32> = Vec::new();

    let limit = limit + 1;

    for i in 2..limit {
        println!("checking: {}", i);
        if composites.contains(&i) { continue }

        for j in i*2..limit {
            if j % i == 0 { composites.push(j) }
        }

        primes.push(i);
    }

    return primes;
}