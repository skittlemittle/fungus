# Useage

```
fungus <steps> <division>

# eg: 16 steps with 4 steps to a beat
fungus 16 4
```

- `s` soft hit
- ` ` regular hit
- `d` loud hit
- `j, k` to go down / up a track
- `h, l` to go left / right
- `m` to toggle mute
- '+' bpm++
- '-' bpm--

_These apply to the current track:_

- `c` to clear any beat on the current step
- `C` to clear the whole track

# Cross compiling

THIS ONLY WORKS ON armv7 PIs: 2/3/4/zero2 etc.

- build the container
  ```
  docker build -t pi .
  ```
- run it, the --user stuff is so the binary isnt owned by root
  ```
  docker run -v $PWD:/src --user $(id -u):$(id -g) --rm -ti pi cargo build --target=armv7-unknown-linux-gnueabihf
  ```
- your executable is in target/armv7-unknown-linux-gnueabihf/debug
