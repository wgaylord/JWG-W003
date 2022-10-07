
use std::fs;


use crate::BusInterface;

#[derive(Debug)]
pub struct RAM {
    data: Vec<u8>,
    start: u32,
    length: u32,
}

impl RAM {
    pub fn new(start_address: u32, length: u32) -> RAM {
        let mut data = Vec::new();
        data.resize(length as usize, 0);
        RAM{ data: data, start:start_address,length:length }
        
    }
}

impl BusInterface for RAM {
    fn get_start_address(&self) -> u32 {
        return self.start;    
    }

    fn get_end_address(&self) -> u32{
        return self.start+self.length-1;    
    }

    fn read(&self,addr: u32) -> Result<u8, ()>{
        if (addr-self.start) > 0 && (addr-self.start) < self.length {
            return Ok(self.data[(addr-self.start) as usize]);
        }
        Err(())
    }
    fn write(&mut self,addr:u32,data:u8) -> () {
        if (addr as i64 - self.start as i64 ) >= 0 && (addr as i64 - self.start as i64) < self.length as i64 {
            self.data[(addr-self.start) as usize] = data;
        }
    }

    
    fn tick(&mut self) -> Result<(), u8>{
        return Ok(());    
    }
}


#[derive(Debug)]
pub struct ROM {
    data: Vec<u8>,
    start: u32,
    length: u32,
}

impl ROM {
    pub fn new(start_address: u32, length: u32,source_file:&str) -> ROM {

        let mut data = fs::read(source_file).expect("Error Reading file while initalizing a ROM.");
        data.resize(length as usize,0);
        ROM{ data: data, start:start_address,length:length }
    }
}


impl BusInterface for ROM {
    fn get_start_address(&self) -> u32 {
        return self.start;    
    }

    fn get_end_address(&self) -> u32{
        return self.start+self.length-1;    
    }

    fn read(&self,addr: u32) -> Result<u8, ()>{
        if (addr as i64 - self.start as i64) >= 0 && (addr as i64 - self.start as i64) < self.length as i64 {
            return Ok(self.data[(addr-self.start) as usize]);
        }
        Err(())
    }
    fn write(&mut self,_addr:u32,_data:u8) -> () {
     
    }


    fn tick(&mut self) -> Result<(), u8>{
        return Ok(());    
    }
}
