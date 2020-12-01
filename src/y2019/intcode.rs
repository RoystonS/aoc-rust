pub type InstructionByte = isize;
pub type InstructionBytes = Vec<InstructionByte>;
pub type MemoryData = Vec<isize>;

pub struct IntCodeInterpreter {
    pub memory: InstructionBytes,
    ip: usize,
    inputs: MemoryData,
    relative_base: usize,
}

#[derive(Debug)]
pub enum Action {
    Output(isize),
    Halt,
}

#[derive(Debug)]
pub enum Parameter {
    Position(usize),
    Immediate(isize),
    Relative(isize),
}
impl Parameter {
    pub fn new(mode: isize, value: InstructionByte) -> Parameter {
        match mode {
            0 => Parameter::Position(value as usize),
            1 => Parameter::Immediate(value),
            2 => Parameter::Relative(value),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Add(Parameter, Parameter, Parameter),
    Multiply(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equals(Parameter, Parameter, Parameter),
    AdjustRelativeBase(Parameter),
    Halt,
}

impl IntCodeInterpreter {
    pub fn new(instructions: &MemoryData) -> Self {
        Self {
            ip: 0,
            memory: instructions.clone(),
            inputs: Vec::new(),
            relative_base: 0,
        }
    }

    fn read(&mut self) -> InstructionByte {
        let x = self.memory[self.ip];
        self.ip += 1;
        x
    }

    fn next(&mut self) -> Instruction {
        let inst_byte = self.read();
        let opcode = inst_byte % 100;
        let mode1 = (inst_byte / 100) % 10;
        let mode2 = (inst_byte / 1000) % 10;
        let mode3 = (inst_byte / 10000) % 10;

        match opcode {
            1 => Instruction::Add(
                Parameter::new(mode1, self.read()),
                Parameter::new(mode2, self.read()),
                Parameter::new(mode3, self.read()),
            ),
            2 => Instruction::Multiply(
                Parameter::new(mode1, self.read()),
                Parameter::new(mode2, self.read()),
                Parameter::new(mode3, self.read()),
            ),
            3 => Instruction::Input(Parameter::new(mode1, self.read())),
            4 => Instruction::Output(Parameter::new(mode1, self.read())),
            5 => Instruction::JumpIfTrue(
                Parameter::new(mode1, self.read()),
                Parameter::new(mode2, self.read()),
            ),
            6 => Instruction::JumpIfFalse(
                Parameter::new(mode1, self.read()),
                Parameter::new(mode2, self.read()),
            ),
            7 => Instruction::LessThan(
                Parameter::new(mode1, self.read()),
                Parameter::new(mode2, self.read()),
                Parameter::new(mode3, self.read()),
            ),
            8 => Instruction::Equals(
                Parameter::new(mode1, self.read()),
                Parameter::new(mode2, self.read()),
                Parameter::new(mode3, self.read()),
            ),
            9 => Instruction::AdjustRelativeBase(Parameter::new(mode1, self.read())),
            99 => Instruction::Halt,
            _ => unimplemented!("Unexpected opcode"),
        }
    }

    fn get_parameter(&mut self, parameter: Parameter) -> isize {
        let final_pos;

        match parameter {
            Parameter::Immediate(val) => { return val; },
            Parameter::Position(pos) =>  { final_pos = pos; },
            Parameter::Relative(pos) => {
                final_pos = (self.relative_base as isize + pos) as usize;
            }
        }

        self.grow_memory_to(final_pos);
        self.memory[final_pos]
    }

    fn write(&mut self, to: Parameter, value: isize) {
        let final_pos;

        match to {
            Parameter::Position(pos) => {
                final_pos = pos;
            }
            Parameter::Immediate(_) => unimplemented!("Cannot write to immediate"),
            Parameter::Relative(pos) => {
                final_pos = (self.relative_base as isize + pos) as usize;
            }
        }
        self.grow_memory_to(final_pos);

        self.memory[final_pos] = value;
    }

    fn grow_memory_to(&mut self, size: usize) {
        loop {
            let len = self.memory.len();
            if len <= size {
                self.memory.push(0);
            } else {
                break;
            }
        }
    }

    fn read_input(&mut self) -> Option<isize> {
        if self.inputs.len() > 0 {
            Some(self.inputs.remove(0))
        } else {
            None
        }
    }

    pub fn write_input(&mut self, input: InstructionByte) {
        self.inputs.push(input);
    }

    pub fn execute(&mut self) -> Option<Action> {
        let inst = self.next();
        let mut result = None;

        match inst {
            Instruction::Add(lhs, rhs, output) => {
                let val = self.get_parameter(lhs) + self.get_parameter(rhs);
                self.write(output, val);
            }
            Instruction::Multiply(lhs, rhs, output) => {
                let val = self.get_parameter(lhs) * self.get_parameter(rhs);
                self.write(output, val);
            }
            Instruction::Input(pos) => {
                let val = self.read_input();
                if let Some(value) = val {
                    self.write(pos, value);
                } else {
                    unimplemented!("input exhausted");
                }
            }
            Instruction::Output(pos) => {
                let value = self.get_parameter(pos);
                result = Some(Action::Output(value))
            }
            Instruction::JumpIfTrue(value, target) => {
                let value = self.get_parameter(value);
                if value != 0 {
                    let target = self.get_parameter(target) as usize;
                    self.ip = target;
                }
            }
            Instruction::JumpIfFalse(value, target) => {
                let value = self.get_parameter(value);
                if value == 0 {
                    let target = self.get_parameter(target) as usize;
                    self.ip = target;
                }
            }
            Instruction::LessThan(op1, op2, target) => {
                let value1 = self.get_parameter(op1);
                let value2 = self.get_parameter(op2);
                self.write(target, if value1 < value2 { 1 } else { 0 });
            }
            Instruction::Equals(op1, op2, target) => {
                let value1 = self.get_parameter(op1);
                let value2 = self.get_parameter(op2);
                self.write(target, if value1 == value2 { 1 } else { 0 });
            }
            Instruction::AdjustRelativeBase(delta) => {
                let value = self.get_parameter(delta);
                let final_base = self.relative_base as isize + value;
                self.relative_base = final_base as usize;
            }

            Instruction::Halt => {
                result = Some(Action::Halt);
            }
        }

        result
    }

    pub fn run(&mut self) -> Action {
        loop {
            let action_option = self.execute();

            if let Some(action) = action_option {
                return action;
            }
        }
    }
}
