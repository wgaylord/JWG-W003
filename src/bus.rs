
use crate::BusInterface;


pub struct BUS {
    devices: Vec<Box<dyn BusInterface>>, // This Vector hold all the devices on the bus, have to wrap inside a box because I am allowing all devices impl BusInterface
    start: u32, //This is the start address of the bus, useful if we want to nest buses to break up some more logically.
    length: u32, // Length of the bus, how many addresses does it hold.
}


impl BUS {
    pub fn new(start_address: u32, length: u32) -> BUS { // Just an initalizer
        BUS{ devices: Vec::new() , start:start_address,length:length }
    }
    
    pub fn add_device<T: BusInterface+ 'static>(&mut self,device: T) -> () { //Add a device, we take in anything implmenting BusInterface and wrap it in a box.
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
        let mut out = Vec::<u8>::new(); // This will hold all responses from all devices that are listening for reads on the address (used to allow for bus collisons.)
        if (addr as i64 - self.start as i64) > 0 && (addr as i64 - self.start as i64) < self.length as i64 { //Have to do this weird cast stuff to get the logic happy woth the compiler
            for device in &self.devices{
                if device.get_start_address() <=addr && device.get_end_address() > addr { //Check if the device cares about the address
                    match device.read(addr) {
                        Ok(v) => out.push(v),//If it has something here for us to read add it to the out vec
                        Err(_e) => {}, //Shouldn't get here unless this device cared about the address but then didn't return anything.
                    }        
                }
            }

            let mut output: u8 = 0;
            for num in out{
                output = output | num; //Implement Bus Collisons             
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

  fn tick(&mut self) -> Result<(),u8>{ //Tick all our devices
        for device in self.devices.iter_mut(){
            match device.tick() {
                Ok(_v) => {},
                Err(e) => {return Err(e)},
            }   
        }  
        Ok(()) 
    }
}


