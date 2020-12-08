#include <libusb-1.0/libusb.h>
#include <stdio.h>
#include<stdlib.h>
#include<unistd.h>
void callback(struct libusb_transfer *result){
    //
    //printf("Callback called");
    printf("%s", result->buffer);
}
void do_transfer(unsigned char endpoint_addr, libusb_device_handle* microphone, int count){
    
    //Not sure how to determine the correct number of packets, so seeing if I can just throw a number and call it a day
    struct libusb_transfer* transfer=libusb_alloc_transfer(100);
    unsigned char* buffer=malloc(2000);
    transfer->dev_handle=microphone;
    transfer->endpoint=endpoint_addr;
    transfer->type=1;
    transfer->timeout=0;
    transfer->buffer = buffer;
    //I think it's correct to say length is just num_packets times max packet size; at any rate, it's the size of buffer, so if I got one of these wrong, I'll find out soon
    transfer->length=2000;
    transfer->num_iso_packets=100;
    transfer->callback=&callback;
    int result = libusb_submit_transfer(transfer);
    printf("attempt %d: %s", count, libusb_error_name(result));
}

int main(){
    libusb_context* ctx;
    libusb_init(&ctx);
    //Being lazy, hardcoding the device handle
    libusb_device_handle* microphone=libusb_open_device_with_vid_pid(ctx, 22136, 4096);
    int detach_attempt=libusb_detach_kernel_driver(microphone,1);
    printf("Detach attempted: %s", libusb_error_name(detach_attempt));
    int claim = libusb_claim_interface(microphone, 1);
    sleep(5);
    printf("Claim attempted: %s", libusb_error_name(claim)); 
    //Some more hardcoding fun
    
   
    for(int i=0; i<100; i++){
         do_transfer(132, microphone, i);
    }
    sleep(10);
    //return 0;
}