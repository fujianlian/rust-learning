trait PlayTrait {
    fn play(&self);
}

struct PlayOne;
struct PlayTwo;
struct PlayThree;

impl PlayTrait for PlayOne {
    fn play(&self) {
        println!("Calling play on PlayOne");
    }
}

impl PlayTrait for PlayTwo {
    fn play(&self) {
        println!("Calling play on PlayTwo");
    }
}

impl PlayTrait for PlayThree {
    fn play(&self) {
        println!("Calling play on PlayThree");
    }
}

enum PlayEnum {
    Variant1(Box<dyn PlayTrait>),
    Variant2(Box<dyn PlayTrait>),
    Variant3(Box<dyn PlayTrait>),
}

fn main() {
    let vec: Vec<Box<dyn PlayTrait>> = vec![
        Box::new(PlayOne),
        Box::new(PlayTwo),
        Box::new(PlayThree),
    ];

    for item in &vec {
        item.play();
    }
}