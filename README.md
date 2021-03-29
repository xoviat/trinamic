# Trinamic Motion Control Language (TMCL) in Rust

This project aims to: 
- Closely mirror [PyTrinamic](https://github.com/trinamic/PyTrinamic)
- Be operable on both std and no-std environments
- Employ async as the primary API

**Features are currently added on an as-needed basis. If you need functionality, open an issue!**

## What this project isn't.

This project isn't a driver for Trinamic chips. The scope of this project is to communicate via TMCL
or CANOpen to Trinamic motors and other equipment on a std (PC) or no-std (embedded) setting. Note
that CANOpen will be difficult to implement because there currently isn't a usable rust CANOpen
library, but it is within the scope of this project. However, if you're developing a system with
Trinamic hardware, you should consider selecting the TMCL version for now because it's currently
supported and new drivers are trivial to implement, whereas CANOpen will not be.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
