use std::env;
use std::sync::OnceCell;



fn set_program_name(name: String) {
    PROGRAM_NAME.set(name).expect("PROGRAM_NAME was already set");
}
 
fn get_program_name() -> &'static str {
    PROGRAM_NAME.get().expect("PROGRAM_NAME was not set")
}

fn process_arguments(_args: &[String]) {
    // 这里不直接使用全局变量，但你可以这样做
    println!("Processing arguments...");
}
