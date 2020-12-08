use rusb;
use std::slice;
use std::time::Duration;
fn main() {
    //Step one - list all usb devices, see what I can see about them
    let _ctx = rusb::Context::new();//*
    let devices = match rusb::DeviceList::new() {
        Err(e) => panic!("Device list failed"),
        Ok(t) => t
    };
    println!("Can I access HID? {}", rusb::has_hid_access());
    //let temp = devices+1;
    let dev_iter = devices.iter();
    for val in dev_iter{
        println!("Device is {:#?}", val);
        let desc= match val.device_descriptor() {
            Err(e) => panic!("Failed to get descriptor"),
            Ok(t) => t
        };
        println!("Description of device is {:#?}", desc);
        let configs= match val.config_descriptor(0) {
            Err(e) => panic!("Failed to get descriptor"),
            Ok(t) => t
        }; 
        println!("Device configs is {:#?}", configs);
        let interfaces = configs.interfaces();
        for interface in interfaces{
            println!("Interface descriptors:");
            for descriptor in interface.descriptors(){
                println!("{:#?}", descriptor);
            }
        }
    }
    println!("Hello, world!");
   // */
    let microphone_handle = match rusb::open_device_with_vid_pid(22136, 4096){
        Some(t) => t,
        _ => panic!("Opening device failed")
    };
    let microphone = microphone_handle.device();
    let config_descriptor = match microphone.config_descriptor(0){
        Ok(t) => t,
        Err(_) => panic!("Opening device failed")
    };
    let interfaces = config_descriptor.interfaces();
    let interface_i_want = match interfaces.map(|x| x.descriptors()).flatten().find(|x| x.num_endpoints()==1){
        Some(t) => t,
        _ => panic!("My error handling is perfect")
    };
    let endpoint = match interface_i_want.endpoint_descriptors().last(){
        Some(t) => t,
        _ => panic!("No last endpoint descriptor")
    };
    //for endpoint in interface_i_want.endpoint_descriptors(){
        println!("{:#?}", endpoint);
        println!("dir: {:#?}", endpoint.direction());
        println!("transfer type: {:#?}", endpoint.transfer_type());
        println!("sync type: {:#?}", endpoint.sync_type());
        println!("usage type: {:#?}", endpoint.usage_type());
    //};
    let endpoint_addr = endpoint.address();
    println!("endpoint address is {}", endpoint_addr);
    let mut buffer = vec![1];
    //Ah, glorious documentation.  Did you know notfound isn't an error type the docs list for this method?

    let read_request = match microphone_handle.read_interrupt(endpoint_addr, &mut buffer, Duration::from_millis(10)) {
        Ok(n) => n,
        Err(e) => panic!("Error reading: {:#?}", e)
    };
    println!("{:#?}", buffer);
    
    /*
        Quick sketch of where we are - 
            interface_i_want is the audio streaming interface, with one endpoint, an Isochronous asynchronous data endpoint
            Using endpoint.address(), I can get an endpoint to send and receive from
            Where things get confusing is actually reading from the endpoint; figure out
    */
    //let interface_i_want = descriptors.find(|x| x.num_endpoints()==1);
    /*for interface in interfaces{
        for desc in interface.descriptors(){
            println!("This device has {} endpoints", desc.num_endpoints());
        }
    };*/
    /*
    
    microphone.claim_interface(1);
    microphone.set_alternate_setting(1, 1);*/

    
}
