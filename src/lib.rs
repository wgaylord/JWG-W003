pub trait BusInterface {
    fn read(&self,addr: u32) -> Result<u8, ()>; //Read a byte from the memory device
    fn write(&mut self,addr: u32,data: u8) -> (); //Write a byte to the memory device
    fn get_start_address(&self) -> u32; //Get memory start address
    fn get_end_address(&self) -> u32; //Get memory length
    fn tick(&mut self) -> Result<(), u8>; //Ticks interface devices returns a boolean if to cause an inttrupt or not. True == Cause one.
}


pub mod memory;
pub mod bus;
pub mod dma;
