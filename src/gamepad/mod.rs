//! Access controller input from connected gamepads. The API is designed to only capture buttons and joystick inputs by default but can be extended with pointer, gyro, and touch controls when needed.
//!
//! Supported controllers:
//! * Display Remote Controller (DRC)
//! * Wiimote
//! * Wiimote + Nunchuk
//! * Wiimote + Classic
//! * Motion Plus
//! * Motion Plus + Nunchuk
//! * Motion Plus + Classic
//! * Wii U Pro Controller (URCC)
//!
//! Currently only buttons and joysticks are supported. The basics for pointer, gyro, and touch support is there but not implemented.
//!
//! The Wii U has in interal ring buffer which the controller data is written to. [Gamepads] acts as a secondary cache to simplify the management of input states.
//!
//! # Example
//!
//! ```rust
//! # use cafe_rs::prelude::*;
//! use cafe::gamepads::Gamepads;
//!
//! let mut gamepads = Gamepads::default();
//!
//! loop {
//!     gamepads.poll();
//!
//!     for (port, input) in &gamepads {
//!         // ...
//!     }
//! }
//! ```

use crate::prelude::*;

use crate::rrc::{Resource, Rrc};
use bitflags::bitflags;

static PADS: Rrc = Rrc::new(
    || unsafe {
        // sys::vpad::init();
        sys::padscore::kpad::init();
    },
    || unsafe {
        // sys::vpad::deinit();
        sys::padscore::kpad::deinit();
    },
);

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

macro_rules! map_bitfields {
    ($val:expr, $src_ty:ty, $dst_ty:ty, [$($field:ident),* $(,)?]) => {{
        let mut s = <$dst_ty>::empty();
        $(
            if $val.contains(<$src_ty>::$field) {
                s |= <$dst_ty>::$field;
            }
        )*
        s
    }};
}

impl From<sys::vpad::Button> for Button {
    fn from(value: sys::vpad::Button) -> Self {
        map_bitfields!(
            value,
            sys::vpad::Button,
            Self,
            [
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
            ]
        )
    }
}

impl From<sys::padscore::wpad::Button> for Button {
    fn from(value: sys::padscore::wpad::Button) -> Self {
        map_bitfields!(
            value,
            sys::padscore::wpad::Button,
            Self,
            [
                Left, Right, Down, Up, Plus, Two, One, B, A, Minus, Z, C, Home
            ]
        )
    }
}

impl From<sys::padscore::wpad::NunchukButton> for Button {
    fn from(value: sys::padscore::wpad::NunchukButton) -> Self {
        map_bitfields!(
            value,
            sys::padscore::wpad::NunchukButton,
            Self,
            [LStickLeft, LStickRight, LStickDown, LStickUp, Z, C]
        )
    }
}

impl From<sys::padscore::wpad::ClassicButton> for Button {
    fn from(value: sys::padscore::wpad::ClassicButton) -> Self {
        map_bitfields!(
            value,
            sys::padscore::wpad::ClassicButton,
            Self,
            [
                Up,
                Left,
                ZR,
                X,
                A,
                Y,
                B,
                ZL,
                R,
                Plus,
                Home,
                Minus,
                L,
                Down,
                Right,
                LStickLeft,
                LStickRight,
                LStickDown,
                LStickUp,
                RStickLeft,
                RStickRight,
                RStickDown,
                RStickUp
            ]
        )
    }
}

impl From<sys::padscore::wpad::URCCButton> for Button {
    fn from(value: sys::padscore::wpad::URCCButton) -> Self {
        map_bitfields!(
            value,
            sys::padscore::wpad::URCCButton,
            Self,
            [
                Up,
                Left,
                ZR,
                X,
                A,
                Y,
                B,
                ZL,
                R,
                Plus,
                Home,
                Minus,
                L,
                Down,
                Right,
                R3,
                L3,
                LStickUp,
                LStickDown,
                LStickLeft,
                LStickRight,
                RStickUp,
                RStickDown,
                RStickLeft,
                RStickRight
            ]
        )
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
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

impl From<sys::padscore::kpad::Vec2> for Joystick {
    fn from(value: sys::padscore::kpad::Vec2) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

pub struct GamepadsConfig<P, G, T>(core::marker::PhantomData<(P, G, T)>);

impl<P, G, T> GamepadsConfig<P, G, T> {
    const fn new() -> Self {
        Self(core::marker::PhantomData)
    }

    /// Wiimotes are enabled by default.
    pub fn wiimote(self, enabled: bool) -> Self {
        unsafe { sys::padscore::wpad::enable_wiimote(enabled) }
        self
    }

    /// URCC (Wii U Pro Controller) are disabled by default.
    pub fn urcc(self, enabled: bool) -> Self {
        unsafe { sys::padscore::wpad::enable_urcc(enabled) }
        self
    }

    /// WBC (Wii U Balance Board) is disabled by default.
    pub fn wbc(self, enabled: bool) -> Self {
        unsafe { sys::padscore::wpad::enable_wbc(enabled) }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

pub trait Pointer: Debug + Clone + PartialEq {
    fn from_vpad(status: &sys::vpad::Status) -> Self;
    fn from_kpad(status: &sys::padscore::kpad::Status) -> Self;
}

pub trait Gyro: Debug + Clone + PartialEq {
    fn from_vpad(status: &sys::vpad::Status) -> Self;
    fn from_kpad(status: &sys::padscore::kpad::Status) -> Self;
}

pub trait Touch: Debug + Clone + PartialEq {
    fn from_vpad(status: &sys::vpad::Status) -> Self;
    fn from_kpad(status: &sys::padscore::kpad::Status) -> Self;
}

impl Pointer for () {
    fn from_vpad(_status: &sys::vpad::Status) -> Self {}
    fn from_kpad(_status: &sys::padscore::kpad::Status) -> Self {}
}

impl Gyro for () {
    fn from_vpad(_status: &sys::vpad::Status) -> Self {}
    fn from_kpad(_status: &sys::padscore::kpad::Status) -> Self {}
}

impl Touch for () {
    fn from_vpad(_status: &sys::vpad::Status) -> Self {}
    fn from_kpad(_status: &sys::padscore::kpad::Status) -> Self {}
}

/// Holds the input data from the various gamepad types.
#[derive(Debug, Clone, PartialEq)]
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
    /// Convenience method for held buttons regardless of input device.
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

    /// Convenience method for triggered buttons regardless of input device.
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

    /// Convenience method for released buttons regardless of input device.
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

    /// Convenience method for left joystick. Returns `None` if controller does not have a (left) stick.
    ///
    /// Nunchuks stick is considers left stick.
    pub const fn left_stick(&self) -> Option<Joystick> {
        match self {
            Self::Nunchuk { stick, .. } => Some(*stick),
            Self::UURC { left_stick, .. } | Self::DRC { left_stick, .. } => Some(*left_stick),
            _ => None,
        }
    }

    /// Convenience method for left joystick. Returns `None` if controller does not have a right stick.
    pub const fn right_stick(&self) -> Option<Joystick> {
        match self {
            Self::UURC { right_stick, .. } | Self::DRC { right_stick, .. } => Some(*right_stick),
            _ => None,
        }
    }

    /// Checks if input is from a Wiimote.
    pub const fn is_wiimote(&self) -> bool {
        matches!(self, Self::Wiimote { .. })
    }

    /// Checks if input is from a Wiimote + Nunchuk.
    pub const fn is_nunchuk(&self) -> bool {
        matches!(self, Self::Nunchuk { .. })
    }

    /// Checks if input is from a Motion Plus.
    pub const fn is_wiimote_plus(&self) -> bool {
        matches!(self, Self::WiimotePlus { .. })
    }

    /// Checks if input is from a Motion Plus + Nunchuk.
    pub const fn is_nunchuk_plus(&self) -> bool {
        matches!(self, Self::NunchukPlus { .. })
    }

    /// Checks if input is from a Wii Classic Controller.
    pub const fn is_classic(&self) -> bool {
        matches!(self, Self::Classic { .. })
    }

    /// Checks if input is from a Wii U Pro Controller (URCC).
    pub const fn is_urcc(&self) -> bool {
        matches!(self, Self::UURC { .. })
    }

    /// Checks if input is from a Display Remote Controller.
    pub const fn is_drc(&self) -> bool {
        matches!(self, Self::DRC { .. })
    }

    /// Checks if a button is held.
    pub const fn is_held(&self, button: Button) -> bool {
        self.hold().contains(button)
    }

    /// Checks if a button is triggered.
    pub const fn is_triggered(&self, button: Button) -> bool {
        self.trigger().contains(button)
    }

    /// Checks if a button is released.
    pub const fn is_released(&self, button: Button) -> bool {
        self.release().contains(button)
    }
}

impl<P: Pointer, G: Gyro, T: Touch> From<sys::vpad::Status> for Input<P, G, T> {
    fn from(value: sys::vpad::Status) -> Self {
        let gyro = G::from_vpad(&value);
        let touch = T::from_vpad(&value);

        Input::DRC {
            hold: value.hold.into(),
            trigger: value.trigger.into(),
            release: value.release.into(),
            left_stick: value.left_stick.into(),
            right_stick: value.right_stick.into(),
            gyro,
            touch,
        }
    }
}

impl<P: Pointer, G: Gyro, T: Touch> From<sys::padscore::kpad::Status> for Input<P, G, T> {
    fn from(value: sys::padscore::kpad::Status) -> Self {
        let pointer = P::from_kpad(&value);
        let gyro = G::from_kpad(&value);

        use sys::padscore::kpad::ExtensionType as Ext;

        match value.extension_type {
            Ext::Core => Input::Wiimote {
                hold: value.hold.into(),
                trigger: value.trigger.into(),
                release: value.trigger.into(),
                pointer,
            },
            Ext::Nunchuk => Input::Nunchuk {
                hold: Button::from(value.hold) | unsafe { value.extension.nunchuk }.hold.into(),
                trigger: Button::from(value.trigger)
                    | unsafe { value.extension.nunchuk }.trigger.into(),
                release: Button::from(value.release)
                    | unsafe { value.extension.nunchuk }.release.into(),
                stick: unsafe { value.extension.nunchuk }.stick.into(),
                pointer,
            },
            Ext::MotionPlus => Input::WiimotePlus {
                hold: value.hold.into(),
                trigger: value.trigger.into(),
                release: value.release.into(),
                pointer,
                gyro,
            },
            Ext::MotionPlusNunchuk => Input::NunchukPlus {
                hold: value.hold.into(),
                trigger: value.trigger.into(),
                release: value.release.into(),
                stick: unsafe { value.extension.nunchuk }.stick.into(),
                pointer,
                gyro,
            },
            Ext::Classic | Ext::MotionPlusClassic => Input::Classic {
                hold: Button::from(value.hold) | unsafe { value.extension.classic }.hold.into(),
                trigger: Button::from(value.trigger)
                    | unsafe { value.extension.classic }.trigger.into(),
                release: Button::from(value.release)
                    | unsafe { value.extension.classic }.release.into(),
                left_stick: unsafe { value.extension.classic }.left_stick.into(),
                right_stick: unsafe { value.extension.classic }.right_stick.into(),
            },
            Ext::Urcc => Input::UURC {
                hold: Button::from(value.hold) | unsafe { value.extension.urcc }.hold.into(),
                trigger: Button::from(value.trigger)
                    | unsafe { value.extension.urcc }.trigger.into(),
                release: Button::from(value.release)
                    | unsafe { value.extension.urcc }.release.into(),
                left_stick: unsafe { value.extension.urcc }.left_stick.into(),
                right_stick: unsafe { value.extension.urcc }.right_stick.into(),
                gyro,
            },
            ext => {
                log::debug!("Input device not supported: {:?}", ext);
                // still output generic controller data
                Input::Wiimote {
                    hold: value.hold.into(),
                    trigger: value.trigger.into(),
                    release: value.trigger.into(),
                    pointer,
                }
            }
        }
    }
}

/// Manages the state of connected gamepads and provides methods to query their input. `IntoIterator` will yield the port and input of all currently connected gamepads. The state of the gamepads is only updated when [poll][Gamepads::poll] is called, allowing for more control over when input is captured. The systems expects [poll][Gamepads::poll] to be called on a regular basis (at best every frame).
///
/// # Example
///
/// ```
/// # use cafe_rs::prelude::*;
/// use cafe::gamepad::{Gamepads, Port};
///
/// let mut gamepads = Gamepads::default();
///
/// gamepads.poll();
///
/// for (port, input) in &gamepads {}
///
/// let input = gamepads.port(Port::DRC)?;
/// ```
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
    /// Use default gamepad types for the application.
    ///
    /// To modify which gamepads can be used use [config][Gamepad::config].
    ///
    /// # Example
    ///
    /// ```
    /// # use cafe_rs::prelude::*;
    /// use cafe::gamepad::Gamepads;
    ///
    /// let mut gamepads = Gamepads::default();
    /// ```
    pub fn default() -> Self {
        <Self as Default>::default()
    }
}

impl<P: Pointer, G: Gyro, T: Touch> Gamepads<P, G, T> {
    /// Enable and disabled certain gamepad types for the application.
    ///
    /// To use the default configuration use [default][Gamepads::default].
    ///
    /// # Example
    ///
    /// ```
    /// # use cafe_rs::prelude::*;
    /// use cafe::gamepad::Gamepads;
    ///
    /// let mut gamepads = Gamepads::<()>::config().urcc(true).init();
    /// ```
    pub fn config() -> GamepadsConfig<P, G, T> {
        GamepadsConfig::new()
    }

    /// Returns the input of a single port from cache.
    pub fn port(&self, port: Port) -> Option<&Input<P, G, T>> {
        match port {
            Port::P0 => self.inputs[0].1.as_ref(),
            Port::P1 => self.inputs[1].1.as_ref(),
            Port::P2 => self.inputs[2].1.as_ref(),
            Port::P3 => self.inputs[3].1.as_ref(),
            Port::P4 => self.inputs[4].1.as_ref(),
            Port::P5 => self.inputs[5].1.as_ref(),
            Port::P6 => self.inputs[6].1.as_ref(),
            Port::DRC => self.inputs[7].1.as_ref(),
        }
    }

    /// Updates the interal cache with current input states.
    ///
    /// # Example
    ///
    /// ```
    /// # use cafe_rs::prelude::*;
    /// use cafe::gamepad::Gamepads;
    ///
    /// let mut gamepads = Gamepads::default;
    ///
    /// gamepads.poll();
    ///
    /// for (port, input) in gamepads.poll() {}
    /// ```
    pub fn poll(&mut self) -> &Self {
        for (port, input) in &mut self.inputs {
            match port {
                Port::DRC => {
                    let mut status = std::mem::MaybeUninit::zeroed();
                    let mut error = sys::vpad::Error::Success;

                    let n = unsafe {
                        sys::vpad::poll(sys::vpad::Channel::C0, status.as_mut_ptr(), 1, &mut error)
                    };

                    if n == 1 && error == sys::vpad::Error::Success {
                        *input = Some(Input::from(unsafe { status.assume_init() }));
                    } else if error == sys::vpad::Error::NoSamples {
                    } else {
                        *input = None;
                    }
                }
                _ => {
                    let channel = match port {
                        Port::P0 => sys::padscore::wpad::Channel::C0,
                        Port::P1 => sys::padscore::wpad::Channel::C1,
                        Port::P2 => sys::padscore::wpad::Channel::C2,
                        Port::P3 => sys::padscore::wpad::Channel::C3,
                        Port::P4 => sys::padscore::wpad::Channel::C4,
                        Port::P5 => sys::padscore::wpad::Channel::C5,
                        Port::P6 => sys::padscore::wpad::Channel::C6,
                        Port::DRC => unreachable!(),
                    };

                    let mut status = std::mem::MaybeUninit::zeroed();
                    let mut error = sys::padscore::kpad::Error::Ok;

                    let n = unsafe {
                        sys::padscore::kpad::poll(channel, status.as_mut_ptr(), 1, &mut error)
                    };

                    if n == 1 && error == sys::padscore::kpad::Error::Ok {
                        *input = Some(Input::from(unsafe { status.assume_init() }));
                    } else if error == sys::padscore::kpad::Error::NoSamples {
                    } else {
                        *input = None;
                    }
                }
            }
        }

        self
    }

    /// Checks if a button is held on any connected gamepad.
    pub fn is_held(&self, button: Button) -> bool {
        self.inputs
            .iter()
            .any(|(_, input)| input.as_ref().map_or(false, |input| input.is_held(button)))
    }

    /// Checks if a button is triggered on any connected gamepad.
    pub fn is_triggered(&self, button: Button) -> bool {
        self.inputs.iter().any(|(_, input)| {
            input
                .as_ref()
                .map_or(false, |input| input.is_triggered(button))
        })
    }

    /// Checks if a button is released on any connected gamepad.
    pub fn is_released(&self, button: Button) -> bool {
        self.inputs.iter().any(|(_, input)| {
            input
                .as_ref()
                .map_or(false, |input| input.is_released(button))
        })
    }

    /// Checks if a button is held on a specific port.
    pub fn is_held_by(&self, button: Button, port: Port) -> bool {
        self.port(port)
            .as_ref()
            .map_or(false, |input| input.is_held(button))
    }

    /// Checks if a button is triggered on a specific port.
    pub fn is_triggered_by(&self, button: Button, port: Port) -> bool {
        self.port(port)
            .as_ref()
            .map_or(false, |input| input.is_triggered(button))
    }

    /// Checks if a button is released on a specific port.
    pub fn is_released_by(&self, button: Button, port: Port) -> bool {
        self.port(port)
            .as_ref()
            .map_or(false, |input| input.is_released(button))
    }
}

impl<'a, P, G, T> IntoIterator for &'a Gamepads<P, G, T> {
    type Item = (Port, &'a Input<P, G, T>);
    type IntoIter = core::iter::FilterMap<
        core::slice::Iter<'a, (Port, Option<Input<P, G, T>>)>,
        fn(&(Port, Option<Input<P, G, T>>)) -> Option<(Port, &Input<P, G, T>)>,
    >;

    /// Iterates over all connected gamepads.
    fn into_iter(self) -> Self::IntoIter {
        self.inputs
            .iter()
            .filter_map(|(port, input)| input.as_ref().map(|input| (*port, input)))
    }
}
