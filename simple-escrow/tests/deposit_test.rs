
pub mod utilts;
use utilts::*;

#[test]
fn good_deposit(){
    let sys = init_system();
    let escrow_program = init_escrow(&sys);

    sys.mint_to(BUYER, PRICE);

    check::deposit(&escrow_program, BUYER, PRICE);
}

#[test]
fn not_equal_price_money_deposit(){
    let sys = init_system();
    let escrow_program = init_escrow(&sys);

    sys.mint_to(BUYER, PRICE);

    fail::deposit(&escrow_program, BUYER, PRICE - 100);
}

#[test]
fn not_buyer_deposit(){
    let sys = init_system();
    let escrow_program = init_escrow(&sys);

    sys.mint_to(FOREIGH_USER, PRICE);

    fail::deposit(&escrow_program, FOREIGH_USER, PRICE);
}

#[test]
fn double_deposit(){
    let sys = init_system();
    let escrow_program = init_escrow(&sys);

    sys.mint_to(BUYER, 2 * PRICE);

    check::deposit(&escrow_program, BUYER, PRICE);

    fail::deposit(&escrow_program, BUYER, PRICE); 
}