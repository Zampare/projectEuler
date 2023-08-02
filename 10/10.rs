

fn main(){
    println!("{:?}",(2i64..2000000).filter(|x| isPrime(*x)).sum::<i64>());
}


fn isPrime(num:i64) -> bool{
    for i in 2..((num as f64).sqrt()+1.0) as i32{
        if num % i as i64 == 0{
            return false;
        }
    }
    return true;
}


