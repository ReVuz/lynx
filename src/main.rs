use std::io::{self,Read};

fn main(){
    for b in io::stdin().bytes(){
        let c:char = b.unwrap() as char;
        println!("{}",c);
    }
}