## Example_Wrangle

Example on how do to a bunch of file processing using Fractalide

### Build Instructions

Make sure you have installed [`nix`](http://nixos.org/nix/), most likely your package manager has `nix` available, if not you'll need to compile from source.

Nix is the only direct dependency you have to install "manually", basically it's a make replacement.

```
$ NIX_PATH="nixpkgs=https://github.com/NixOS/nixpkgs/archive/125ffff089b6bd360c82cf986d8cc9b17fc2e8ac.tar.gz:fractalide=https://github.com/fractalide/fractalide/archive/master.tar.gz"
$ export NIX_PATH
$ git clone https://github.com/fractalide/frac_example_wrangle.git
$ cd frac_example_wrangle
$ nix-build
downloading ‘https://github.com/fractalide/fractalide/archive/7db197c99f145ee0e7506f8c5f7d71d128dbd67a.tar.gz’... [0/0 KiB, 0.0 KiB/s]
downloading ‘https://github.com/NixOS/nixpkgs/archive/125ffff089b6bd360c82cf986d8cc9b17fc2e8ac.tar.gz’... [0/0 KiB, 0.0 KiB/s]
/nix/store/sa0nlg8s517j454sdsnfcgl7zsclp82z-frac_example_wrangle

$ ./result/bin/frac_example_wrangle
```
