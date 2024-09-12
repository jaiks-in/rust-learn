pub fn fib(num:i32)->i32{
    let mut first_num=0;
    let mut second_num:i32=1;
    if num==0{
        return first_num;
    }
    if num==1{
        return second_num;
    }

    for _ in 0..num-1{
        let temp=second_num;
        second_num=second_num+first_num;
        first_num=temp;
    }
    return second_num;
}