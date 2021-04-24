# Chimp-8

A [Chip-8](https://en.wikipedia.org/wiki/CHIP-8) emulator built using [Rust](rust-lang.org).

---

## How to run

### For the desktop version:

```
$ cargo run --release --bin chimp_desktop roms\PUZZLE
```

### For the web version:

1. install `wasm-pack`

```
$ cargo install wasm-pack
```

2. while being in `chimp_wasm` directory, run:

```
$ wasm-pack build --target web
```

that will produces a `pkg` folder. we need 2 files from that folder:

    - `chimp_wasm.js`
    - `chimp_wasm_bg.wasm`

We have a symbolic link in the `web` folder for them, so there is no need to copy them.

4. server the `web` folder using python:

```
$ python -m http.server --directory web
```

or use `miniserve`

```
$ miniserve web
```

5. (optional) If we want to add more roms to the web dropdown list instead of using the browse version. First, we need put them inside the `web/roms` directory. Then, while we are inside the `web/roms` directory, we run:

```
$ python ../../generate_rom_list.py -d .
```

This will generate a new `rom_list.txt` which will be read from the javascript and auto-populate the dropdown list. 

---


### References

My main reference is **Austin Bricker**'s [Introduction to Chip-8 book](https://github.com/aquova/chip8-book).

But for my previous attempts I used these references:

- **Chip-8** [Wikipedia Entry](https://en.wikipedia.org/wiki/CHIP-8).
- **Cowgod**'s [Chip-8 Technical Reference](https://en.wikipedia.org/wiki/CHIP-8).
- **Mat Mikolay**'s [Chip-8 Technical Reference](https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference).

---

### Credits

The game packs are from [**Zophar's Domain**](https://www.zophar.net/pdroms/chip8/chip-8-games-pack.html).

---

### License

[MIT](./LICENSE)