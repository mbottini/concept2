# Concept2 HID API Project

## Mike Bottini

# Description

The Concept2 rowing machine offers an interface through HID to get data and set
workout paramters. Typically, this is done through Bluetooth, but it's also
possible to interface with the device through a USB port on the back. The format
for interacting through HID is called CSAFE - the Communications Specification
for Fitness Equipment. In short, it's a format for the bytes that you send to
the device and interpreting the bytes that you get.

CSAFE commands are varied and involve things like the following:

* How many meters are left in the workout?
* How much power was in the last stroke?
* What's the serial number of the rowing machine?
* What is the user's current heart rate?
* End the current workout.
* Get the rowing machine's stored date and time.

This program is a library for abstracting the CSAFE commands into Rust enums
and structs, and providing an example program for sending commands and
parsing responses.

# Dependencies

The Rust toolchain is needed. This compiles under stable (1.52.1, as of today).

`libusb-1.0-0-dev` is required for the `hidapi` Rust crate to compile.


# Building

    cargo build

# Running

    cargo run --example hid_parser

`hidapi` is an exploratory program for examining the raw bytes that the Concept2
responds with.

Note that this only works if you're connected to a Concept2 device through USB.
Bluetooth doesn't seem to work, as the HID crate works through `libusb` rather
than `hidraw`, which is required for Bluetooth support.

# What Worked

I sent a variety of commands and got responses back! I also came up with some
pretty good abstractions; adding additional commands is very straightforward.
I had to do a fair amount of reverse engineering of the CSAFE specification,
as Concept2's documentation was extremely lacking in certain specifics,
especially exact sequences of bytes that comprised certain complicated commands.

# What Didn't Work

Bluetooth, unfortunately. The Rust crate for `hidapi` works with `libusb`, which
obviously doesn't work over Bluetooth. It's likely that I'll need to use Bluez
for this, which would be outside the scope of this project.

# Satisfied?

Yes, definitely. It still needs work to get an actual working project, but the
hard part is done; the rest is just scutwork.

# Testing

The unit tests can be run with

    cargo test

Testing involved a five-step process:

* Using the Concept2 specification to crafting the CSAFE data types needed to
send a command over HID.
* Sending the command through the exploratory `hidapi` program to get a response,
and manually parsing the resulting data to make sure that it was correct.
* Crafting a CSAFE Response data type and function to parse the bytes.
* Creating a unit test with that response and expected result 
to make sure that it was parsed correctly.
* Running the test again with `hid_parser` to make sure that the actual bytes
from the machine were parsed correctly.