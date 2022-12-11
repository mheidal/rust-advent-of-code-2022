use crate::read_input;

struct RegisterCycle {
    cycle: i32,
    register: i32,
    output: String,
    signal_sum: i32
}

impl RegisterCycle {
    fn increment(&mut self) {
        self.cycle += 1;
        if (self.cycle - 20) % 40 == 0 {
            self.signal_sum += self.register * self.cycle
        }
        let crt_pos = (self.cycle - 1) % 40;
        let reg_offset = crt_pos - self.register;
        if -1 <= reg_offset && reg_offset <= 1 {
            self.output.push('█')
        } else {
            self.output.push(' ')
        }
        if crt_pos == 39 {
            self.output.push('\n')
        }
    }
    fn add(&mut self, value: i32) {
        self.register += value;
    }
}

impl Default for RegisterCycle {
    fn default() -> Self {
        RegisterCycle {
            cycle: 0,
            register: 1,
            output: String::new(),
            signal_sum: 0,
        }
    }
}

fn execute_instructions() -> RegisterCycle {
    let input = read_input::read("inputs/10.txt");
    let mut register_cycle = RegisterCycle::default();
    for line in input.lines() {
        register_cycle.increment();
        match line.trim().split(" ").collect::<Vec<&str>>()[..] {
            ["noop"] => continue,
            ["addx", value_str] => {
                register_cycle.increment();
                let value = value_str.parse::<i32>().expect("Value");
                register_cycle.add(value);
            },
            _ => panic!("Unexpected line"),
        }
    }
    register_cycle
}

fn part_1() {
    let register_cycle = execute_instructions();
    println!("Part 1: {}", register_cycle.signal_sum);
}

fn part_2() {
    let register_cycle = execute_instructions();
    println!("Part 2: \n{}", register_cycle.output);
}

pub fn solve() {
    println!("Day 10");
    part_1();
    part_2();
    println!();
}
