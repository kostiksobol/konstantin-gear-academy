pub use escrow_io::*;
use gstd::{prelude::*, ActorId};
pub use gtest::{Log, Program, System};

pub mod check;
pub mod fail;

pub const ESCROW_PROGRAM_ID: u64 = 1;
pub const BUYER: u64 = 2;
pub const SELLER: u64 = 3;
pub const FOREIGH_USER: u64 = 4;
pub const PRICE: u128 = 100_000;

pub fn init_system() -> System {
    let system = System::new();
    system.init_logger();

    system
}

pub fn init_escrow(sys: &System) -> Program {
    let escrow_program = Program::current_with_id(sys, ESCROW_PROGRAM_ID);
    let res = escrow_program.send(
        FOREIGH_USER,
        InitEscrow {
            seller: SELLER.into(),
            buyer: BUYER.into(),
            price: PRICE,
        },
    );
    assert!(res.log().is_empty());

    escrow_program
}
