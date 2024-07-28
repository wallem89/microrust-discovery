#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
// use cortex_m::interrupt::free;
use panic_halt as _;
use microbit::{
    board::Board,
    // board::Buttons,
    // pac::{self, GPIOTE},
    display::blocking::Display,
    hal::Timer,
};
// use microbit::pac::interrupt;
use embedded_hal as _;
use embedded_hal::delay::DelayNs;

// pub crate Turn {

// };

// /// Initialise the buttons and enable interrupts.
// pub(crate) fn init_buttons(board_gpiote: GPIOTE, board_buttons: Buttons) {
//     let gpiote = Gpiote::new(board_gpiote);

//     let channel0 = gpiote.channel0();
//     channel0
//         .input_pin(&board_buttons.button_a.degrade())
//         .hi_to_lo()
//         .enable_interrupt();
//     channel0.reset_events();

//     let channel1 = gpiote.channel1();
//     channel1
//         .input_pin(&board_buttons.button_b.degrade())
//         .hi_to_lo()
//         .enable_interrupt();
//     channel1.reset_events();

//     free(move |cs| {
//         *GPIO.borrow(cs).borrow_mut() = Some(gpiote);

//         unsafe {
//             pac::NVIC::unmask(pac::Interrupt::GPIOTE);
//         }
//         pac::NVIC::unpend(pac::Interrupt::GPIOTE);
//     });
// }

#[entry]
fn main() -> ! {
    let _y;
    let x = 42;
    _y = x;
    
    // take the board
    let board = Board::take().unwrap();
    // make a timer
    let mut timer = Timer::new(board.TIMER0);
    // create the Display
    let mut display = Display::new(board.display_pins);
    // Init buttons
    // init_buttons(board.GPIOTE, board.buttons);
    // and light up some LEDs
    let _heart = [
        [0, 1, 0, 1, 0],
        [1, 0, 1, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 0, 1, 0],
        [0, 0, 1, 0, 0],
    ];
    let open = [
        [1, 1, 1, 0, 0],
        [1, 0, 1, 0, 0],
        [1, 0, 1, 0, 0],
        [1, 0, 1, 0, 0],
        [1, 1, 1, 0, 0],
    ];
    let close = [
        [1, 1, 1, 0, 0],
        [1, 0, 0, 0, 0],
        [1, 0, 0, 0, 0],
        [1, 0, 0, 0, 0],
        [1, 1, 1, 0, 0],
    ];
    let error: [[u8; 5]; 5] = [
        [1, 1, 1, 0, 0],
        [1, 0, 0, 0, 0],
        [1, 1, 1, 0, 0],
        [1, 0, 0, 0, 0],
        [1, 1, 1, 0, 0],
    ];
    // infinite loop; just so we don't leave this stack frame
    loop {
        display.show(&mut timer, open, 1000);
        timer.delay_ms(1_000);
        display.show(&mut timer, close, 1000);
        timer.delay_ms(1_000);
        display.show(&mut timer, error, 1000);
        timer.delay_ms(1_000);
    }
}

// #[interrupt]
// fn GPIOTE() {
//     free(|cs| {
//         if let Some(gpiote) = GPIO.borrow(cs).borrow().as_ref() {
//             let a_pressed = gpiote.channel0().is_event_triggered();
//             let b_pressed = gpiote.channel1().is_event_triggered();

//             let turn = match (a_pressed, b_pressed) {
//                 (true, false) => Turn::Left,
//                 (false, true) => Turn::Right,
//                 _ => Turn::None
//             };

//             gpiote.channel0().reset_events();
//             gpiote.channel1().reset_events();

//             *TURN.borrow(cs).borrow_mut() = turn;
//         }
//     });
// }