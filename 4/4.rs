fn main(){
    for i in 900..999{
        for j in 900..999{
            if(isPalindrome(i*j)){
                println!("{:?}", i*j);
            }
        }
    }
}
fn isPalindrome(num:i32)->bool{
    let str:Vec<char> = num.to_string().chars().collect();
    for i in 0..str.len()/2{
        if(str[i]!=str[str.len()-i-1]){
            return false;
        }
    }
    return true;
}
