fn main() {
    let mut i: usize = 0;
    for _ in 0..30 {
        println!("{}", i);
        i = (i as isize - 1).rem_euclid(3) as usize;
    }
    println!("{}", usize::MAX);
    println!("{}", isize::MAX);
    println!("{}", isize::MAX as usize);
}