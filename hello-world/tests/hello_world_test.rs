use gtest::{Log, Program, System};
use hello_world_io::{HelloAction, HelloEvent, InitHello};
use parity_scale_codec::Encode;

#[test]
fn hello_test(){
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    let res = program.send(2, InitHello {greeting: String::from("H_E_L_L_O W_O_R_L_D")});
    assert!(!res.main_failed());
    assert!(res.log().is_empty());

    let res = program.send(2, HelloAction::SendHelloReply);
    let log = Log::builder()
        .dest(2)
        .payload(String::from("H_E_L_L_O W_O_R_L_D"));
    assert!(res.contains(&log));

    let res = program.send(2, HelloAction::SendHelloTo(4.into()));
    let log = Log::builder()
        .dest(2)
        .payload(HelloEvent::HelloToSended);
    assert!(res.contains(&log));    
}