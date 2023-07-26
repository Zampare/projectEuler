fn main(){
    let mut x: (i32, i32, i32) = (1,2,3);
    let mut sum:i32 = 0;
    while x.1 <=4000000 {
        if x.1 % 2 == 0{
            sum += x.1;
        }
        x.0 = x.1;
        x.1 = x.2;
        x.2 = x.0 + x.1;  
    }
    println!("{:?}", sum);
}