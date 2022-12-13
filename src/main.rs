use weighing_problem::odd_balls::OddBalls;

fn iterate_ptr(v: &[u64]) {

    // iterate value at v
    
    for i in 0..v.len() {
        println!("addr = 0x{:08x} = {} at {}", 
                v.as_ptr() as usize + i * std::mem::size_of::<u64>(), 
                v[i], i);
    }
}


fn main() {
    let n = 12;
    let x = 128;
    let delta = 2;
    let odd_balls = OddBalls::new(n, x, delta);

    let v0: &[u64] = &odd_balls.v;
    let mut v: &[u64] = v0;

    while v.len() > 1 {
        println!("Weighs: {:?}", v);
        v = weigh_two_pans(v);
    
    }

    let vector_start = v0.as_ptr() as usize;
    let slice_start = v.as_ptr() as usize;

    let mem_size = std::mem::size_of::<u64>();
    let mem_usize = std::mem::size_of::<usize>();


    let raw_offset = slice_start - vector_start;

    println!("===============================");
    println!("Some Information");
    println!("===============================");
    println!("mem_size of u64 = {}", mem_size);
    println!("mem_usize = {}", mem_usize);
    println!("raw_offset = {}", raw_offset);
    
    let located_pos = (slice_start - vector_start) / std::mem::size_of::<u64>();
    let actual_pos = odd_balls.odd_pos;
   
    println!("acquired_value = {:?}", v);
    println!("located_pos = {:?}", located_pos);
    println!("actual_pos = {:?}", odd_balls.odd_pos);

    println!("value_located_pos = {:?}", &odd_balls.v[located_pos]);
    println!("value_actual_pos = {:?}", &odd_balls.v[actual_pos]);


    println!("adrr = {:p} of vector", v.as_ptr());
    println!("adrr = {:p} of location", v0.as_ptr());

    iterate_ptr(&odd_balls.v);

}

fn weigh_two_pans(v: &[u64]) -> &[u64] {
 
    let n3 = v.len();
    
    if n3 == 1 {
        return &v[0..1];
    }

    let n0 = 0;
    let n1 = n3 / 3;
    let n2 = 2 * n1;

    let s0 = &v[n0..n1];
    let s1 = &v[n1..n2];

    let w0 = s0.iter().sum::<u64>();
    let w1 = s1.iter().sum::<u64>();

    let ge = w0 > w1;
    let le = w0 < w1;
    let eq = w0 == w1;
    
    let res = match (ge | le, eq) {
        (true, false) => &v[n0..n2],
        _ => &v[n2..n3],
    };

    if n1 - n0 <= 1 {
        
        let v0 = *&v[n0];
        let v1 = *&v[n2];
        
        let norm_weight = match (ge | le, eq) {
            (true, false) => v1,
            _ => v0,
        };

        let odd_pos = res.iter().position(|&x| x != norm_weight).unwrap();
        return &res[odd_pos..odd_pos+1];
    }
    res

}

