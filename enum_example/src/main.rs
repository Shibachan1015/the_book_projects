#![allow(unused)]
fn main() {
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            println!("{:?}",&self);
        }
    }
    
    let w = Message::Write(String::from("hello"));
    let m = Message::Move { x: 50, y: 70 };
    let c = Message::ChangeColor(01, 00, 01);

    w.call();
    m.call();
    c.call();
}
