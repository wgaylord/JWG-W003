
use crate::BusInterface;


pub struct BUS {
    devices: Vec<Box<dyn BusInterface>>,
    start: u32,
    length: u32,
}


impl BUS {
    pub fn new(start_address: u32, length: u32) -> BUS {

        BUS{ devices: Vec::new() , start:start_address,length:length }

    }
    
    pub fn add_device<T: BusInterface+ 'static>(&mut self,device: T) -> () {
        self.devices.push(Box::new(device))
    }
}


impl BusInterface for BUS {
    fn get_start_address(&self) -> u32 {
        return self.start;    
    }

    fn get_end_address(&self) -> u32{
        return self.start+self.length-1;    
    }


    fn read(&self,addr: u32) -> Result<u8, ()>{
        let mut out = Vec::<u8>::new();
        if (addr as i64 - self.start as i64) > 0 && (addr as i64 - self.start as i64) < self.length as i64 {
            for device in &self.devices{
                if device.get_start_address() <=addr && device.get_end_address() > addr {
                    match device.read(addr) {
                        Ok(v) => out.push(v),
                        Err(_e) => {},
                    }        
                }
            }

            let mut output: u8 = 0;
            for num in out{
                
                output = output | num;            
            }
            return Ok(output);
        }
        Err(())
    }
    fn write(&mut self,addr:u32,data:u8) -> () {
        if (addr as i64 - self.start as i64) > 0 && (addr as i64 - self.start as i64) < self.length as i64 {
            for device in self.devices.iter_mut(){
                if device.get_start_address() <= addr && device.get_end_address() > addr{
                    device.write(addr,data)
                }
            }
        }
    }

  fn tick(&mut self) -> Result<(),u8>{
        for device in self.devices.iter_mut(){
            match device.tick() {
                Ok(_v) => {},
                Err(e) => {return Err(e)},
            }   
        }  
        Ok(()) 
    }
}


