# Pirate MIDI Updater

This is a (hopefully) easy-to-use cross-platform executable designed to update the firmware for Pirate MIDI devices.

It supports updates for the following devices:
- [x] Bridge 6
- [x] Bridge 4
- [x] CLiCK
- [ ] uLOOP

It's backend functionality is written in [Rust](https://www.rust-lang.org/), and uses the [Tauri Framework](https://tauri.app/) + Typescript + Next.js for the GUI.

## Status

This app is considered stable, but that doesn't mean it can't run into issues. 

If you're concerned, or believe you may have bricked your device, please read the section: [Bridge Device Recovery](#bridge-device-recovery)

## Building Locally

You can build this locally if you desire! Here's what you'll need:
- [Rust](https://www.rust-lang.org/tools/install)
- [Node](https://nodejs.org/en/)

Once you have it all setup, run:
- `npm install --legacy-peer-deps`
- `npm run tauri dev`

### Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) 
- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Bridge Device Recovery

_Note: These instructions are for the Bridge 6 and Bridge 4 devices._

If you're concerned, or believe you may have bricked your device, there is a path to reapply the update! The creators of the Bridge devices had great foresight for this exact issue, and you should do the following:

- **DON'T PANIC**
- Download the latest release from the [Pirate MIDI Website](https://learn.piratemidi.com/software/downloads).
- Make sure you're not supplying power via the 9V port!
- For the Bridge6, hold FS6 while powering up/plugging in a USB cable.
- For the Bridge4, hold FS3 while powering up/plugging in a USB cable (Thanks Simon!).
- Wait about 10-15 seconds, as the device won't appear to do anything.
- As a backup method, you can use the `dfu-util` command as [laid out here](https://learn.piratemidi.com/software/downloads) (click "Details & Instructions").
