# .prtgn

A Protogen file extension written in Rust. 

![.prtgn logo](https://github.com/PRTGN-Development-Team/.prtgn/blob/b8513ab7eab12692378fa95eada18cb786817ed9/Artiy_The_PRTGN_Protogen/prtgn_logo_colour.png)

> [!CAUTION]
> 
>.prtgn, a protogen inspired file extension written in Rust.
>Copyright (C) 2025  PRTGN Development Team
>
>This program is free software: you can redistribute it and/or modify
>it under the terms of the GNU General Public License as published by
>the Free Software Foundation, either version 3 of the License, or
>(at your option) any later version.
>
>This program is distributed in the hope that it will be useful,
>but WITHOUT ANY WARRANTY; without even the implied warranty of
>MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
>GNU General Public License for more details.
>
>You should have received a copy of the GNU General Public License
>along with this program.  If not, see <https://www.gnu.org/licenses/>.

## What we have to offer

Welcome to .prtgn! The all new CLI file format for Protogens. Powered by Rust and [![Built With Ratatui](https://img.shields.io/badge/Built_With_Ratatui-000?logo=ratatui&logoColor=fff)](https://ratatui.rs/)!

We offer 'secure' files only select programs have the ability to understand, so your RAM cash and USB serial is safe with us!

Along with that we have a wonderful Protogen friendly CLI interface, and a built-in translation for Human or other species to understand.  

Have a look at our wonderful command structure! Start out with 'prtgn' and then apply any of the following sub commands!

>[!TIP]
>Check out [wiki](https://github.com/ExoticDG/.prtgn/wiki) for the different commands!

## Wana help?

Some ways to contribute are code or ideas, but you stil can contribute if you don't have either of those!

Fanart is a wonderful way to show support, there is a referance drawing for the mascot, Artiy, located in the [PRTGN_Artwork Reposotory](https://github.com/PRTGN-Development-Team/PRTGN_Artwork).

If you cant art, you can still help! Shareing arround helps more than you might think! Make a post about us, share us with word of mouth or digital word of mouth. The more people know, the more support there is.

## A few other protogen repositories 

[Proto-OS](https://github.com/dimitrivlachos/Proto-OS), "An operating system for a Protogen head which utilizes computer vision and machine learning to replicate the user's facial expressions and eye position onto the exterior displays of the character head. Made for an RPI4." -- [dimitrivlachos](https://github.com/dimitrivlachos)

[Proto-Ear-Twitch](https://github.com/stef1949/Proto-Ear-Twitch), "Code for controlling protogen ears" -- [stef1949](https://github.com/stef1949)

[ProtogenHelmet-ESP32](https://github.com/NCPlyn/ProtogenHelmet-ESP32), "Controller & Remote & Animator for Furry Protogen helmet using ESP32-S3 & MAX7219/WS2812" -- [NCPlyn](https://github.com/NCPlyn)

[PRTGN](https://github.com/bismarx-v1/PRTGN) "OwO whats this (This repo contains electronics, mechanical and software data for a protogen visor)" -- [bismarx-v1](https://github.com/bismarx-v1) | NOTE : **'PRTGN by bismarx' IS UNRELATED TO THIS PROJECT AND THE PRTGN-Development-Team**

## Plans

- [x] Rataui for a CLI UI for file editing and all sorts of stuff -- https://ratatui.rs/tutorials/json-editor/ \\ https://ratatui.rs \\ https://github.com/rhysd/tui-textarea
- [x] CLI command (prtgn) for doing things. Example, `prtgn new <filename>` or something like that would create a new file and open the file editing UI -- https://rust.code-maven.com/clap-subcommand \\ https://medium.com/coderhack-com/writing-a-cli-tool-in-rust-237d7e6417f6 \\ https://rust-cli.github.io/book/tutorial/index.html
- [x] 'Security' Through Obscurity
- [x] Fedora / Rocky / RHL Support
- [x] ARM support
- [ ] More than text in the files. I.E. Making it able to do more stuff. Maybe images or a wrapper for Rust or something. | [Terminal IDE in Rust - Helix](https://github.com/helix-editor/helix)
- [x] Automatically adding .prtgn to a filename in the init command
- [ ] Benchmark/test sub command \\ usage stats and whatnot \\ https://github.com/sharkdp/hyperfine ?
- [ ] *File format converter*
- [x] MIDI / other musics | Inspired by [Ivycomb](https://youtube.com/@ivycomb?si=hL9f19mSvyffFUk1) - [YTShort](https://youtube.com/shorts/dQyZ-WTuBwQ?si=PoWy2zuMMxrF3mQX) / [Ivycomb Music](https://youtube.com/@ivycombmusic?si=K92ak8535oQ7ik8r) - [YTMusic](https://music.youtube.com/watch?v=J620cBDOrj4&si=S0GaU3D3IH-71s0k)
- [ ] Website

## For Thoust dev's

**Debian (deb) install package :** [Cargo-deb](https://crates.io/crates/cargo-deb)

**Fedora (rpm) install package :** [Cargo-rmp](https://crates.io/crates/cargo-rpm)

**Microsoft Windows install package :** [Inno Setup](https://jrsoftware.org/isinfo.php)

INNO Registry PATH : `Root: HKCU; Subkey: "Environment"; ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}"`
