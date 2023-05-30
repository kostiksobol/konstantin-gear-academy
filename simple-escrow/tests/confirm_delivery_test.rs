
pub mod utilts;
use utilts::*;

#[test]
fn good_confirm_delivery(){
    let sys = init_system();
    let escrow_program = init_escrow(&sys);

    sys.mint_to(BUYER, PRICE);

    check::deposit(&escrow_program, BUYER, PRICE);

    check::confirm_delivery(&escrow_program, BUYER);
}

#[test]
fn not_buyer_confirm(){
    let sys = init_system();
    let escrow_program = init_escrow(&sys);

    sys.mint_to(BUYER, PRICE);

    check::deposit(&escrow_program, BUYER, PRICE);

    fail::confirm_delivery(&escrow_program, FOREIGH_USER);
}

#[test]
fn double_confirm(){
    let sys = init_system();
    let escrow_program = init_escrow(&sys);

    sys.mint_to(BUYER, PRICE);

    check::deposit(&escrow_program, BUYER, PRICE);

    check::confirm_delivery(&escrow_program, BUYER);

    fail::confirm_delivery(&escrow_program, BUYER);
}

#[test]
fn confirm_before_deposit(){
    let sys = init_system();
    let escrow_program = init_escrow(&sys);

    sys.mint_to(BUYER, PRICE);

    fail::confirm_delivery(&escrow_program, BUYER);
}

#[test]
fn deposit_after_confirm(){
    let sys = init_system();
    let escrow_program = init_escrow(&sys);

    sys.mint_to(BUYER, 2 * PRICE);

    check::deposit(&escrow_program, BUYER, PRICE);

    check::confirm_delivery(&escrow_program, BUYER);

    fail::deposit(&escrow_program, BUYER, PRICE);
}