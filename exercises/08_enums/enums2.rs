#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

// you can use the above struct in the enum below or you can use the Resize struct syntax that defines it in the enum.

#[derive(Debug)]
enum Message {
    Resize { width: i32, height: i32 },
    Move (Point),
    Echo (String),
    ChangeColor (i32, i32, i32),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 20 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
