use libcasm::config::machine::Machine;

#[test]
fn simple_parse_test() {
    let mut file = std::fs::File::open("../configs/x86.yaml").unwrap();
    let machine_config = Machine::new(&mut file);
    if machine_config.is_err() {
        let error = machine_config.err();
        println!("error: {:?}", error);
        assert!(false);
        return;
    }
    println!("Machine config: {:?}", machine_config.unwrap());
    assert!(true);
}
