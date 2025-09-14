fn main() {
    let num = add_one::rand_num(0, 10);
    println!("{num} + 1 = {}", add_one::add_one(num));
}
