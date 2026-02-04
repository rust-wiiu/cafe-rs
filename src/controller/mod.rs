//! Controller

use bitflags::bitflags;

pub trait Controller {
    type Status;

    fn poll(&self) -> Self::Status;
}

pub struct Gamepad;

pub struct Wiimote;
pub struct WiiMotionPlus;
pub struct DisplayRemoteController;
pub struct ProController;
pub struct WiimoteClassic;

// KPAD
// VPAD

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct Button : u32 {
        const A = 1 << 0;
        const B = 1 << 1;
        const X = 1 << 2;
        const Y = 1 << 3;
        const Left = 1 << 4;
        const Right = 1 << 5;
        const Up = 1 << 6;
        const Down = 1 << 7;
        const L = 1 << 8;
        const R = 1 << 9;
        const ZL = 1 << 10;
        const ZR = 1 << 11;
        const Plus = 1 << 12;
        const Minus = 1 << 13;
        const Home = 1 << 14;
        const Sync = 1 << 15;
        const RStick = 1 << 16;
        const LStick = 1 << 17;
        const RStickLeft = 1 << 18;
        const RStickRight = 1 << 19;
        const RStickUp = 1 << 20;
        const RStickDown = 1 << 21;
        const LStickLeft = 1 << 22;
        const LStickRight = 1 << 23;
        const LStickUp = 1 << 24;
        const LStickDown = 1 << 25;
        const One = 1 << 26;
        const Two = 1 << 27;
        const Z = 1 << 28;
        const C = 1 << 29;
    }
}

#[derive(Debug, Default, Clone)]
pub struct Joystick {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Default, Clone)]
pub struct State {
    pub hold: Button,
    pub trigger: Button,
    pub release: Button,
    pub left_stick: Option<Joystick>,
    pub right_stick: Option<Joystick>,
}
