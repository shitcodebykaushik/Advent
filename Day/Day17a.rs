struct Computer {
    register_a: usize,
    register_b: usize,
    register_c: usize,// A = 0, B = 1, C = 2
    ip: usize,
    program: Vec<u8>,
    output: Vec<u8>,
}

impl Computer {
    fn new(a: usize, b: usize, c: usize, program: Vec<u8>) -> Self {
        Self {
            register_a: a,
            register_b: b,
            register_c: c,
            ip: 0,
            program,
            output: Vec::new(),
        }
    }

    fn run(&mut self) {
        while self.ip < self.program.len() {
            let opcode = self.program[self.ip];
            let operand = self.program[self.ip + 1];
            match opcode {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => self.jnz(operand),
                4 => self.bxc(),
                5 => self.out(operand),
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => break, // Halt on invalid opcode
            }

        }
    }

    fn get_combo_value(&self, operand: u8) -> usize {
        match operand {
            0..=3 => operand as usize,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Invalid combo operand"),
        }
    }

    fn adv(&mut self, operand: u8) {
        self.register_a = self.register_a >> self.get_combo_value(operand);
        self.ip += 2;
    }

    fn bxl(&mut self, operand: u8) {
        self.register_b ^= operand as usize;
        self.ip += 2;
    }

    fn bst(&mut self, operand: u8) {
        self.register_b = self.get_combo_value(operand) % 8;
        self.ip += 2;
    }

    fn jnz(&mut self, operand: u8) {
        if self.register_a != 0 {
            self.ip = operand as usize;
        }
        else { self.ip += 2; }
    }

    fn bxc(&mut self) {
        self.register_b = self.register_b ^ self.register_c;
        self.ip += 2;
    }

    fn out(&mut self, operand: u8) {
        self.output.push((self.get_combo_value(operand) % 8) as u8);
        self.ip += 2;
    }

    fn bdv(&mut self, operand: u8) {
        self.register_b = self.register_a >> self.get_combo_value(operand);
        self.ip += 2;
    }

    fn cdv(&mut self, operand: u8) {
        self.register_c = self.register_a >> self.get_combo_value(operand);
        self.ip += 2;
    }


    fn reverse_eng(&mut self)  {
        self.output = Vec::new();
        while self.ip < self.program.len() {
            let opcode = self.program[self.ip];
            let operand = self.program[self.ip + 1];
            match opcode {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => panic!("No idea what to do here, but I dont have one of these"),
                4 => self.bxc(),
                5 => {
                    self.out(operand);
                    break; // break because we have to re-run for each output
                }
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => break,
            }
        }
    }
    
    
}
fn solve_for_a(target: &[u8], a: usize, program: &[u8]) -> Option<usize> {
    if target.is_empty() {
        return Some(a);
    }

    for t in 0..8 {
        let candidate_a = (a << 3) | t;
        let mut computer = Computer::new(candidate_a, 0, 0, program.to_vec());
        computer.reverse_eng();
        if computer.output.len() >0 {
            if computer.output.last() == target.last() {
                if let Some(result) = solve_for_a(&target[..target.len() - 1], candidate_a, program) {
                    return Some(result);
                }
            }
        }
    }
    None
}


fn main() {
    let initial_a = 37293246;
    let initial_b = 0;
    let initial_c = 0;
    let program = vec![2,4,1,7,7,5,1,7,0,3,4,1,5,5,3,0]; // Replace with puzzle input

    let mut computer = Computer::new(initial_a, initial_b, initial_c, program.clone());
    computer.run();
    let output = computer.output.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",");

    println!("{}", output);

    

    if let Some(a) = solve_for_a(&program, 0, &program) {
        println!("The lowest positive value of A is: {}", a);
    } else {
        println!("No solution found");
    }
}