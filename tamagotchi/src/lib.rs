#![no_std]
use gstd::{msg, exec, prelude::*, ActorId};
use tamagotchi_io::{TmgAction, TmgEvent};

static mut TAMAGOTCHI: Option<Tamagotchi> = None;


const HUNGER_PER_BLOCK: u64 = 1;
const ENERGY_PER_BLOCK: u64 = 2;
const BOREDOM_PER_BLOCK: u64 = 2;
const FILL_PER_SLEEP: u64 = 1000;
const FILL_PER_FEED: u64 = 1000;
const FILL_PER_ENTERTAINMENT: u64 = 1000;


#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Tamagotchi {
   pub name: String,
   pub date_of_birth: u64,
   pub owner: ActorId,
   pub fed: u64,
   pub fed_block: u64,
   pub entertained: u64,
   pub entertained_block: u64,
   pub rested: u64,
   pub rested_block: u64,
}

impl Tamagotchi{
    fn name(&self){
        msg::reply(TmgEvent::Name(self.name.clone()), 0).expect("Error in reply name");

    }
    fn age(&self){
        msg::reply(TmgEvent::Age(self.date_of_birth), 0).expect("Error in reply age");
    }
    fn sleep(&mut self){
        let block_hight = exec::block_height() as u64;
        self.rested += FILL_PER_SLEEP - (block_hight - self.rested_block) * ENERGY_PER_BLOCK;
        self.rested_block = block_hight;
        msg::reply(TmgEvent::Slept, 0).expect("Error in reply slept");
    }
    fn feed(&mut self){
        let block_hight = exec::block_height() as u64;
        self.fed += FILL_PER_FEED - (block_hight - self.fed_block) * HUNGER_PER_BLOCK;
        self.fed_block = block_hight;
        msg::reply(TmgEvent::Fed, 0).expect("Error in reply fed");
    }
    fn play(&mut self){
        let block_hight = exec::block_height() as u64;
        self.entertained += FILL_PER_ENTERTAINMENT - (block_hight - self.entertained_block) * BOREDOM_PER_BLOCK;
        self.entertained_block = block_hight;
        msg::reply(TmgEvent::Entertained, 0).expect("Error in reply entertained");
    }
}

#[no_mangle]
unsafe extern "C" fn init() {
    let init_name: String = msg::load().expect("Unable to decode InitHello");

    let block_hight = exec::block_height() as u64;
    let tamagotchi = Tamagotchi{
        name: init_name,
        date_of_birth: exec::block_timestamp(),
        owner: msg::source(),
        fed: 1,
        fed_block: block_hight,
        entertained: 1,
        entertained_block: block_hight,
        rested: 1,
        rested_block: block_hight,
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
        TmgAction::Feed => tamagotchi.feed(),
        TmgAction::Sleep => tamagotchi.sleep(),
        TmgAction::Play => tamagotchi.play(),
    }
}

#[no_mangle]
extern "C" fn state() {
    let tamagotchi = unsafe {TAMAGOTCHI.get_or_insert(Default::default())};

    msg::reply(&tamagotchi, 0).expect("Failed to share state");
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Failed to share metahash");
}