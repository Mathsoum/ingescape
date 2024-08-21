use ingescape::igs;

fn main() {
    println!("Using ingescape VERSION={}", igs::version());
    igs::agent_set_name("rust_agent");
    igs::log_set_file(true, None);
    igs::log_set_console(true);
    let device = "en0";
    let port = 1337;
    println!(
        "Start with {} {} => result: {}",
        device,
        port,
        igs::start_with_device(device, port)
    );
    loop {}
}
