#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

struct ColorTuple(u8, u8, u8);

#[derive(Debug)]
enum Message {
    // DONE: Define the different variants used below.
    Move (Point),
    Echo (String),
    ChangeColor (u8, u8, u8),
    Quit,
    Resize { width: u64, height: u64 },
}

impl From<ColorTuple> for Message {
    fn from(color: ColorTuple) -> Self {
        Message::ChangeColor(color.0, color.1, color.2)
    }
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
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
