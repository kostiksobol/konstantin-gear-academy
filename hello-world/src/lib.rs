#![no_std]
use gstd::{msg, prelude::*, ActorId};
use hello_world_io::{HelloAction, HelloEvent, InitHello, HelloState};

static mut HELLO: Option<Hello> = None;

#[derive(Default, Clone, Encode, Decode, TypeInfo)]
pub struct Hello {
    pub greeting: String,
}

impl Hello {
    fn send_hello_reply(&self) {
        msg::reply(&self.greeting, 0).expect("Error in reply SendHelloReply");
        // msg::reply(HelloEvent::HelloReplySended, 0).expect("Error in reply HelloEvent::HelloReplySended");
    }
    fn send_hello_to(&self, address: ActorId) {
        msg::send(address, &self.greeting, 0).expect("Error in sending SendHelloTo");
        msg::reply(HelloEvent::HelloToSended, 0).expect("Error in reply HelloEvent::HelloToSended");
    }
}

#[no_mangle]
unsafe extern "C" fn init() {
    let init_config: InitHello = msg::load().expect("Unable to decode InitHello");
    let hello = Hello {
        greeting: init_config.greeting,
    };
    HELLO = Some(hello);
}

#[no_mangle]
unsafe extern "C" fn handle() {
    let action: HelloAction = msg::load().expect("Unable to decode HelloAction");
    let hello = HELLO.get_or_insert(Default::default());
    match action {
        HelloAction::SendHelloReply => hello.send_hello_reply(),
        HelloAction::SendHelloTo(address) => hello.send_hello_to(address),
    }
}

#[no_mangle]
extern "C" fn state() {
    let hello = unsafe { HELLO.get_or_insert(Default::default()) };
    let hello_state = HelloState {greeting: hello.greeting.clone()};
    msg::reply(hello_state, 0).expect("Failed to share state"); 
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Failed to share metahash");
}
