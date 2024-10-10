# Ani2Png-rs

Rust rewrite of [this script](https://github.com/Mastermindzh/Scripts/blob/master/c%2B%2B/ani2png.c) as i wanted to use this [miku cursor](https://x.com/blz_pixel/status/1844167444321210838) on my linux install.


## Build 

1. Clone repo

```bash
git clone https://github.com/fvckgrimm/ani2png-rs && cd ani2png-rs
```

2. Build

```bash
cargo build -r
```

## Usage

```bash
ani2png-rs Move.ani
```

It will create a subdirectory called `Move` and place all the png files for the related .ani file in there to be used.
