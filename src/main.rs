use crate::string::string_reader;
mod fib;
mod string;
fn main(){
   let ans=is_even(5);
   println!("this is odd_even checker {}",ans);
   println!("this is fib console{}",fib::fib(4));
   let title=String::from("jaik");
   string_reader(title);
}

fn is_even(num:i32)->bool {

    if num%2==0 {
        return true;
    }else{
        return false;
    }
}
