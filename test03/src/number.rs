use core::ops::Add;

#[derive(Debug)]
struct CustomNumber {
    value: i32,
}

impl Add for CustomNumber {
    type Output = CustomNumber;

    fn add(self, other: CustomNumber) -> CustomNumber {
        CustomNumber {
            value: self.value + other.value,
        }
    }
}

trait CustomTrait {
    fn add(&self);
}

impl CustomTrait for CustomNumber {
    fn add(&self) {
        println!("Calling add on CustomNumber: {}", self.value);
    }
}

fn call_method(obj: &dyn CustomTrait) {
    obj.add();
}

fn main() {
    let num1 = CustomNumber { value: 15 };
    let num2 = CustomNumber { value: 10 };

    let result = num1 + num2;
    println!("Result of addition: {:?}", result);

    call_method(&result);
}
