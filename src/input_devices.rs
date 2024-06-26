//! Input device component interfaces for devices such as `Button`
use rppal::gpio::{self, Gpio, InputPin, Level, Trigger};
use std::time::Duration;

/// Represents a generic GPIO input device.
#[derive(Debug)]
pub struct InputDevice {
    pin: InputPin,
    active_state: bool,
    inactive_state: bool,
}

impl InputDevice {
    /// Returns an InputDevice with the pin number given with the pin pulled to low by default
    /// `is_active` property is adjusted accordingly so that
    /// ``True`` still means active regardless of the :attr:`pull_up` setting
    /// # Arguments
    ///
    /// * `pin` - The GPIO pin which the device is attached to
    ///
    pub fn new(pin: u8) -> InputDevice {
        match Gpio::new() {
            Err(e) => panic!("{:?}", e),
            Ok(gpio) => match gpio.get(pin) {
                Err(e) => panic!("{:?}", e),
                Ok(pin) => InputDevice {
                    pin: pin.into_input_pulldown(),
                    active_state: true,
                    inactive_state: false,
                },
            },
        }
    }
    /// Returns an InputDevice with the pin number given with the pin pulled high with an internal resistor by default
    /// `is_active` property is adjusted accordingly so that
    /// ``True`` still means active regardless of the :attr:`pull_up` setting
    /// # Arguments
    ///
    /// * `pin` - The GPIO pin which the device is attached to
    ///
    pub fn new_with_pullup(pin: u8) -> InputDevice {
        match Gpio::new() {
            Err(e) => panic!("{:?}", e),
            Ok(gpio) => match gpio.get(pin) {
                Err(e) => panic!("{:?}", e),
                Ok(pin) => InputDevice {
                    pin: pin.into_input_pullup(),
                    active_state: false,
                    inactive_state: true,
                },
            },
        }
    }

    impl_device!();
    impl_gpio_device!();
    impl_io_device!();
}

macro_rules! impl_events_mixin {
    () => {
        /// Pause the program until the device is activated, or the timeout is reached.
        fn wait_for(&mut self, timeout: Option<f32>, active: bool) {
            let trigger = match active {
                true => Trigger::RisingEdge,
                false => Trigger::FallingEdge,
            };
            let timeout = timeout.map(|seconds| Duration::from_millis((seconds * 1000.0) as u64));
            self.pin.set_interrupt(trigger).unwrap();
            loop {
                match self.pin.poll_interrupt(true, timeout) {
                    Ok(_) => break,
                    Err(_) => continue,
                };
            }
        }
    };
}

/// Represents a generic input device with typical on/off behaviour.
/// Adds machinery to fire the active and inactive events for devices
/// that operate in a typical digital manner: straight forward on / off
/// states with (reasonably) clean transitions between the two.
#[derive(Debug)]
pub struct DigitalInputDevice {
    pin: InputPin,
    active_state: bool,
    inactive_state: bool,
}

impl DigitalInputDevice {
    /// Returns a DigitalInputDevice with the pin number given with the pin pulled to low by default
    /// `is_active` property is adjusted accordingly so that
    /// ``True`` still means active regardless of the :attr:`pull_up` setting
    /// # Arguments
    ///
    /// * `pin` - The GPIO pin which the device is attached to
    /// # Note: BCM pins 2 and 3 are i2c SDA and SCL respectively and include a fixed, 1.8 kohms pull-up to 3.3v
    /// These pins are not suitable for use where no pullup resistor is required
    /// Source: https://pinout.xyz/pinout/pin5_gpio3
    pub fn new(pin: u8) -> DigitalInputDevice {
        match Gpio::new() {
            Err(e) => panic!("{:?}", e),
            Ok(gpio) => match gpio.get(pin) {
                Err(e) => panic!("{:?}", e),
                Ok(pin) => DigitalInputDevice {
                    pin: pin.into_input_pulldown(),
                    active_state: true,
                    inactive_state: false,
                },
            },
        }
    }
    /// Returns a DigitalInputDevice with the pin number given with the pin pulled high with an internal resistor by default
    /// `is_active` property is adjusted accordingly so that
    /// ``True`` still means active regardless of the :attr:`pull_up` setting
    /// # Arguments
    ///
    /// * `pin` - The GPIO pin which the device is attached to
    ///
    pub fn new_with_pullup(pin: u8) -> DigitalInputDevice {
        match Gpio::new() {
            Err(e) => panic!("{:?}", e),
            Ok(gpio) => match gpio.get(pin) {
                Err(e) => panic!("{:?}", e),
                Ok(pin) => DigitalInputDevice {
                    pin: pin.into_input_pullup(),
                    active_state: false,
                    inactive_state: true,
                },
            },
        }
    }

    impl_device!();
    impl_gpio_device!();
    impl_io_device!();
    impl_events_mixin!();

    /// Pause the program until the device is deactivated, or the timeout is reached.
    pub fn wait_for_inactive(&mut self, timeout: Option<f32>) {
        self.wait_for(timeout, false)
    }

    /// Pause the program until the device is activated, or the timeout is reached.
    pub fn wait_for_active(&mut self, timeout: Option<f32>) {
        self.wait_for(timeout, true)
    }
}

/// Represents a simple push button or switch.
/// Connect one side of the button to a ground pin, and the other to any GPIO pin. The GPIO pin will be pulled high by default.
/// Alternatively, connect one side of the button to the 3V3 pin, and the other to any GPIO pin,
/// and then create a Button instance with Button::new_with_pulldown
pub struct Button {
    pub(crate) pin: InputPin,
    active_state: bool,
    inactive_state: bool,
}

impl Button {
    /// Returns a Button with the pin number given and the pin pulled high with an internal resistor by default
    /// * `pin` - The GPIO pin which the device is attached to
    pub fn new(pin: u8) -> Button {
        match Gpio::new() {
            Err(e) => panic!("{:?}", e),
            Ok(gpio) => match gpio.get(pin) {
                Err(e) => panic!("{:?}", e),
                Ok(pin) => Button {
                    pin: pin.into_input_pullup(),
                    active_state: false,
                    inactive_state: true,
                },
            },
        }
    }
    /// Returns a Button with the pin number given and the pin pulled down with an internal resistor by default
    /// * `pin` - The GPIO pin which the device is attached to
    pub fn new_with_pulldown(pin: u8) -> Button {
        match Gpio::new() {
            Err(e) => panic!("{:?}", e),
            Ok(gpio) => match gpio.get(pin) {
                Err(e) => panic!("{:?}", e),
                Ok(pin) => Button {
                    pin: pin.into_input_pulldown(),
                    active_state: true,
                    inactive_state: false,
                },
            },
        }
    }

    impl_device!();
    impl_gpio_device!();
    impl_io_device!();
    impl_events_mixin!();

    //// Pause the program until the device is deactivated, or the timeout is reached.
    /// * `timeout` - Number of seconds to wait before proceeding. If this is None, then wait indefinitely until the device is inactive.
    pub fn wait_for_release(&mut self, timeout: Option<f32>) {
        self.wait_for(timeout, false)
    }

    /// Pause the program until the device is activated, or the timeout is reached.
    /// * `timeout` - Number of seconds to wait before proceeding. If this is None, then wait indefinitely until the device is active.
    pub fn wait_for_press(&mut self, timeout: Option<f32>) {
        self.wait_for(timeout, true)
    }

    /// Invokes the passed closure everytime the button is pressed
    pub fn when_pressed<C>(&mut self, action: C) -> Result<(), gpio::Error>
    where
        C: FnMut(Level) + Send + 'static,
    {
        self.action_on(true, action)
    }

    /// Invokes the passed closure everytime the button is released
    pub fn when_released<C>(&mut self, action: C) -> Result<(), gpio::Error>
    where
        C: FnMut(Level) + Send + 'static,
    {
        self.action_on(false, action)
    }

    /// Adds an async interrupt for the corresponding trigger type to support `when_pressed`/`when_released`
    pub(crate) fn action_on<C>(&mut self, active: bool, action: C) -> Result<(), gpio::Error>
    where
        C: FnMut(Level) + Send + 'static,
    {
        let trigger = match active {
            true => Trigger::RisingEdge,
            false => Trigger::FallingEdge,
        };
        self.pin.set_async_interrupt(trigger, action)
    }

    /// Removes all previously configured async trigger(s) (E.g. `when_pressed`/`when_released`)
    pub fn clear_async_interrupt(&mut self) -> Result<(), gpio::Error> {
        self.pin.clear_async_interrupt()
    }
}
