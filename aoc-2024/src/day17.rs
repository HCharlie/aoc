use anyhow::Result;

pub const EXAMPLE_INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

pub fn p1(input_text: &str) -> Result<String> {
    let parts = input_text.split("\n\n").collect::<Vec<&str>>();
    // Parse initial register values from input
    let re = regex::Regex::new(r"Register [A-C]: (\d+)")?;
    let registers: Vec<i64> = parts[0]
        .lines()
        .filter_map(|line| re.captures(line))
        .filter_map(|cap| cap[1].parse().ok())
        .collect();
    let (mut ra, mut rb, mut rc) = (registers[0], registers[1], registers[2]);
    // Parse instructions
    let instructions: Vec<i64> = parts[1]
        .trim_start_matches("Program: ")
        .split(',')
        .filter_map(|n| n.parse().ok())
        .collect();
    // println!("{:?}", registers);
    // println!("{:?}", instructions);
    let mut output: Vec<i64> = Vec::new();

    let mut ip: i64 = 0;
    while ip < instructions.len() as i64 {
        // print!("Instruction pointer: {}\n", ip);
        // println!("{:?}", output);
        // print!("Registers: {:?}\n", (ra, rb, rc));
        let instr = instructions[ip as usize];
        let literal_operand = instructions[(ip + 1) as usize];
        let operand = match literal_operand {
            0|1|2|3 => literal_operand,
            4 => ra,
            5 => rb,
            6 => rc,
            _ => panic!("Invalid operand"),
        };
        match instr {
            0 => ra = ra >> operand,
            1 => rb = rb ^ literal_operand,
            2 => rb = operand % 8,
            3 => if ra != 0 { ip = literal_operand; continue; },
            4 => rb = rc ^ rb,
            5 => output.push(operand % 8),
            6 => rb = ra >> operand,
            7 => rc = ra >> operand,
            _ => panic!("Invalid opcode"),
        }
        ip += 2;

    }
    
    let ans = output.iter().map(|&n| n.to_string()).collect::<Vec<String>>().join(",");
    println!("{:?}", ans);
    Ok(ans)
}


fn _check(a: i64, instructions: &Vec<i64>) -> i64 {
    let mut ra = a;
    let mut rb = 0;
    let mut rc = 0;
    
    let mut output: Vec<i64> = Vec::new();

    let mut ip: i64 = 0;
    while ip < instructions.len() as i64 {
        // print!("Instruction pointer: {}\n", ip);
        // println!("{:?}", output);
        // print!("Registers: {:?}\n", (ra, rb, rc));
        let instr = instructions[ip as usize];
        let literal_operand = instructions[(ip + 1) as usize];
        let operand = match literal_operand {
            0|1|2|3 => literal_operand,
            4 => ra,
            5 => rb,
            6 => rc,
            _ => panic!("Invalid operand"),
        };
        match instr {
            0 => ra = ra >> operand,
            1 => rb = rb ^ literal_operand,
            2 => rb = operand % 8,
            3 => if ra != 0 { ip = literal_operand; continue; },
            4 => rb = rc ^ rb,
            5 => output.push(operand % 8),
            6 => rb = ra >> operand,
            7 => rc = ra >> operand,
            _ => panic!("Invalid opcode"),
        }
        ip += 2;

    }
    println!("{:?}", output);
    return output[0];
}

fn find_min_a(instructions: &Vec<i64>, index: usize, curr_a: i64) -> Option<i64> {
    //the idea behind this function is based on assumption that one 3,0 at the end, and a 5 for output and 0, 3 for right shift 3
    // another idea of writing the recursive function is to pop/trim the last element of the instructions and always compare the last element of the instructions with the output
    if index == instructions.len() {
        return Some(curr_a);
    }
    let instr_index = instructions.len() - 1 - index;
    let expected_output = instructions[instr_index];
    for i in 0..8 {
        let next_a = (curr_a << 3) + i;
        if _check(next_a, &instructions) == expected_output {
            if let Some(result) = find_min_a(instructions, index + 1, next_a) {
                return Some(result);
            }
        }
    }
    None
}

pub fn p2(input_text: &str) -> Result<String> {
    // 0,1, 5,4, 3,0
    // 2,4, 1,1, 7,5, 0,3, 1,4, 4,0, 5,5, 3,0
    let parts = input_text.split("\n\n").collect::<Vec<&str>>();
    // Parse instructions
    let instructions: Vec<i64> = parts[1]
        .trim_start_matches("Program: ")
        .split(',')
        .filter_map(|n| n.parse().ok())
        .collect();
    // println!("{:?}", instructions);
    

    if let Some(min_a) = find_min_a(&instructions, 0, 0) {
        Ok(min_a.to_string())
    } else {
        panic!("not found");
    }
}

