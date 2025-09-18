fn hoge<'a>(a:i32, b:i32) -> &'a i32{
    let c= a + b;
    return &c;
}

fn main(){
    println!("{}", hoge(8, 2));
}
