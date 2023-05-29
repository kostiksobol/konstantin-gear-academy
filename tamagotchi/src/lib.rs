#![no_std]
use gstd::{msg, exec, prelude::*, ActorId};
use tamagotchi_io::{TmgAction, TmgEvent};

static mut TAMAGOTCHI: Option<Tamagotchi> = None;


#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Tamagotchi {
    pub name: String,
    pub date_of_birth: u64,
}

impl Tamagotchi{
    fn name(&self){
        msg::reply(TmgEvent::Name(self.name.clone()), 0).expect("Error in reply name");

    }
    fn age(&self){
        msg::reply(TmgEvent::Age(self.date_of_birth), 0).expect("Error in reply age");
    }
}

#[no_mangle]
unsafe extern "C" fn init() {
    let init_name: String = msg::load().expect("Unable to decode InitHello");
    let tamagotchi = Tamagotchi{
        name: init_name,
        date_of_birth: exec::block_timestamp(),
    };
    TAMAGOTCHI = Some(tamagotchi);
    msg::reply(String::from("Tamagotchi created"), 0).expect("Error in reply init");
}

#[no_mangle]
unsafe extern "C" fn handle() {
    let action: TmgAction = msg::load().expect("Unable to decode TmgAction");
    let tamagotchi = TAMAGOTCHI.get_or_insert(Default::default());
    match action {
        TmgAction::Name => tamagotchi.name(),
        TmgAction::Age => tamagotchi.age(),
    }
}

#[no_mangle]
extern "C" fn state() {
    let tamagotchi = unsafe { TAMAGOTCHI.get_or_insert(Default::default()) };
    msg::reply(&tamagotchi, 0).expect("Failed to share state");
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Failed to share metahash");
}