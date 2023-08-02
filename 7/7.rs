fn main(){
    let mut i:i32 = 0;
    let mut counter:i32 = 10001;
    while(counter >=0){
        i+=1;
        if(isPrime(i)){
            counter-=1;
        }
    }
    println!("{:?}",i);
}
fn isPrime(num:i32) -> bool{
    for i in 2..((num as f64).sqrt()+1.0) as i32{
        if num % i == 0{
            return false;
        }
    }
    return true;
}
