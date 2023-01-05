use crate::helpers::reader::read_file;
use std::collections::HashSet;

pub fn solution() {
    let string = read_file("10");
    let lines: Vec<&str> = string.split('\n').collect();

    let mut part_1_state_machine = Part1StateMachine {
        cycle: 0,
        x: 1,
        signal_strength_snapshots: HashSet::new(),
    };
    part_1_state_machine.run_instructions(&lines);
    println!("Part 1: {}", part_1_state_machine.get_signal_strength_sum());

    let mut part_2_state_machine = Part2StateMachine {
        row_length: 40,
        pixel_position: 0,
        x: 1,
        crt: String::from(""),
    };
    part_2_state_machine.run_instructions(&lines);
    print!("Part 2:\n{}", part_2_state_machine.crt);
}

struct Part1StateMachine {
    cycle: i32,
    x: i32,
    signal_strength_snapshots: HashSet<i32>,
}

impl Part1StateMachine {
    fn tick(&mut self) {
        self.cycle += 1;
        if (self.cycle + 20) % 40 == 0 {
            self.signal_strength_snapshots.insert(self.x * self.cycle);
        }
    }
    fn addx(&mut self, incr: i32) {
        self.tick();
        self.tick();
        self.x += incr;
    }
    fn get_signal_strength_sum(&mut self) -> i32 {
        self.signal_strength_snapshots
            .iter()
            .fold(0, |acc, strength| acc + strength)
    }
    fn run_instructions(&mut self, lines: &Vec<&str>) {
        for line in lines {
            let instruction: Vec<&str> = line.split(' ').collect();

            match instruction[0] {
                "noop" => self.tick(),
                "addx" => self.addx(instruction[1].parse::<i32>().expect("Should be an int")),
                _ => unimplemented!(),
            }
        }
    }
}

struct Part2StateMachine {
    row_length: i32,
    pixel_position: i32,
    x: i32,
    crt: String,
}

impl Part2StateMachine {
    fn tick(&mut self) {
        self.crt.push(self.get_pixel());
        self.pixel_position += 1;
        if self.pixel_position == self.row_length {
            self.reset_pixel_position();
        }
    }
    fn addx(&mut self, incr: i32) {
        self.tick();
        self.tick();
        self.x += incr;
    }
    fn sprite_is_visible(&self) -> bool {
        (self.x - 1..=self.x + 1).contains(&self.pixel_position)
    }
    fn get_pixel(&self) -> char {
        if self.sprite_is_visible() {
            '#'
        } else {
            '.'
        }
    }
    fn reset_pixel_position(&mut self) {
        self.crt.push('\n');
        self.pixel_position = 0;
    }
    fn run_instructions(&mut self, lines: &Vec<&str>) {
        for line in lines {
            let instruction: Vec<&str> = line.split(' ').collect();

            match instruction[0] {
                "noop" => self.tick(),
                "addx" => self.addx(instruction[1].parse::<i32>().expect("Should be an int")),
                _ => unimplemented!(),
            }
        }
    }
}
