use std::{io, io::prelude::*};

fn main() {
    let reader = io::stdin();
    let lines = reader.lock().lines();
    let values = lines.map(|x| x.unwrap().parse::<i32>().unwrap());
    println!("{}", larger_than_previous(values));
}

fn larger_than_previous(mut v: impl Iterator<Item = i32>) -> i32 {
    let mut prev = v.next();
    let mut num_larger = 0;
    for elem in v {
        if elem > prev.unwrap() {
            num_larger += 1;
        }
        prev = Some(elem);
    }
    num_larger
}

#[test]
fn test_from_example() -> io::Result<()> {
    let v = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let answer = larger_than_previous(v.into_iter());

    assert_eq!(7, answer);
    Ok(())
}
