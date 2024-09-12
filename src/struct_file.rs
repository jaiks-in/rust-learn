pub fn struct_reader(){
    struct User{
        active:bool,
        username:String,
        email:String,
        sign_in_count:u64
    }
    struct Rect{
        height:u32,
        width:u32
    }
    impl Rect {
        fn area(&self)->u32{
            self.width*self.height
        }
    }
    let user1=User{
        active:true,
        username:String::from("jaiksin"),
        email:String::from("jaiks@jaiks.com"),
        sign_in_count:1
    };
    let Rect1=Rect{
        height:30,
        width:20
    };
    println!("Rect is {}",Rect1.area());
    print!("username {:?}",user1.username);
}
