struct Registers {
  a: u8,
  b: u8,
  c: u8,
  d: u8,
  e: u8,
  f: FlagsRegister
  h: u8,
  l: u8,
}

impl Registers {
  fn get_bc(&self) -> u16 {
      (self.b as u16) << 8
      | self.c as u16
  }
  
  fn set_bc(&mut self, value: u16) {
      self.b = ((value & 0xFF00) >> 8) as u8;
      self.c = (value & 0xFF) as u8;
  }
}

struct FlagsRegister {
  zero: bool,
  subtract: bool,
  half_carry: bool,
  carry: bool
}
const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl std::convert::From<FlagsRegister> for u8  {
   fn from(flag: FlagsRegister) -> u8 {
       (if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
       (if flag.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
       (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
       (if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
   }
}

impl std::convert::From<u8> for FlagsRegister {
   fn from(byte: u8) -> Self {
       let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
       let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
       let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
       let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

       FlagsRegister {
           zero,
           subtract,
           half_carry,
           carry
       }
   }
}

struct CPU {
  registers: Registers,
  pc: u16,
  bus: MemoryBus,
}

struct MemoryBus {
  memory: [u8; 0xFFFF]
}

impl MemoryBus {
  fn read_byte(&self, address: u16) -> u8 {
    self.memory[address as usize]
  }
}

enum Instruction { ADD(ArithmeticTarget), }
enum ArithmeticTarget { A, B, C, D, E, H, L, }

impl Instruction {
  fn from_byte(byte: u8) -> Option<Instruction> {
    match byte {
      0x02 => Some(Instruction::INC(IncDecTarget::BC)),
      0x13 => Some(Instruction::INC(IncDecTarget::DE)),
      _ => /* TODO: Add mapping for rest of instructions */ None
    }
  }
}

impl CPU {
  fn execute(&mut self, instruction: Instruction) {
    match instruction {
      Instruction::ADD(target) => {
        match target {
          ArithmeticTarget::C => {
            let value = self.registers.c;
            let new_value = self.add(value);
            self.registers.a = new_value;
            self.pc.wrapping_add(1);
            // TODO: implement ADD on register C
          }
          _ => { /* TODO: support more targets */ }
        }
      }
      _ => { /* TODO: support more instructions */ }
    }
    fn add(&mut self, value:u8) {
      let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
      self.registers.f.zero = new_value == 0;
      self.registers.f.subtract = false;
      self.registers.f.carry = did_overflow;
      // Half Carry is set if adding the lower nibbles of the value and register A
      // together result in a value bigger than 0xF. If the result is larger than 0xF
      // than the addition caused a carry from the lower nibble to the upper nibble.
      new_value;
    }
    fn step(&mut self) {
      let mut instruction_byte = self.bus.read_byte(self.pc);

      let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte) {
        self.execute(instruction)
      } else {
        panic!("Unkown instruction found for: 0x{:x}", instruction_byte);
      };

      self.pc = next_pc;
    }
  }
}


  