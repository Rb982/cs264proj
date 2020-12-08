#include <libusb-1.0/libusb.h>
#include <stdio.h>
void callback(struct libusb_transfer *result){
    //
    printf("Callback called");
}
void do_transfer(char endpoint_addr){
    libusb_context* ctx;
    libusb_init(&ctx);
    //Being lazy, hardcoding the device handle
    libusb_device_handle* microphone=libusb_open_device_with_vid_pid(ctx, 22136, 4096);
    int claim = libusb_claim_interface(microphone, 1);
    printf("Claim attempted: %s", libusb_error_name(claim)); 
    //Not sure how to determine the correct number of packets, so seeing if I can just throw a number and call it a day
    struct libusb_transfer* transfer=libusb_alloc_transfer(100);
    char buffer[2000];
    transfer->dev_handle=microphone;
    transfer->endpoint=endpoint_addr;
    transfer->type=1;
    transfer->timeout=0;
    //I think it's correct to say length is just num_packets times max packet size; at any rate, it's the size of buffer, so if I got one of these wrong, I'll find out soon
    transfer->length=2000;
    transfer->num_iso_packets=100;
    transfer->callback=&callback;
    int result = libusb_submit_transfer(transfer);
    printf("%s", libusb_error_name(result));
}

int main(){
    //Some more hardcoding fun
   
   // while(1){
         do_transfer(132);
    //}
    return 0;
}