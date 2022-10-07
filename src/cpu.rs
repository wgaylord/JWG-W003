use emulator::BusInterface;

struct RegisterFile{
  gp: Vec<u32>,
  flag: u16,
  status: u16,
  pc: u32
}

struct CPU {
    registers: RegisterFile,
    
    
}
