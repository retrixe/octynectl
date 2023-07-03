# octynectl

Command-line interface to control Octyne.

## Quick Start

### For Windows

Download the latest octynectl from [GitHub Releases here](https://github.com/retrixe/octynectl/releases/latest) and place it in a folder in your PATH.

### For Linux and macOS

Run the following command:

> `sudo wget -O /usr/local/bin/octynectl https://github.com/retrixe/octynectl/releases/latest/download/octynectl-PLATFORM && sudo chmod +x /usr/local/bin/octynectl`

Replace `PLATFORM` in the command with your appropriate platform e.g. `linux-x86_64`, `linux-arm64`, `linux-armv6`, `macos-arm64` or `macos-x86_64` as necessary.

You should be able to run `octynectl` now.

## Configuration

Currently, there is no way to configure `octynectl`. Octynectl will only work with Octyne running on the default port 42069 with the Unix socket API enabled (and accessible by the current user, of course).
