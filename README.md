# writedisk

Small, cross-platform utility for writing a disk image to a USB drive.

**Usage: `writedisk <path/to/file>`**

This will scan for connected removable drives and prompt you to select
one. Then the input file will be copied to the drive. The copying
operation is done with a `wd_copier` process that is
automatically invoked with elevated permissions.

## Installation

### Cargo

```shell
cargo install writedisk
```

### Nix

Per user:

```shell
nix-env --install writedisk
```

System-wide:

```shell
environment.systemPackages = with pkgs; [ writedisk ];
```


---
This software is available under the [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0.html) license
