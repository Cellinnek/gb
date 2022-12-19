pub struct  CPU {
    pub registers: Registers,
}

// Registers
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8,
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

    fn get_af(&self) -> u16 {
        (self.a as u16) << 8
            | u8::from(self.f) as u16
    }

    fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = FlagsRegister::from((value & 0xFF) as u8);
    }

    fn get_de(&self) -> u16 {
        (self.d as u16) << 8
            | self.e as u16
    }

    fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8
            | self.l as u16
    }

    fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}

#[derive(Copy,Clone)]
pub struct FlagsRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl From<FlagsRegister> for u8  {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
            (if flag.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
            (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
            (if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl From<u8> for FlagsRegister {
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
// Registers

// Instructions
pub enum Instruction {
    ADD(ArithmeticTarget),
    ADDHL(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
    CP(ArithmeticTarget),
    INC(ArithmeticTarget),
    DEC(ArithmeticTarget),
    CCF(),
    SCF(),
    RRA(),
    RLA(),
    RRCA(),
    RRLA(), // Possibly RCLA?
    CPL(),
    BIT(ArithmeticTarget, u8),
    RES(ArithmeticTarget, u8),
    SET(ArithmeticTarget, u8),
    SRL(ArithmeticTarget),
    SLL(ArithmeticTarget),
    RR(ArithmeticTarget),
    RL(ArithmeticTarget),
    RRC(ArithmeticTarget),
    RLC(ArithmeticTarget),
    SRA(ArithmeticTarget),
    SLA(ArithmeticTarget),
    SWAP(ArithmeticTarget),
}

pub enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}

impl CPU {
    pub fn print(&self){
        println!("a: {:#010b} b: {:#010b}",self.registers.a,self.registers.b);
        println!("c: {:#010b} d: {:#010b}",self.registers.c,self.registers.d);
        println!("e: {:#010b} f: {:#010b}",self.registers.e,u8::from(self.registers.f));
        println!("h: {:#010b} l: {:#010b}",self.registers.h,self.registers.l);

    }
    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        self.registers.a = self.add(value);
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        self.registers.a = self.add(value);
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        self.registers.a = self.add(value);
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        self.registers.a = self.add(value);
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        self.registers.a = self.add(value);
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        self.registers.a = self.add(value);
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        self.registers.a = self.add(value);
                    }
                }
            },
            Instruction::ADDHL(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.addhl(value);
                        self.registers.set_hl(new_value);
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.addhl(value);
                        self.registers.set_hl(new_value);
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.addhl(value);
                        self.registers.set_hl(new_value);
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.addhl(value);
                        self.registers.set_hl(new_value);
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.addhl(value);
                        self.registers.set_hl(new_value);
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.addhl(value);
                        self.registers.set_hl(new_value);
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.addhl(value);
                        self.registers.set_hl(new_value);
                    }
                }
            },
            Instruction::ADC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        self.registers.a = self.add(value);
                        self.registers.a = self.add(u8::from(self.registers.f.carry));
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        self.registers.a = self.add(value);
                        self.registers.a = self.add(u8::from(self.registers.f.carry));
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        self.registers.a = self.add(value);
                        self.registers.a = self.add(u8::from(self.registers.f.carry));
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        self.registers.a = self.add(value);
                        self.registers.a = self.add(u8::from(self.registers.f.carry));
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        self.registers.a = self.add(value);
                        self.registers.a = self.add(u8::from(self.registers.f.carry));
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        self.registers.a = self.add(value);
                        self.registers.a = self.add(u8::from(self.registers.f.carry));
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        self.registers.a = self.add(value);
                        self.registers.a = self.add(u8::from(self.registers.f.carry));
                    }
                }
            },
            Instruction::SUB(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        self.registers.a = self.sub(value);
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        self.registers.a = self.sub(value);
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        self.registers.a = self.sub(value);
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        self.registers.a = self.sub(value);
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        self.registers.a = self.sub(value);
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        self.registers.a = self.sub(value);
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        self.registers.a = self.sub(value);
                    }
                }
            },
            Instruction::SBC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        self.registers.a = self.sub(value);
                        self.registers.a = self.sub(u8::from(self.registers.f.carry));
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        self.registers.a = self.sub(value);
                        self.registers.a = self.sub(u8::from(self.registers.f.carry));
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        self.registers.a = self.sub(value);
                        self.registers.a = self.sub(u8::from(self.registers.f.carry));
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        self.registers.a = self.sub(value);
                        self.registers.a = self.sub(u8::from(self.registers.f.carry));
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        self.registers.a = self.sub(value);
                        self.registers.a = self.sub(u8::from(self.registers.f.carry));
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        self.registers.a = self.sub(value);
                        self.registers.a = self.sub(u8::from(self.registers.f.carry));
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        self.registers.a = self.sub(value);
                        self.registers.a = self.sub(u8::from(self.registers.f.carry));
                    }
                }
            },
            Instruction::AND(target) => {
                match target {
                    ArithmeticTarget::A => {
                    }
                    ArithmeticTarget::B => {
                        self.registers.a &= self.registers.b;
                    }
                    ArithmeticTarget::C => {
                        self.registers.a &= self.registers.c;
                    }
                    ArithmeticTarget::D => {
                        self.registers.a &= self.registers.d;
                    }
                    ArithmeticTarget::E => {
                        self.registers.a &= self.registers.e;
                    }
                    ArithmeticTarget::H => {
                        self.registers.a &= self.registers.h;
                    }
                    ArithmeticTarget::L => {
                        self.registers.a &= self.registers.l;
                    }
                }
                self.registers.f.zero = self.registers.a == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = true;
                self.registers.f.carry = false;
            },
            Instruction::OR(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.a |= self.registers.a;
                    }
                    ArithmeticTarget::B => {
                        self.registers.a |= self.registers.b;
                    }
                    ArithmeticTarget::C => {
                        self.registers.a |= self.registers.c;
                    }
                    ArithmeticTarget::D => {
                        self.registers.a |= self.registers.d;
                    }
                    ArithmeticTarget::E => {
                        self.registers.a |= self.registers.e;
                    }
                    ArithmeticTarget::H => {
                        self.registers.a |= self.registers.h;
                    }
                    ArithmeticTarget::L => {
                        self.registers.a |= self.registers.l;
                    }
                }
                self.registers.f.zero = self.registers.a == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = false;
            },
            Instruction::XOR(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.a ^= self.registers.a;
                    }
                    ArithmeticTarget::B => {
                        self.registers.a ^= self.registers.b;
                    }
                    ArithmeticTarget::C => {
                        self.registers.a ^= self.registers.c;
                    }
                    ArithmeticTarget::D => {
                        self.registers.a ^= self.registers.d;
                    }
                    ArithmeticTarget::E => {
                        self.registers.a ^= self.registers.e;
                    }
                    ArithmeticTarget::H => {
                        self.registers.a ^= self.registers.h;
                    }
                    ArithmeticTarget::L => {
                        self.registers.a ^= self.registers.l;
                    }
                }
                self.registers.f.zero = self.registers.a == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = false;
            },
            Instruction::CP(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.f.zero = self.registers.a == self.registers.a;
                        self.registers.f.carry = false;
                        self.registers.f.subtract = (self.registers.a ^ self.registers.a ^ self.sub(self.registers.l)) & 0x10 != 0;
                    }
                    ArithmeticTarget::B => {
                        self.registers.f.zero = self.registers.a == self.registers.b;
                        self.registers.f.carry = self.registers.a < self.registers.b;
                        self.registers.f.subtract = (self.registers.a ^ self.registers.b ^ self.sub(self.registers.l)) & 0x10 != 0;
                    }
                    ArithmeticTarget::C => {
                        self.registers.f.zero = self.registers.a == self.registers.c;
                        self.registers.f.carry = self.registers.a < self.registers.c;
                        self.registers.f.subtract = (self.registers.a ^ self.registers.c ^ self.sub(self.registers.l)) & 0x10 != 0;
                    }
                    ArithmeticTarget::D => {
                        self.registers.f.zero = self.registers.a == self.registers.d;
                        self.registers.f.carry = self.registers.a < self.registers.d;
                        self.registers.f.subtract = (self.registers.a ^ self.registers.d ^ self.sub(self.registers.l)) & 0x10 != 0;
                    }
                    ArithmeticTarget::E => {
                        self.registers.f.zero = self.registers.a == self.registers.e;
                        self.registers.f.carry = self.registers.a < self.registers.e;
                        self.registers.f.subtract = (self.registers.a ^ self.registers.e ^ self.sub(self.registers.l)) & 0x10 != 0;
                    }
                    ArithmeticTarget::H => {
                        self.registers.f.zero = self.registers.a == self.registers.h;
                        self.registers.f.carry = self.registers.a < self.registers.h;
                        self.registers.f.subtract = (self.registers.a ^ self.registers.h ^ self.sub(self.registers.l)) & 0x10 != 0;
                    }
                    ArithmeticTarget::L => {
                        self.registers.f.zero = self.registers.a == self.registers.l;
                        self.registers.f.carry = self.registers.a < self.registers.l;
                        self.registers.f.subtract = (self.registers.a ^ self.registers.l ^ self.sub(self.registers.l)) & 0x10 != 0;
                    }
                }
                self.registers.f.half_carry = false;
            },
            Instruction::INC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.a = self.add(1);
                    }
                    ArithmeticTarget::B => {
                        self.registers.b = self.add(1);
                    }
                    ArithmeticTarget::C => {
                        self.registers.c = self.add(1);
                    }
                    ArithmeticTarget::D => {
                        self.registers.d = self.add(1);
                    }
                    ArithmeticTarget::E => {
                        self.registers.e = self.add(1);
                    }
                    ArithmeticTarget::H => {
                        self.registers.h = self.add(1);
                    }
                    ArithmeticTarget::L => {
                        self.registers.l = self.add(1);
                    }
                }
                self.registers.f.subtract = true;
                self.registers.f.half_carry = true;
            },
            Instruction::DEC(target) => {
                match target {
                    ArithmeticTarget::A => {

                        self.registers.f.half_carry = self.registers.a & 0xf == 0;
                        self.registers.a = self.sub(1);
                        self.registers.f.zero = self.registers.a == 0;
                    }
                    ArithmeticTarget::B => {
                        self.registers.f.half_carry = self.registers.b & 0xf == 0;
                        self.registers.b = self.sub(1);
                        self.registers.f.zero = self.registers.b == 0;
                    }
                    ArithmeticTarget::C => {
                        self.registers.f.half_carry = self.registers.c & 0xf == 0;
                        self.registers.c = self.sub(1);
                        self.registers.f.zero = self.registers.c == 0;
                    }
                    ArithmeticTarget::D => {
                        self.registers.f.half_carry = self.registers.d & 0xf == 0;
                        self.registers.d = self.sub(1);
                        self.registers.f.zero = self.registers.d == 0;
                    }
                    ArithmeticTarget::E => {
                        self.registers.f.half_carry = self.registers.e & 0xf == 0;
                        self.registers.e = self.sub(1);
                        self.registers.f.zero = self.registers.e == 0;
                    }
                    ArithmeticTarget::H => {
                        self.registers.f.half_carry = self.registers.h & 0xf == 0;
                        self.registers.h = self.sub(1);
                        self.registers.f.zero = self.registers.h == 0;
                    }
                    ArithmeticTarget::L => {
                        self.registers.f.half_carry = self.registers.l & 0xf == 0;
                        self.registers.l = self.sub(1);
                        self.registers.f.zero = self.registers.l == 0;
                    }
                }
                self.registers.f.subtract = true;
            },
            Instruction::CCF() => {
                self.registers.f.carry = !self.registers.f.carry;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
            },
            Instruction::SCF() => {
                self.registers.f.carry = true;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
            },
            Instruction::RRA() => {
                let newcarry = (self.registers.a & 1) != 0;
                let oldcarry = self.registers.f.carry as u8;

                self.registers.a = (self.registers.a >> 1) | (oldcarry << 7);

                self.registers.f.carry = newcarry;
                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
            },
            Instruction::RLA() => {
                let newcarry = (self.registers.a >> 7) != 0;
                let oldcarry = self.registers.f.carry as u8;

                self.registers.a = (self.registers.a << 1) | (oldcarry);

                self.registers.f.carry = newcarry;
                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
            },
            Instruction::RRCA() => {
                let c = self.registers.a & 1;

                self.registers.a = (self.registers.a >> 1) | (c << 7);

                self.registers.f.carry = (c != 0);
                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
            },
            Instruction::RRLA() => {
                let c = self.registers.a >> 1;

                self.registers.a = (self.registers.a << 1) | c;

                self.registers.f.carry = (c != 0);
                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
            },
            Instruction::CPL() => {
                self.registers.a = !self.registers.a;

                self.registers.f.subtract = true;
                self.registers.f.half_carry = true;
            },
            Instruction::BIT(target, b) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.f.zero = ((1u8 << b) & self.registers.a) == 0;
                    }
                    ArithmeticTarget::B => {
                        self.registers.f.zero = ((1u8 << b) & self.registers.b) == 0;
                    }
                    ArithmeticTarget::C => {
                        self.registers.f.zero = ((1u8 << b) & self.registers.c) == 0;
                    }
                    ArithmeticTarget::D => {
                        self.registers.f.zero = ((1u8 << b) & self.registers.d) == 0;
                    }
                    ArithmeticTarget::E => {
                        self.registers.f.zero = ((1u8 << b) & self.registers.e) == 0;
                    }
                    ArithmeticTarget::H => {
                        self.registers.f.zero = ((1u8 << b) & self.registers.h) == 0;
                    }
                    ArithmeticTarget::L => {
                        self.registers.f.zero = ((1u8 << b) & self.registers.l) == 0;
                    }
                }
                self.registers.f.subtract = false;
                self.registers.f.half_carry = true;
            },
            Instruction::RES(target, b) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.a &= !(1u8 << b);
                    }
                    ArithmeticTarget::B => {
                        self.registers.b &= !(1u8 << b);
                    }
                    ArithmeticTarget::C => {
                        self.registers.c &= !(1u8 << b);
                    }
                    ArithmeticTarget::D => {
                        self.registers.d &= !(1u8 << b);
                    }
                    ArithmeticTarget::E => {
                        self.registers.e &= !(1u8 << b);
                    }
                    ArithmeticTarget::H => {
                        self.registers.h &= !(1u8 << b);
                    }
                    ArithmeticTarget::L => {
                        self.registers.l &= !(1u8 << b);
                    }
                }
            },
            Instruction::SET(target, b) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.a |= (1u8 << b);
                    }
                    ArithmeticTarget::B => {
                        self.registers.b |= (1u8 << b);
                    }
                    ArithmeticTarget::C => {
                        self.registers.c |= (1u8 << b);
                    }
                    ArithmeticTarget::D => {
                        self.registers.d |= (1u8 << b);
                    }
                    ArithmeticTarget::E => {
                        self.registers.e |= (1u8 << b);
                    }
                    ArithmeticTarget::H => {
                        self.registers.h |= (1u8 << b);
                    }
                    ArithmeticTarget::L => {
                        self.registers.l |= (1u8 << b);
                    }
                }
            },
            Instruction::SRL(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.f.carry = (self.registers.a & 1 != 0);
                        self.registers.a >>= 1;
                        self.registers.f.zero = self.registers.a == 0;
                    }
                    ArithmeticTarget::B => {
                        self.registers.f.carry = (self.registers.a & 1 != 0);
                        self.registers.b >>= 1;
                        self.registers.f.zero = self.registers.a == 0;
                    }
                    ArithmeticTarget::C => {
                        self.registers.f.carry = (self.registers.a & 1 != 0);
                        self.registers.c >>= 1;
                        self.registers.f.zero = self.registers.a == 0;
                    }
                    ArithmeticTarget::D => {
                        self.registers.f.carry = (self.registers.a & 1 != 0);
                        self.registers.d >>= 1;
                        self.registers.f.zero = self.registers.a == 0;
                    }
                    ArithmeticTarget::E => {
                        self.registers.f.carry = (self.registers.a & 1 != 0);
                        self.registers.e >>= 1;
                        self.registers.f.zero = self.registers.a == 0;
                    }
                    ArithmeticTarget::H => {
                        self.registers.f.carry = (self.registers.a & 1 != 0);
                        self.registers.h >>= 1;
                        self.registers.f.zero = self.registers.a == 0;
                    }
                    ArithmeticTarget::L => {
                        self.registers.f.carry = (self.registers.a & 1 != 0);
                        self.registers.l >>= 1;
                        self.registers.f.zero = self.registers.a == 0;
                    }
                }
                self.registers.f.half_carry = false;
                self.registers.f.subtract = false;
            },
            Instruction::SLL(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.f.carry = (self.registers.a & 1 != 0);
                        self.registers.a <<= 1;
                        self.registers.f.zero = self.registers.a == 0;
                    }
                    ArithmeticTarget::B => {
                        self.registers.f.carry = (self.registers.a & 1 != 0);
                        self.registers.a <<= 1;
                        self.registers.f.zero = self.registers.a == 0;
                    }
                    ArithmeticTarget::C => {
                        self.registers.f.carry = (self.registers.a & 1 != 0);
                        self.registers.a <<= 1;
                        self.registers.f.zero = self.registers.a == 0;
                    }
                    ArithmeticTarget::D => {
                        self.registers.f.carry = (self.registers.a & 1 != 0);
                        self.registers.a <<= 1;
                        self.registers.f.zero = self.registers.a == 0;
                    }
                    ArithmeticTarget::E => {
                        self.registers.f.carry = (self.registers.a & 1 != 0);
                        self.registers.a <<= 1;
                        self.registers.f.zero = self.registers.a == 0;
                    }
                    ArithmeticTarget::H => {
                        self.registers.f.carry = (self.registers.a & 1 != 0);
                        self.registers.a <<= 1;
                        self.registers.f.zero = self.registers.a == 0;
                    }
                    ArithmeticTarget::L => {
                        self.registers.f.carry = (self.registers.a & 1 != 0);
                        self.registers.a <<= 1;
                        self.registers.f.zero = self.registers.a == 0;
                    }
                }
                self.registers.f.half_carry = false;
                self.registers.f.subtract = false;
            },
            _ => { /* TODO: support more instructions */ }
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = (new_value == 0);
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }
    fn addhl(&mut self, value: u8) -> u16 {
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value as u16);
        self.registers.f.zero = (new_value == 0);
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }
    fn sub(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        self.registers.f.zero = (new_value == 0);
        self.registers.f.subtract = true;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) < 0xF;
        new_value
    }
}

//Instructions