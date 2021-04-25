# CS510P: Rust Programming

## Programming Project: Concept2 HID Interaction

# Project Members

Michael Bottini ([bottini@pdx.edu](mailto:bottini@pdx.edu))

# Overview

Concept2 is a manufacturer of exercise equipment. Each of their ergometers
are monitored by an embedded microcomputer called a Performance Monitor, (PM)
which tracks data about the intensity and duration of the workout. This PM
is a Human Interface Device, (HID) which allows any operating system that
implements HID drivers to interact with it by either USB or Bluetooth.

The interaction is wide; the PM implements most of the Communications
Specification for Fitness Equipment, (CSAFE) which defines a wide interface
for controlling and monitoring fitness equipment.

However, at this time, there are only a couple of available ways to interact
over this stack - the official, closed-source implementation from Concept2, which
is only available on Android, and a couple of abortive attempts to implement
a NodeJS server that interfaces with the device and presents data. We can do better
than this.

# Existing Resources

* Rust has an HID crate, which serves as a wrapper over the `hidapi` C library.
This library serves as an abstraction for interacting with HIDs over both USB
and Bluetooth. Thus, the only thing that I need to do is learn how to use it.
* Concept2 has an ad-hoc, disorganized specification from 2006 in their SDK
that describes a lot of its functionality. It does not describe HID
interaction at all and is limited to a description of CSAFE and the SDK
functions that it implements. The SDK's functionality consists of several
closed-source DLLs that only work on Windows, but are helpful in that the
functions provide examples of CSAFE commands that the PM will respond to.
* The existing NodeJS implementation, which is useful despite the fact that its
HID implementation is very different from Rust's.

# Goals

* Learn how to use HID in Rust.
* Implement the CSAFE protocol and a variety of commands, both for sending requests
to the PM and parsing responses, and create a Rust crate that provides low-level
abstractions for interacting with the PM.
* Create rigorous documentation of this protocol for other people who want to follow
in my footsteps and don't want to read a disorganized heap of specs from 15 years ago.
* Implement a CLI that uses the above library to obtain data from the PM and print
it to the terminal.
* Implement a GUI that clones the existing closed-source ErgData Android
application so that anyone with a PC or laptop can interact with a PM instead
of using a tablet or phone. I will likely use GTK for this, but I might also
try Qt.

# Concerns

None with the meat of the project. I have already figured out HID,
implemented a couple of CSAFE commands, gotten back responses from the
device, and done a bunch of documentation of the protocol.

# Repo

[https://github.com/mbottini/concept2](https://github.com/mbottini/concept2)