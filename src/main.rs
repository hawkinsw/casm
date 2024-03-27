use libcasm::config::machine::Machine;
fn main() {
    let mut f = std::fs::File::open("configs/x86.yaml").unwrap();
    let machine = Machine::new(&mut f);
    println!("{:?}", machine);
}
