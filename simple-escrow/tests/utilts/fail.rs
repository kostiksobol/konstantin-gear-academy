use super::*;

pub fn deposit(escrow_program: &Program, buyer: u64, price: u128) {
    assert!(escrow_program
        .send_with_value(buyer, EscrowAction::Deposit, price)
        .main_failed());
}

pub fn confirm_delivery(escrow_program: &Program, buyer: u64) {
    assert!(escrow_program
        .send(buyer, EscrowAction::ConfirmDelivery)
        .main_failed());
}
