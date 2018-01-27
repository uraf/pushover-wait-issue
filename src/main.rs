extern crate pushover;

use pushover::SyncAPI;
use pushover::requests::message::SendMessage;

fn main() {
    let token = "eh";
    let user_key = "eh";

    let api = SyncAPI::new().expect("Error creating API");
    for k in 0..20 {
        let msg = SendMessage::new(token, user_key, "msg");
        println!("Sending message #{}", k+1);
        let response = api.send(&msg);
        println!("Response: {:?}", response);
    }
    println!("All good.");
}
