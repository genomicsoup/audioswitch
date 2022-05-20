# audioswitch

`audioswitch` swaps out the current in-use audio output with any other available device
using a simple command line interface. 
It was initially written to provide an easy-to-use method for swapping between headphone and
speaker outputs using a [duckyPad](https://github.com/dekuNukem/duckypad).
There is no easy way on Windows to configure hot key audio switching or let an application swap
audio outputs, so this CLI was made to do so.

## How it works

Windows doesn't provide a (documented) programmatic means or an API to let applications swap audio 
outputs.
To overcome this, `audioswitch` uses the undocumented `IPolicyConfig` API first reverse engineered 
by [EreTIk](http://web.archive.org/web/20170506033941/http://eretik.omegahg.com/art/07.html).
This CLI implements the `IPolicyConfig` 
virtual method table and interface in Rust.
It uses the `SetDefaultEndpoint()` function to change audio devices to one a user specifies.

## Usage

```powershell
$ .\audioswitch.exe --help
audioswitch 0.1.0
Switch the current, default audio device on Win10

USAGE:
    audioswitch.exe <--list|--device-id <device-id>|--device-name <device-name>>

OPTIONS:
    -h, --help                         Print help information
    -i, --device-id <device-id>        Switch audio output using a device ID
    -l, --list                         List available audio devices
    -n, --device-name <device-name>    Switch audio output using a device name
    -V, --version                      Print version information
```

Use the `--list` option to list the current, available audio devices.
This will output the unique ID and name for each audio device:

```powershell
$ .\audioswitch.exe --list
--- Listing available audio devices
<AudioDevice id: {0.0.0.00000000}.{2d045987-eda7-4a7d-874c-f1ad1b4b8dab} name: Speakers (Steam Streaming Speakers)>
<AudioDevice id: {0.0.0.00000000}.{4e08256a-eb68-4f08-b890-0e384cd975a6} name: Headphones (High Definition Audio Device)>
<AudioDevice id: {0.0.0.00000000}.{73b45fae-5b9c-4bb3-8f6e-71dc6af92df5} name: Speakers (Audioengine HD3 Stereo)>
<AudioDevice id: {0.0.0.00000000}.{f1d26838-9f97-4051-991d-f0d7d99d5b16} name: Speakers (NVIDIA Broadcast)>
<AudioDevice id: {0.0.1.00000000}.{fb8f8f15-0870-404c-9340-3891f5a7b8fd} name: Microphone (NVIDIA Broadcast)
```

Then use either the `--device-id` or `--device-name` option to swap to a different audio device using the ID or name respectively.

```powershell
$ .\audioswitch.exe --device-id "{0.0.0.00000000}.{73b45fae-5b9c-4bb3-8f6e-71dc6af92df5}"
--- Successfully changed audio devices
```

```powershell
$ .\audioswitch.exe --device-name "Headphones (High Definition Audio Device)"
--- Successfully changed audio devices
```

## Development

This (obviously) requires and only runs on Windows.
It's only been tested on Windows 10 but might work on Win 7/8.
Use `cargo` to build:

```powershell
$ cargo build
```

Run tests:

```powershell
$ cargo test --all
```