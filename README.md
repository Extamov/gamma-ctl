# gamma-ctl
gamma-ctl is a CLI software that allows changing monitor gamma settings.

Currently the software only supports Windows, in the future support for Linux and MacOS will be added.
## Usage
Run the following command to change the gamma of the monitor:
```
gamma-ctl <monitor-index> <brightness> <contrast> <gamma>
```
### Arguments
* Monitor Index: A number between 1 and 9.
* Brightness: A number between 0 and 100, the default is 50.
* Contrast: A number between 0 and 100, the default is 50.
* Gamma: A number between 0.3 and 2.8, the default is 1.
## Building
1. Install Rust (If using Windows also install Visual C++ Build Tools).
2. Install Rust Nightly toolchain with the following command:
```
rustup toolchain install nightly -c rust-src
```
3. Clone the project.
4. Compile the project with the following command:
```
cargo build --release
```