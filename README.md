# ruscio

## 1. Introduction
Welcome to Rust Rocket REST app tutorial, where we'll explore the powerful combination of Rust's performance and Rocket's simplicity in building web applications. In this project, we'll delve into the world of cryptography, showcasing how Rust excels in fast and secure calculations. Through implementing algorithms like AES, RSA, and SHA-256, we'll demonstrate Rust's efficiency in handling sensitive data encryption and hashing. Additionally, we'll leverage Rocket's intuitive framework to create RESTful endpoints, enabling seamless communication with our cryptographic functionalities.

## 2. Requirements
 - VS Code or other code editor

## 3. Rust installation
We will need:

`rustup` - a command line tool for managing Rust versions and associated tools (`rustc`, `cargo`).

`rustc` - Rust compiler\
`cargo` - Rust’s build system and package manager
### Linux & macOS
Run in the terminal:
`curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`

The command downloads a script and starts the installation of the rustup tool, which installs the latest stable version of Rust. You might be prompted for your password. If the install is successful, the following line will appear:
`Rust is installed now. Great!`

You will also need a linker, which is a program that Rust uses to join its compiled outputs into one file. It is likely you already have one. If you get linker errors, you should install a C compiler, which will typically include a linker. A C compiler is also useful because some common Rust packages depend on C code and will need a C compiler.

On macOS, you can get a C compiler by running:
`xcode-select --install`

Linux users should generally install GCC or Clang, according to their distribution’s documentation. For example, if you use Ubuntu, you can install the `build-essential` package.

### Windows
On Windows, go to https://www.rust-lang.org/tools/install and follow the instructions for installing Rust. At some point in the installation, you’ll receive a message explaining that you’ll also need the MSVC build tools for Visual Studio 2013 or later.

To acquire the build tools, you’ll need to install Visual Studio 2022. When asked which workloads to install, include:

- “Desktop Development with C++”
- The Windows 10 or 11 SDK
- The English language pack component, along with any other language pack of your choosing

***
#### Troubleshooting
Now check if all packages were installed correctly:

Run: `rustup --version`

If you can see in the output `rustc x.y.z (abcabcabc yyyy-mm-dd)`, you have installed Rust successfully!

If not, check the `PATH` environment variable.

In the Rust development environment, all tools are installed to the `~/.cargo/bin` directory, and this is where you will find the Rust toolchain, including `rustc`, `cargo`, and `rustup`.

Locate `~/.cargo/bin` on your PC and add full directory to `PATH` variable. 
***
Once Rust is installed via rustup, let's update to a newly released version:

Run: `rustup update`

#### Package versions used in this tutorial:
`rustup 1.27.0`\
`rustc 1.78.0`\
`cargo 1.78.0`
