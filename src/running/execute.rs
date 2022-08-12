#[path = "scheduler.rs"] mod scheduler;

pub fn run_code(code: String) {
    let op_codes = scheduler::get_op_codes(code);

    for op_code in op_codes {
        //Iterates through each opcode set

        if op_code != "" {
            let first_char = op_code.chars().nth(0).unwrap();
            let hex = format!("{:x}", first_char as u32).parse::<u32>().unwrap();

            //Get data
            let data_set = rem_first_and_last(&op_code);
            let data_split = data_set.split("");
            let vec: Vec<String> = data_split.map(String::from).collect();

            //Run it
            scheduler::run_op_code(hex, vec);
        }
    }
}


fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}