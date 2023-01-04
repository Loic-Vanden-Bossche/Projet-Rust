use std::env;

pub fn parse_args() -> (String, String){
    let args: Vec<String> = env::args().collect();
    let mut name = "macaron".to_string();
    let mut ip = "127.0.0.1:7878".to_string();
    let mut next = 0;
    for arg in args {
        if arg == "--name" {
            next = 1;
        }
        if arg == "--ip" {
            next = 2;
        }
        if next == 1 {
            name = arg;
        }else if next == 2 {
            ip = arg
        }
    }
    (name, ip)
}