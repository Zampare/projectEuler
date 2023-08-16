fn main(){
    let mut trianglenum:i32 = 1;
    for i in 2..30000{
        trianglenum += i;
        let divisors = numberofdivisors(trianglenum);
        if(divisors>400){
            println!("{:?}:{:?}",trianglenum, divisors);
        }
    }
}
fn numberofdivisors(num:i32) -> i32{
    let mut count:i32 = 0;
    for i in 1..(num as f64).sqrt() as i32{
        if num % i == 0{
            count +=2;
        }
    }
    return count;

}
