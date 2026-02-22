//! Controller

use crate::prelude::*;

use crate::rrc::{Resource, Rrc};
use bitflags::bitflags;

static PADS: Rrc = Rrc::new(
    || {
        // sys::vpad::init();
        // sys::kpad::init();
    },
    || {
        // sys::vpad::deinit();
        // sys::kpad::deinit();
    },
);

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
        const R3 = 1 << 16;
        const L3 = 1 << 17;
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
        const TV = 1 << 30;
    }
}

impl From<sys::vpad::Button> for Button {
    fn from(value: sys::vpad::Button) -> Self {
        let mut s = Self::empty();

        macro_rules! convert {
            ($($btn:ident),*) => {
                $(
                    if value.contains(sys::vpad::Button::$btn) {
                        s |= Button::$btn;
                    }
                )*
            };
        }

        convert!(
            A,
            B,
            X,
            Y,
            Left,
            Right,
            Up,
            Down,
            ZL,
            ZR,
            L,
            R,
            Plus,
            Minus,
            Home,
            Sync,
            R3,
            L3,
            TV,
            RStickLeft,
            RStickRight,
            RStickUp,
            RStickDown,
            LStickLeft,
            LStickRight,
            LStickUp,
            LStickDown
        );

        s
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Joystick {
    pub x: f32,
    pub y: f32,
}

impl From<sys::vpad::Vec2> for Joystick {
    fn from(value: sys::vpad::Vec2) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

pub struct GamepadsConfig<P, G, T>(core::marker::PhantomData<(P, G, T)>);

impl<P, G, T> GamepadsConfig<P, G, T> {
    /// Wiimotes are enabled by default.
    pub fn wiimote(self, enabled: bool) -> Self {
        log::info!("wiimote: {enabled}");
        self
    }

    /// URCC (Wii U Pro Controller) are disabled by default.
    pub fn urcc(self, enabled: bool) -> Self {
        log::info!("urcc: {enabled}");
        self
    }

    /// WBC (Wii U Balance Board) is disabled by default.
    pub fn wbc(self, enabled: bool) -> Self {
        log::info!("wbc: {enabled}");
        self
    }

    pub fn init(self) -> Gamepads<P, G, T>
    where
        P: Pointer,
        G: Gyro,
        T: Touch,
    {
        <Gamepads<P, G, T> as Default>::default()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Port {
    P0,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    DRC,
}

pub trait Pointer: Debug + Clone {
    fn from_vpad(status: &sys::vpad::Status) -> Self;
    fn from_kpad(status: ()) -> Self;
}

pub trait Gyro: Debug + Clone {
    fn from_vpad(status: &sys::vpad::Status) -> Self;
    fn from_kpad(status: ()) -> Self;
}

pub trait Touch: Debug + Clone {
    fn from_vpad(status: &sys::vpad::Status) -> Self;
    fn from_kpad(status: ()) -> Self;
}

impl Pointer for () {
    fn from_vpad(_status: &sys::vpad::Status) -> Self {}
    fn from_kpad(_status: ()) -> Self {}
}

impl Gyro for () {
    fn from_vpad(_status: &sys::vpad::Status) -> Self {}
    fn from_kpad(_status: ()) -> Self {}
}

impl Touch for () {
    fn from_vpad(_status: &sys::vpad::Status) -> Self {}
    fn from_kpad(_status: ()) -> Self {}
}

#[derive(Debug, Clone)]
pub enum Input<Pointer = (), Gyro = (), Touch = ()> {
    Wiimote {
        hold: Button,
        trigger: Button,
        release: Button,
        pointer: Pointer,
    },
    Nunchuk {
        hold: Button,
        trigger: Button,
        release: Button,
        stick: Joystick,
        pointer: Pointer,
    },
    WiimotePlus {
        hold: Button,
        trigger: Button,
        release: Button,
        pointer: Pointer,
        gyro: Gyro,
    },
    NunchukPlus {
        hold: Button,
        trigger: Button,
        release: Button,
        stick: Joystick,
        pointer: Pointer,
        gyro: Gyro,
    },
    Classic {
        hold: Button,
        trigger: Button,
        release: Button,
        left_stick: Joystick,
        right_stick: Joystick,
    },
    UURC {
        hold: Button,
        trigger: Button,
        release: Button,
        left_stick: Joystick,
        right_stick: Joystick,
        gyro: Gyro,
    },
    DRC {
        hold: Button,
        trigger: Button,
        release: Button,
        left_stick: Joystick,
        right_stick: Joystick,
        gyro: Gyro,
        touch: Touch,
    },
}

impl<P, G, T> Input<P, G, T> {
    pub const fn hold(&self) -> Button {
        match self {
            Self::Wiimote { hold, .. }
            | Self::Nunchuk { hold, .. }
            | Self::WiimotePlus { hold, .. }
            | Self::NunchukPlus { hold, .. }
            | Self::Classic { hold, .. }
            | Self::UURC { hold, .. }
            | Self::DRC { hold, .. } => *hold,
        }
    }

    pub const fn trigger(&self) -> Button {
        match self {
            Self::Wiimote { trigger, .. }
            | Self::Nunchuk { trigger, .. }
            | Self::WiimotePlus { trigger, .. }
            | Self::NunchukPlus { trigger, .. }
            | Self::Classic { trigger, .. }
            | Self::UURC { trigger, .. }
            | Self::DRC { trigger, .. } => *trigger,
        }
    }

    pub const fn release(&self) -> Button {
        match self {
            Self::Wiimote { release, .. }
            | Self::Nunchuk { release, .. }
            | Self::WiimotePlus { release, .. }
            | Self::NunchukPlus { release, .. }
            | Self::Classic { release, .. }
            | Self::UURC { release, .. }
            | Self::DRC { release, .. } => *release,
        }
    }

    pub const fn left_stick(&self) -> Option<Joystick> {
        match self {
            Self::Nunchuk { stick, .. } => Some(*stick),
            Self::UURC { left_stick, .. } | Self::DRC { left_stick, .. } => Some(*left_stick),
            _ => None,
        }
    }

    pub const fn right_stick(&self) -> Option<Joystick> {
        match self {
            Self::UURC { right_stick, .. } | Self::DRC { right_stick, .. } => Some(*right_stick),
            _ => None,
        }
    }
}

impl<P: Pointer, G: Gyro, T: Touch> From<sys::vpad::Status> for Input<P, G, T> {
    fn from(value: sys::vpad::Status) -> Self {
        let gyro = G::from_vpad(&value);
        let touch = T::from_vpad(&value);

        Input::DRC {
            hold: Button::from(value.hold),
            trigger: Button::from(value.trigger),
            release: Button::from(value.release),
            left_stick: Joystick::from(value.left_stick),
            right_stick: Joystick::from(value.right_stick),
            gyro,
            touch,
        }
    }
}

pub struct Gamepads<Pointer = (), Gyro = (), Touch = ()> {
    inputs: [(Port, Option<Input<Pointer, Gyro, Touch>>); 8],
    _resource: Resource,
}

impl<P: Pointer, G: Gyro, T: Touch> Default for Gamepads<P, G, T> {
    fn default() -> Self {
        Self {
            inputs: [
                (Port::P0, None),
                (Port::P1, None),
                (Port::P2, None),
                (Port::P3, None),
                (Port::P4, None),
                (Port::P5, None),
                (Port::P6, None),
                (Port::DRC, None),
            ],
            _resource: PADS.acquire(),
        }
    }
}

impl Gamepads {
    pub fn default() -> Self {
        <Self as Default>::default()
    }
}

impl<P: Pointer, G: Gyro, T: Touch> Gamepads<P, G, T> {
    pub fn port(&self, port: Port) -> &Option<Input<P, G, T>> {
        match port {
            Port::P0 => &self.inputs[0].1,
            Port::P1 => &self.inputs[1].1,
            Port::P2 => &self.inputs[2].1,
            Port::P3 => &self.inputs[3].1,
            Port::P4 => &self.inputs[4].1,
            Port::P5 => &self.inputs[5].1,
            Port::P6 => &self.inputs[6].1,
            Port::DRC => &self.inputs[7].1,
        }
    }

    pub fn config() -> GamepadsConfig<P, G, T> {
        GamepadsConfig(core::marker::PhantomData)
    }

    pub fn poll(&mut self) -> &Self {
        for (port, input) in &mut self.inputs {
            match port {
                Port::DRC => {
                    let mut status = std::mem::MaybeUninit::zeroed();
                    let mut error = sys::vpad::Error::Success;

                    let n = unsafe {
                        sys::vpad::poll(sys::vpad::Channel::C0, status.as_mut_ptr(), 1, &mut error)
                    };

                    if n != 1 || error != sys::vpad::Error::Success {
                        *input = None;
                    } else {
                        *input = Some(Input::from(unsafe { status.assume_init() }));
                    }
                }
                _ => {
                    // todo!()
                }
            }
        }

        self
    }
}

impl<'a, P, G, T> IntoIterator for &'a Gamepads<P, G, T> {
    type Item = (Port, &'a Input<P, G, T>);
    type IntoIter = core::iter::FilterMap<
        core::slice::Iter<'a, (Port, Option<Input<P, G, T>>)>,
        fn(&(Port, Option<Input<P, G, T>>)) -> Option<(Port, &Input<P, G, T>)>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.inputs
            .iter()
            .filter_map(|(port, input)| input.as_ref().map(|input| (*port, input)))
    }
}
