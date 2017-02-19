# Example Wrangle

## Project Description:

Example app that uses most of Fractalide's features.

## Stability Status:

- [x] Raw
- [ ] Draft
- [ ] Stable
- [ ] Deprecated
- [ ] Legacy

## Build Instructions
Ensure you've installed [nix](https://nixos.org/nix).
```
$ export NIX_PATH+=:fractalide=https://github.com/fractalide/fractalide/archive/v20170218.tar.gz
$ git clone git://github.com/fractalide/example_wrangle.git
$ cd example_wrangle
$ nix-build --argstr rs test
```
