mod sorts {
    pub fn print2(){
        for ch in 'A'..='z' {
            println!("{ch}");
        } 
    }
}

fn print1() {
    for ch in ('Z'..='a').rev() {
        println!("{ch}");
    } 
}

fn main() {
    print1();
    println!("-------------------------");
    sorts::print2();
} 