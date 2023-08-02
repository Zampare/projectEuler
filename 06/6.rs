fn main(){
    let squareOfSums:i32 = (1..101).sum::<i32>().pow(2);
    let sumOfSquares:i32 = (1..101).map(|x|x*x).sum();

    println!("{:?}", squareOfSums-sumOfSquares);
}
