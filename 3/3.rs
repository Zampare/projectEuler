fn main(){
    let bignum:i64 = 600851475143;
    for i in 2..((bignum as f64).sqrt()) as i32{
        if(bignum % i as i64 == 0){
            if(isPrime(i)){
                println!("{:?}", i);
            }
        }
    }
}
fn isPrime(num:i32) -> bool{
    for i in 2..((num as f64).sqrt()) as i32{
        if num % i == 0{
            return false;
        }
    }
    return true;
}

