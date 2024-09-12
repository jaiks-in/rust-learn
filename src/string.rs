//src/string.rs
pub fn string_reader(title:String)->usize{
let  string_num=title.chars().count();
println!("this is char counter{}",string_num);
return string_num;
}