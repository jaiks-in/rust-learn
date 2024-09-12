pub fn main2(){
    let index=find_first_a(String::from("jia"));
    match index{
        Custom_Option::Some(value)=>println!("it has  index of a is {}",value),
        Custom_Option::None=>println!("a not found")
    }

}

enum Custom_Option{
    Some(i32),
    None
}
fn find_first_a(s:String)->Custom_Option{
    for(index,a) in s.chars().enumerate(){
        if a=='a'{
            return Custom_Option::Some(index as i32);
        }
    }
    return Custom_Option::None;
}