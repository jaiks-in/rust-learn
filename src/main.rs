use crate::string::string_reader;
mod fib;
mod string;
mod struct_file;
mod enums_checker;
mod option_checker;
fn main(){
   let ans=is_even(5);
   println!("this is odd_even checker {}",ans);
   println!("this is fib console{}",fib::fib(4));
   let title=String::from("jaik");
   string_reader(title);
   struct_file::struct_reader();
   enums_checker::enum_check();
   option_checker::main2();
}

fn is_even(num:i32)->bool {

    if num%2==0 {
        return true;
    }else{
        return false;
    }
}
