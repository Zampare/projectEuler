fn main(){
    for i in 350i32..500{
        for j in 0i32..400{
            for k in j..i{
                if j.pow(2)+k.pow(2) == i.pow(2){
                   if i+j+k == 1000{
                    println!("{:?}, {:?}, {:?}, {:?}", j, k, i, i*j*k);
                   }
                }
            }
        }
    }
}
