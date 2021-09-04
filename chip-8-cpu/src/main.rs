
struct Chip8CPU {
    registers: [u8; 16],
    program_counter: usize,
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_pointer: usize
}

#[non_exhaustive]
struct Registers;
impl Registers {
    pub const ONE: usize = 0;
    pub const TWO: usize = 1;
    pub const THREE: usize = 2;
    pub const FOUR: usize = 3;
    pub const FIVE: usize = 4;
    pub const SIX: usize = 5;
    pub const SEVEN: usize = 6;
    pub const EIGHT: usize = 7;
    pub const NINE: usize = 8;
    pub const TEN: usize = 9;
    pub const ELEVEN: usize = 10;
    pub const TWELVE: usize = 11;
    pub const THIRTEEN: usize = 12;
    pub const FOURTEEN: usize = 13;
    pub const FIFTEEN: usize = 14;
    pub const SIXTEEN: usize = 15;
}

impl Chip8CPU {
    fn read_opcode(&self) -> u16 {
        // CHIP-8 opcodes are u16 values made up of 4 nibbles (half a byte)
        let pc = self.program_counter;
        let op_byte1 = self.memory[pc] as u16;
        // since memory is a list of u8 elements
        let op_byte2 = self.memory[pc+1] as u16;

        // combine both u8 opcode bytes to create the opcode
        // since opcode is 0x0000 = 0x[op_byte1][op_byte2]
        op_byte1 << 8 | op_byte2
    }

    fn run(&mut self) {
        loop {
            let opcode: u16 = self.read_opcode();
            self.program_counter += 2;
            // opcode is represented as 16 bit hexadecimal value such as 0x1234
            // as mentioned in read_opcode, the opcodes consist of 4 nibbles
            // the first nibble is the opcode group
            // the second nibble is the first register for the instruction
            // the third nibble is the second register for the instruction
            // the fourth nibble is the opcode sub group
            // the & operation is to ensure that all of the other bits for the other nibbles
            // are cleared and only the bits for the specific variable remain set
            let opcode_group = ((opcode & 0xF000) >> 12) as u8;
            let register1 = ((opcode & 0x0F00) >> 8) as u8;
            let register2 = ((opcode & 0x00F0) >> 4) as u8;
            let opcode_sub_group = ((opcode & 0x000F) >> 0) as u8;

            match (opcode_group, register1, register2, opcode_sub_group) {
                (0, 0, 0, 0) => break,
                (0, 0 , 0xE, 0xE) => {
                    if self.stack_pointer == 0 {
                        panic!("Stack Underflow")
                    }
                    self.stack_pointer -= 1;
                    let previous_memory_address = self.stack[self.stack_pointer];
                    self.program_counter = previous_memory_address as usize;
                },
                (8, _, _, 4) => self.add_xy(register1, register2),
                (2, _, _, _) => {
                    if self.stack_pointer > self.stack.len() {
                        panic!("Stack Overflow")
                    }

                    // store the current memory location on the stack
                    self.stack[self.stack_pointer] = self.program_counter as u16;
                    // increment stack pointer
                    self.stack_pointer += 1;
                    // set the current memory location to intended memory address `nnn`
                    // Set program counter to nnn for opcode 0x2nnn
                    let address = opcode & 0x0FFF;
                    self.program_counter = address as usize;
                },
                _  =>  todo!("opcode {:04x}", opcode)
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let register1_value = self.registers[x as usize];
        let register2_value = self.registers[y as usize];
        let (val, overflow) = register1_value.overflowing_add(register2_value);
        self.registers[x as usize] = val;
        if overflow {
            // there are 16 registers we are able to store each register
            self.registers[0xf] = 1
        } else {
            self.registers[0xf] = 0
        }
    }


}

fn main() {
    println!("Beep boop! Running CPU...");
    run_program_one();
    run_program_two();
}


fn run_program_one() {
    println!("Running Program ONE...");
    let mut cpu = Chip8CPU {
        registers: [0; 16],
        program_counter: 0,
        memory: [0; 4096],
        stack_pointer: 0,
        stack: [0; 16]
    };

    // Load values into registers 1, 2, 3 and 4
    cpu.registers[Registers::ONE] = 0;
    cpu.registers[Registers::TWO] = 12;
    cpu.registers[Registers::THREE] = 17;
    cpu.registers[Registers::FOUR] = 9;

    let memory = &mut cpu.memory;
    // instructions to add registers 2, 3 and 4 to register 1
    // Load 0x8014 instruction - add register 2 to register 1
    memory[0] = 0x80; memory[1] = 0x14;
    // Load 0x8024 instruction - add register 3 to register 1
    memory[2] = 0x80; memory[3] = 0x24;
    // Load 0x8034 instruction - add register 4 to register 1
    memory[4] = 0x80; memory[5] = 0x34;

    cpu.run();

    // print value of registers
    println!("Finished calculation!");
    println!("Register 1: {}", cpu.registers[Registers::ONE]);
    println!("Register 2: {}", cpu.registers[Registers::TWO]);
    println!("Register 3: {}", cpu.registers[Registers::THREE]);
    println!("Register 4: {}", cpu.registers[Registers::FOUR]);
}

fn run_program_two() {
    println!("Running Program TWO...");
    let mut cpu = Chip8CPU {
        registers: [0; 16],
        program_counter: 0,
        memory: [0; 4096],
        stack_pointer: 0,
        stack: [0; 16]
    };

    cpu.registers[Registers::ONE] = 5;
    cpu.registers[Registers::TWO] = 10;

    let memory = &mut cpu.memory;
    // opcode 2100 // call that function at memory address 0x100
    memory[0x000] = 0x21; memory[0x001] = 0x00;
    // opcode 2100 // call that function at memory address 0x100
    memory[0x002] = 0x21; memory[0x003] = 0x00;
    // opcode 0000 finish program
    memory[0x004] = 0x00; memory[0x005] = 0x00;

    // function add twice - add register 1 to register 0 twice
    // store function at 0x100
    // first add
    memory[0x100] = 0x80; memory[0x101] = 0x14;
    // second add
    memory[0x102] = 0x80; memory[0x103] = 0x14;
    // return to function call
    memory[0x104] = 0x00; memory[0x105] = 0xEE;

    cpu.run();

    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}