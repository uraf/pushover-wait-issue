extern crate pushover;

use std::{thread, time};
use pushover::SyncAPI;
use pushover::requests::message::SendMessage;

fn main() {
    let api = SyncAPI::new().expect("Error creating API");
    for k in 0..20 {
        let delay_ms = 2500 + 100*k;
        let delay = time::Duration::from_millis(delay_ms);
        let msg = SendMessage::new("token", "key", "msg");
        
        println!("Sending message #{} and waiting {} ms", k+1, delay_ms);
        // let response = api.send_with_timeout(&msg, delay);
        let response = api.send(&msg);
        println!("Response: {:?}", response);
        
        thread::sleep(delay);
    }
    println!("All good.");
}

/*
Output with timeout:

Sending message #1 and waiting 2500 ms
Response: Err(Error(PushoverError { status: 0, errors: ["application token is invalid"], request: "17e4d283-c5bf-49b5-93c4-aaaf6c7b0ac0" }, State { next_error: None, backtrace: None }))
Sending message #2 and waiting 2600 ms
Response: Err(Error(PushoverError { status: 0, errors: ["application token is invalid"], request: "126e9bac-7108-463c-adf5-bf32eb2d38b6" }, State { next_error: None, backtrace: None }))
Sending message #3 and waiting 2700 ms
Response: Err(Error(PushoverError { status: 0, errors: ["application token is invalid"], request: "79d3bd34-e5bc-4baa-8147-3c2acfafb44a" }, State { next_error: None, backtrace: None }))
Sending message #4 and waiting 2800 ms
Response: Err(Error(PushoverError { status: 0, errors: ["application token is invalid"], request: "6cf44f65-d199-4893-94a3-4077152e407a" }, State { next_error: None, backtrace: None }))
Sending message #5 and waiting 2900 ms
Response: Err(Error(PushoverError { status: 0, errors: ["application token is invalid"], request: "47210310-7311-46ba-abb2-91d497991173" }, State { next_error: None, backtrace: None }))
Sending message #6 and waiting 3000 ms
Response: Err(Error(Hyper(Incomplete), State { next_error: None, backtrace: None }))
Sending message #7 and waiting 3100 ms
Response: Err(Error(PushoverError { status: 0, errors: ["application token is invalid"], request: "009cacb6-10ec-4fc4-9ce6-7952d2034f82" }, State { next_error: None, backtrace: None }))
Sending message #8 and waiting 3200 ms
Response: Ok(None)
Sending message #9 and waiting 3300 ms
Response: Err(Error(PushoverError { status: 0, errors: ["application token is invalid"], request: "1d6c44b1-0df2-4b47-a8c0-d8d8017aa9f7" }, State { next_error: None, backtrace: None }))
Sending message #10 and waiting 3400 ms
Response: Ok(None)
Sending message #11 and waiting 3500 ms
Response: Err(Error(PushoverError { status: 0, errors: ["application token is invalid"], request: "2f5f605d-94af-49e3-830f-0c8ee9da18bd" }, State { next_error: None, backtrace: None }))
Sending message #12 and waiting 3600 ms
Response: Ok(None)
Sending message #13 and waiting 3700 ms
Response: Err(Error(PushoverError { status: 0, errors: ["application token is invalid"], request: "d3d9ad20-11a9-4a13-bec6-3b2ad01dc0cd" }, State { next_error: None, backtrace: None }))
Sending message #14 and waiting 3800 ms
Response: Ok(None)
Sending message #15 and waiting 3900 ms
Response: Err(Error(PushoverError { status: 0, errors: ["application token is invalid"], request: "6eb8ed72-e136-4d81-aeb7-813773b8be72" }, State { next_error: None, backtrace: None }))
Sending message #16 and waiting 4000 ms
Response: Ok(None)
Sending message #17 and waiting 4100 ms
Response: Err(Error(PushoverError { status: 0, errors: ["application token is invalid"], request: "9e7c9b60-d747-4ddf-8d27-d01b3f0959b3" }, State { next_error: None, backtrace: None }))
Sending message #18 and waiting 4200 ms
Response: Ok(None)
Sending message #19 and waiting 4300 ms
Response: Err(Error(PushoverError { status: 0, errors: ["application token is invalid"], request: "56e8d7b6-9f17-4b4e-bae5-e825f989c962" }, State { next_error: None, backtrace: None }))
Sending message #20 and waiting 4400 ms
Response: Ok(None)
All good.


Output without timeout:

Sending message #1 and waiting 2500 ms
Response: Err(Error(PushoverError { status: 0, errors: ["application token is invalid"], request: "e92d0939-c44c-4a59-8cd6-2cb917c03984" }, State { next_error: None, backtrace: None }))
Sending message #2 and waiting 2600 ms
Response: Err(Error(PushoverError { status: 0, errors: ["application token is invalid"], request: "33245236-c242-4ab9-a0d1-7f9006f3eaa3" }, State { next_error: None, backtrace: None }))
Sending message #3 and waiting 2700 ms
Response: Err(Error(PushoverError { status: 0, errors: ["application token is invalid"], request: "778d379c-f323-4d9a-92b4-8b42bf23890c" }, State { next_error: None, backtrace: None }))
Sending message #4 and waiting 2800 ms
Response: Err(Error(PushoverError { status: 0, errors: ["application token is invalid"], request: "bfbff7e7-2045-468b-9857-43746b97d0cb" }, State { next_error: None, backtrace: None }))
Sending message #5 and waiting 2900 ms
Response: Err(Error(PushoverError { status: 0, errors: ["application token is invalid"], request: "a458ed28-b794-43eb-8e01-6931d0e0004b" }, State { next_error: None, backtrace: None }))
Sending message #6 and waiting 3000 ms
Response: Err(Error(Hyper(Incomplete), State { next_error: None, backtrace: None }))
Sending message #7 and waiting 3100 ms
Response: Err(Error(PushoverError { status: 0, errors: ["application token is invalid"], request: "f294c099-8ccb-4352-83fb-9db01593a338" }, State { next_error: None, backtrace: None }))
Sending message #8 and waiting 3200 ms

Then it is waiting forever.
*/
