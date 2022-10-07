use emulator::BusInterface;
use std::num::Wrapping;


struct RegisterFile{
  gp: Vec<Wrapping<u32>>, //General Purpose Registers 
  flag: u16, //Flag Register
  status: u16, //Status Register
  pc: Wrapping<u32> //Program Counter
}

struct CPU {
    registers: RegisterFile,
    
    
}
