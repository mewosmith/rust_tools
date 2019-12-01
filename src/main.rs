use std::env;
fn main() {
    print_args();
    for arg in env::args(){
        match arg.as_str() {
            "help" => println!("help me im trapped"),
            _ => println!("{}", arg),
        }
    }
}




fn print_args() {
    for arg in env::args() {
        println!("supplied argument: {}", arg)
    }
}
