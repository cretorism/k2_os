
# K2 OS

An OS made with Rust and Love <3



## License

[MIT](https://choosealicense.com/licenses/mit/)


## Requirements
 - [Rust Nightly](https://rust-lang.github.io/rustup/concepts/channels.html)
 - [QEMU](https://www.qemu.org/) (Optional)
## Installation
Clone the project

```bash
  git clone https://github.com/cretorism/k2_os
```

Go to the project directory

```bash
  cd k2_os
```

Run (With QEMU installed)

```bash
 cargo run 
```

Run (without QEMU)
- Run without QEMU
Load the .bin file after running

```bash
  cargo build
```


```bash
    cd target 
```

```bash
    cd x86_64-K2_os 
```

```bash
    cd debug
```
to a booatable disk.