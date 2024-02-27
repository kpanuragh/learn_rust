## Installing Rust

Rust is a versatile, modern programming language known for its speed, safety, and memory efficiency. This Readme provides instructions for installing Rust on various operating systems:

**Prerequisites:**

-   **Linux:**  A 64-bit Linux distribution with a package manager (e.g.,  `apt`,  `yum`,  `dnf`,  `pacman`).
-   **Windows:**  Windows 10 or later, 64-bit architecture.
-   **macOS:**  macOS 10.13 or later, 64-bit architecture.

**Installation Methods:**

**1. Using the Rustup installer:**

We highly recommend using the Rustup installer, as it provides a convenient way to manage multiple Rust toolchains and keep them up-to-date.

**Linux/macOs:**

1.  Open a terminal and run:


```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.dev | sh

```
2.  Follow the on-screen instructions to install Rustup.
3.  Restart your terminal or shell for the changes to take effect.

**Windows:**

1.  Download the Rustup installer for Windows from  [https://learn.microsoft.com/en-us/windows/dev-environment/rust/setup](https://learn.microsoft.com/en-us/windows/dev-environment/rust/setup).
2.  Run the installer and follow the on-screen instructions.

**2. Using system package managers (Linux only):**

Many Linux distributions provide Rust packages through their package managers. This method is less flexible but may be suitable for quick installations.


```
# Debian/Ubuntu
sudo apt install rustc cargo

# Fedora/CentOS
sudo dnf install rust

# Arch Linux
sudo pacman -S rust

# Other distributions
Consult your distribution's documentation for specific package names and installation instructions.

```

**Verification:**

After installation, open a terminal or command prompt and run:


```
rustc --version
cargo --version

```
These commands should output the installed Rust and Cargo versions.

**Additional Notes:**

-   For more advanced Rustup configurations, refer to the official documentation:  [https://rustup.rs/](https://rustup.rs/)
-   Consider setting up a development environment (e.g., using VS Code, IntelliJ IDEA) for a smoother Rust development experience.
-   If you encounter issues, seek help from the Rust community forums or official channels.
