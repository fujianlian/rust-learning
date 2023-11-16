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
    Variant1(PlayOne),
    Variant2(PlayTwo),
    Variant3(PlayThree),
}

fn main() {
    let vec: Vec<PlayEnum> = vec![
        PlayEnum::Variant1(PlayOne),
        PlayEnum::Variant2(PlayTwo),
        PlayEnum::Variant3(PlayThree),
    ];

    // 遍历 Vec，调用各自类型的方法
    for item in &vec {
        match item {
            PlayEnum::Variant1(type1) => type1.play(),
            PlayEnum::Variant2(type2) => type2.play(),
            PlayEnum::Variant3(type3) => type3.play(),
        }
    }
}




