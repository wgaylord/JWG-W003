
use crate::BusInterface;

#[derive(Clone)]
pub struct DMAEntry{
    src_addr: u32, //Address that the source area of memory starts at
    src_length: u16, //Number of bytes to copy.
    dst_addr: u32, //Address that the destantation area of memory starts at
    dst_length: u16, //Number of bytes it has. (Allows for more then 1 source to be mapped to a destantation.)
    flags: u8, //Flags for the transfer
    transfer_counter: u8, // How many times to run the transfer
    clock_div: u16, // This is used to say how many times to divide down the DMA clock to run the transfers
}

pub struct DMA {
    bus: Box<dyn BusInterface>, //Have the bus in our DMA
    dma_channels: Vec<DMAEntry>, //This is so we can have more then 1 DMA channel
    dma_base_addr: u32, //This is the base address of the dma channel regisers
    dma_channel_count: u8, //Count of how many DMA channels we have.
}


impl DMA {
    pub fn new(bus: Box<dyn BusInterface>, base_addr: u32,channel_count: u8) -> DMA { //Initalize the DMA unit
        let mut dma_chan = Vec::new();
        dma_chan.resize(channel_count as usize,DMAEntry{src_addr:0,src_length:0,dst_addr:0,dst_length:0,flags:0,transfer_counter:0,clock_div:0}); //Initalize our DMA Channel Registers
        DMA{ bus:bus,dma_channels: dma_chan , dma_base_addr:base_addr,dma_channel_count:channel_count }
    }
    fn is_dma_register(&self,addr:u32)-> bool { //Check if we are configuring a DMA register
        if (addr as i64 - self.dma_base_addr as i64) > 0 && (addr as i64 - self.dma_base_addr as i64 ) < (self.dma_channel_count as i64 *16){
            return true
        }
        return false
    }
 
}


impl BusInterface for DMA {
    fn get_start_address(&self) -> u32 { //DMA wraps the bus so we have the BUS's start address.
        return self.bus.get_start_address();    
    }

    fn get_end_address(&self) -> u32{ //DMA wraps the bus so we have the BUS's start end.
        return self.bus.get_end_address();    
    }


    fn read(&self,addr: u32) -> Result<u8, ()>{ //Check if its our registers if not read from the actual bus
        if self.is_dma_register(addr){
                 return Ok(0);
            }else{

                return self.bus.read(addr);
            }

    }
    fn write(&mut self,addr:u32,data:u8) -> () { //Check if its our registers if not write to the actual bus
       if self.is_dma_register(addr){
               
            }else{

            self.bus.write(addr,data);
            }
    }

  fn tick(&mut self) -> Result<(),u8>{ //Tick the bus if we get ticked.
        return self.bus.tick()
    }
}


