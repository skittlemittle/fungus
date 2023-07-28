# Useage

## In terminal

- `j, k` to down / up a track
- `m` to toggle mute
- '+' bpm++
- '-' bpm--

_These apply to the current track_

- `Space` to place a beat on the current step
- `c` to clear any beat on the current step
- `z` to clear the whole track

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
