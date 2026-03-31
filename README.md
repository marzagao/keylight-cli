[![CI](https://github.com/jldeen/keylight-cli/actions/workflows/build-ci.yml/badge.svg)](https://github.com/jldeen/keylight-cli/actions/workflows/build-ci.yml)

# Elgato Keylight CLI

This is a cross platform lightweight CLI tool to simply and easily control your [Elgato Keylights](https://www.elgato.com/key-light) via local IP address. 

## Work To Be Done

- [X] ~~Support for `on` / `off` toggle arguements.~~ **Added in v0.2.0**
- [X] ~~Add help menu with `-h` flag.~~ **Added in v0.2.0**
- [X] ~~Add status to query for on/off~~ **Added in v0.2.2**
- [ ] Support for brightness and temperature via preset arguments, I.E, `low`, `medium`, and `high` or `warm`, `medium`, and `cool`.
- [ ] Support for brightness by percentage.
- [X] ~~Autodiscovery support.~~ **Added in v0.3.0**
- [X] ~~Testing with more than 1 Elgato Keylight.~~ **Tested in v0.3.0**

## Building The App

This app should build with minimal dependencies.  It's been tested with Rust 1.60+ on macOS and 2 Elgato Keylights.

```sh
cargo build
sudo mv target/debug/keylight /usr/local/bin/keylight
```

## Running The App

There are two ways to target your Elgato Keylights: autodiscovery via mDNS or manual IP address.

### Autodiscovery (recommended)

Use the `--discover` / `-d` flag to automatically find all Elgato Keylights on your local network via mDNS:

```sh
# Check the status of all lights on the network
keylight status --discover

# Turn all discovered lights on
keylight on --discover

# Turn all discovered lights off
keylight off --discover
```

### Manual IP address

You can also specify the IP address directly:

```sh
keylight status --elgato-ip <ip-address-here>
keylight on --elgato-ip <ip-address-here> --brightness 30 --temperature 200
```

Environment variables can be provided in place of CLI arguments.

### Full usage

```
keylight v0.3.0
Jessica Deen <jessicadeen@me.com>, Thompson Marzagao <thompson@marzagao.com>
Easy CLI to control Elgato Keylight

USAGE:
    keylight [OPTIONS] <on/off/status>

ARGS:
    <on/off/status>    Toggle light on, off, or query current power state [possible values: off,
                       on, status]

OPTIONS:
    -b, --brightness <brightness>
            Brightness value for light [env: brightness=] [default: 20]

    -d, --discover
            Auto-discover Elgato Keylights on the local network via mDNS

    -h, --help
            Print help information

    -i, --elgato-ip <elgato_ip>
            Elgato Keylight IP address [env: elgato_ip=]

    -n, --number-of-lights <number_of_lights>
            Number of Elgato Keylights in use [env: number_of_lights=] [default: 1]

    -t, --temperature <temperature>
            Temperature value for light [env: temperature=] [default: 213]

    -v, --verbose
            Log Level

    -V, --version
            Print version information
```

## Setting up as daemon on macOS

```sh
# clone this repo
# from within root of this repo folder
mkdir -p ~/bin && cp onair.sh ~/bin # makes a bin directory in your user's home folder, copies onair script to that folder

# add your ip address and system username to the plist file
sed -i 's/<REPLACE_IP_ADDRESS>/your-elgato-ip-address-here/g' com.keylight.daemon.plist
sed -i 's/<REPLACE_USER>/your-username-here/g' com.keylight.daemon.plist

# copy updated plist to launchdaemon folder
cp com.keylight.daemon.plist /Library/LaunchDaemons/com.keylight.daemon.plist

# load/start daemon/plist
sudo launchctl load -w /Library/LaunchDaemons/com.keylight.daemon.plist

# view logs
tail -f /tmp/keylight.stdout #standard out logs
tail -f /tmp/keylight.stderr #standard error logs
```