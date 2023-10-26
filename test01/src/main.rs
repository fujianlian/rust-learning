
mod mod_one;
mod mod_two;

fn main() {
    mod_one::sort_print();
    println!("-------------------------");
    mod_two::sort::sort_print();
} 