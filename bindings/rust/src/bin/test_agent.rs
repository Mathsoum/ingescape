use ingescape::igs;

extern crate ingescape;

fn main() {
    println!("Using ingescape VERSION={}", ingescape::igs::version());
    igs::agent_set_name("rust_agent");
    igs::log_set_file(true, None);
    igs::log_set_console(true);
    let res = igs::start_with_device("en0", 1330);
    println!("res: {}", res);
    loop {}
}
