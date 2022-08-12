pub fn get_op_codes(code: String) -> Vec<String> {
    let split = code.split("");
    let vec: Vec<String> = split.map(String::from).collect();

    return vec;
}


pub fn run_op_code(code: u32, params: Vec<String>) {
    match code {
        80 => println!("{}", params.join("")),

        x => panic!("Unexpected invalid token {:?}", x),
    }
}