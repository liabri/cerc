#![no_std]
#![no_main]

use panic_halt as _;
use rp_pico::{entry};
use rp_pico::hal::{pwm::{Slices, Channel}, pac, sio::Sio, gpio::{PinId, PullDown, FunctionPwm, FunctionSio, SioInput, SioOutput, PullUp, Pin, bank0::{Gpio1, Gpio5, Gpio9, Gpio11, Gpio12,  Gpio13, Gpio14, Gpio15, Gpio16, Gpio17, Gpio18, Gpio19, Gpio20, Gpio21, Gpio22, Gpio26, Gpio27, Gpio28}}};
use embedded_hal::digital::InputPin; // Provides the .is_low() method
use micromath::F32Ext;
use rp2040_hal::gpio::PinState;

#[entry]
fn main() -> ! {
    let mut pico = Pico::new();
    let mut state = State {
        current_stops: 0.0,
        stop_interval: pico.get_stop_interval_state(),
        mode: pico.get_mode_state(),
        safelight: pico.get_safelight_state(),
        focus: pico.get_focus_state(),
        running: false
    };

    loop {
        if pico.get_focus_state() && !matches!(state.mode, Mode::Burn(_)) { state.focus = true; }
        if pico.get_safelight_state() { state.safelight = true; }
        if pico.get_burn_button_state() { state.mode=Mode::Burn(0.0); }
        if pico.get_mode_state()==Mode::Test && !matches!(state.mode, Mode::Burn(_)) { state.mode=Mode::Test }
        if pico.get_mode_state()==Mode::Print && !matches!(state.mode, Mode::Burn(_)) { state.mode=Mode::Print }

        state.stop_interval = pico.get_stop_interval_state();
        state.current_stops += pico.get_stop_increment_state(&state.stop_interval);
    }
}

















struct Pico {
    // input
    btn_burn: Pin<Gpio16, FunctionSio<SioInput>, PullUp>,
    btn_print: Pin<Gpio17, FunctionSio<SioInput>, PullUp>,
    sw_mode: Pin<Gpio11, FunctionSio<SioInput>, PullUp>,
    sw_safelight: Pin<Gpio12, FunctionSio<SioInput>, PullUp>,
    sw_focus: Pin<Gpio13, FunctionSio<SioInput>, PullUp>,
    sw_int_half: Pin<Gpio18, FunctionSio<SioInput>, PullUp>,
    sw_int_third: Pin<Gpio19, FunctionSio<SioInput>, PullUp>,
    sw_int_quarter: Pin<Gpio20, FunctionSio<SioInput>, PullUp>,
    sw_int_sixth: Pin<Gpio21, FunctionSio<SioInput>, PullUp>,
    enc_inc_clk: Pin<Gpio22, FunctionSio<SioInput>, PullUp>,
    enc_inc_dt: Pin<Gpio26, FunctionSio<SioInput>, PullUp>,
    enc_inc_sw: Pin<Gpio27, FunctionSio<SioInput>, PullUp>,
    enc_last_clk: bool,

    // output
    enl_pulse: Pin<Gpio1, FunctionSio<SioOutput>, PullDown>,
    enl_hold: Pin<Gpio5, FunctionSio<SioOutput>, PullDown>,
    sfl_ctrl: Pin<Gpio9, FunctionSio<SioOutput>, PullDown>,
    buzzer_pwm: Buzzer,
    // display_sda: Pin<Gpio14, FunctionSio<SioInput>, PullUp>,
    // display_scl: Pin<Gpio15, FunctionSio<SioInput>, PullUp>
}

impl Pico {
    fn new() -> Pico {
        // configure GPIO pins to work w an internal pull-up resistor. LOW = closed
        let mut pac = pac::Peripherals::take().unwrap();
        let sio = Sio::new(pac.SIO);
        let pins = rp_pico::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        Pico {
            btn_burn: pins.gpio16.into_pull_up_input(),
            btn_print: pins.gpio17.into_pull_up_input(),
            sw_mode: pins.gpio11.into_pull_up_input(),
            sw_safelight: pins.gpio12.into_pull_up_input(),
            sw_focus: pins.gpio13.into_pull_up_input(),

            sw_int_half: pins.gpio18.into_pull_up_input(),
            sw_int_third: pins.gpio19.into_pull_up_input(),
            sw_int_quarter: pins.gpio20.into_pull_up_input(),
            sw_int_sixth: pins.gpio21.into_pull_up_input(),

            enc_inc_clk: pins.gpio22.into_pull_up_input(),
            enc_inc_dt: pins.gpio26.into_pull_up_input(),
            enc_inc_sw: pins.gpio27.into_pull_up_input(),
            enc_last_clk: false,

            enl_pulse: pins.gpio1.into_push_pull_output_in_state(PinState::Low),
            enl_hold: pins.gpio5.into_push_pull_output_in_state(PinState::Low),
            sfl_ctrl: pins.gpio9.into_push_pull_output_in_state(PinState::Low),
            buzzer_pwm: Buzzer::new(pins.gpio28.into_function()),
        }
    }

    fn get_stop_interval_state(&mut self) -> StopInterval {
        if self.sw_int_half.is_low().unwrap_or(false) { return StopInterval::Half; }
        else if self.sw_int_third.is_low().unwrap_or(false) { return StopInterval::Third; }
        else if self.sw_int_quarter.is_low().unwrap_or(false) { return StopInterval::Quarter; }
        else if self.sw_int_sixth.is_low().unwrap_or(false) { return StopInterval::Sixth; }
        else { panic!("something went wrong reading the interval knob")}
    }

    fn get_stop_increment_state(&mut self, interval: &StopInterval) -> f32 {
        let clk = self.enc_inc_clk.is_low().unwrap_or(false);
        let dt  = self.enc_inc_dt.is_low().unwrap_or(false);
        let mut delta = 0.0;

        // rising edge on CLK = new detent
        if !self.enc_last_clk && clk {
            delta = if dt { -interval.as_f32() } else { interval.as_f32() };
        }

        self.enc_last_clk = clk;
        delta
    }

    fn get_mode_state(&mut self) -> Mode {
        match self.sw_mode.is_low().unwrap_or(false) {
            true => Mode::Print,
            false => Mode::Test,
        }
    }

    fn get_focus_state(&mut self) -> bool {
        self.sw_focus.is_low().unwrap_or(false)
    }

    fn get_safelight_state(&mut self) -> bool {
        self.sw_safelight.is_low().unwrap_or(false)
    }

    fn get_burn_button_state(&mut self) -> bool {
        self.btn_burn.is_low().unwrap_or(false)
    }
}


















struct Buzzer<P: PinId> {
    pin: Pin<P, FunctionPwm, PullDown>,
    slice: Slices,
    channel: Channel,
}

impl<P: PinId> Buzzer<P> {
    fn new(pac_pwm: pac::PWM, resets: &mut pac::RESETS) -> Self {
        let mut slices = Slices::new(pac_pwm, resets);
        let mut slice0 = slices.pwm0;  // GPIO28 → PWM0 channel A
        slice0.enable();
        slice0.set_ph_correct();        // optional
        slice0.set_top(65535);          // default period
        let buzzer_pin: Pin<Gpio16, FunctionPwm, _> = pin.into_mode();

        Self {
            pin: buzzer_pin,
            slice: slices,
            channel: Channel::A,
        }
    }

    pub fn on(&mut self, duty: u16) {
        self.slice.pwm0.channel(self.channel).set_duty(duty);
    }

    pub fn off(&mut self) {
        self.slice.pwm0.channel(self.channel).set_duty(0);
    }
}























struct State {
    current_stops: f32,
    stop_interval: StopInterval,
    mode: Mode,
    safelight: bool,
    focus: bool,
    running: bool,
}

struct TestMode {
    interval: u32, // interval between exposures (if pause==false)
    pause: bool, // pause between exposures?
}

impl State {


    // calculates total seconds for a given stop value
    fn stops_to_ms(stops: f32) -> u32 {
        // 0.0 stops = 1.0 second = 1000ms
        // formula: 1000 * 2^stops
        let seconds = 2.0_f32.powf(stops);
        (seconds * 1000.0) as u32
    }

    // calculates burn time
    fn set_burn(&mut self) {
        self.burn_stops = self.burn_stops + self.knob_offset;
    }

    // convert remaining time to respective stop
    fn get_current_stops_display(&self, remaining_ms: u32, total_ms: u32) -> f32 {
        let elapsed_secs = (total_ms - remaining_ms) as f32 / 1000.0;
        elapsed_secs.log2()
    }

    // convert the decimal stop to 7-segment "digits"
    fn get_display_digits(&self) -> (u8, u8, u8) {
        // round to one decimal place for the 3-digit display
        let rounded = (self.current_stops * 10.0).round() / 10.0;

        let tens = (rounded / 10.0) as u8;
        let ones = (rounded as u8) % 10;
        let tenths = ((rounded * 10.0) as u8) % 10;

        (tens, ones, tenths)
    }
}

//once BURN button is pressed, the interval knob will just add stops (as per the knob_offset) to State.burn_stops. when print is clicked, it will count down from current_stops + burn_stops to current_stops. the display will exclusively count down from the burn_stops though, as ending on 4.7 is weirder than 0.0
fn burn() {

}



#[derive(PartialEq)]
enum Mode {
    Print,
    Test,
    Burn(f32),
}

























enum StopInterval {
    Half,
    Third,
    Quarter,
    Sixth
}

impl StopInterval {
    fn as_f32(&self) -> f32 {
        match self {
            StopInterval::Half => 0.5,
            StopInterval::Third => 0.3333,
            StopInterval::Quarter => 0.25,
            StopInterval::Sixth => 0.12
        }
    }
}





struct Display {}
impl Display {
    // generates bitmasks for a 3-digit HT16K33. returns [Digit1, Digit2, Digit3]
    fn stops_to_segments(&self, stops: f32) -> [u8; 3] {
        let mut segments = [0x00u8; 3];
        let mut val = stops;

        // handle negatives
        let is_negative = val < 0.0;
        if is_negative {
            val = val.abs();
        }

        // determine formatting based on value size, no need to do [X][Y.][Z] as limit is 9.9
        if is_negative {
            // pattern: [-][X.][Y] (e.g. -1.1)
            segments[0] = 0x40; // the minus sign (Segment G)
            let integer_part = val as u8;
            let fractional_part = ((val - integer_part as f32) * 10.0 + 0.5) as u8;

            segments[1] = self.digit_to_7segment(integer_part % 10) | 0x80; // digit + dot
            segments[2] = self.digit_to_7segment(fractional_part % 10);
        } else {
            // Pattern: [ ][X.][Y] (e.g. 4.2)
            let integer_part = val as u8;
            let fractional_part = ((val - integer_part as f32) * 10.0 + 0.5) as u8;

            segments[0] = 0x00; // blank leading digit
            segments[1] = self.digit_to_7segment(integer_part) | 0x80;
            segments[2] = self.digit_to_7segment(fractional_part);
        }

        segments
    }

    fn digit_to_7segment(&self, digit: u8) -> u8 {
        match digit {
            0 => 0x3F, 1 => 0x06, 2 => 0x5B, 3 => 0x4F,
            4 => 0x66, 5 => 0x6D, 6 => 0x7D, 7 => 0x07,
            8 => 0x7F, 9 => 0x6F,
            _ => 0x00,
        }
    }
}
