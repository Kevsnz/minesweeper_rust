# Minesweeper in Rust

## Overview

This is an implementation of original Microsoft Minesweeper in Rust. It is made by me over the course of 2 days completely from scratch including all the code and assets.

Minesweeper in Rust uses SDL library.

## Installation Instructions

1. Install Rust from [rust-lang.org](https://www.rust-lang.org/)
2. Clone this repository to any directory with write access
3. Build and run package with the following command
```sh
cargo run
```

## Changing game settings

By default game starts with the field of 8x8 tiles and 10 mines.

To change field size and mine count provide respective values as command line arguments:

```sh
cargo run -- <width> <height> <mine count>
```

## Differences from original game

Two-way click on number to reveal all adjacent non-flagged tiles replaced with simple left click. It works only if number of flags is the same as the number on clicked tile.

This change significantly reduces hand strain and avoids mice grinding but introduces risk of accidentally misclicking on a mine.

## Licence

[![Creative Commons «Attribution-NonCommercial» 4.0](https://i.creativecommons.org/l/by-nc/4.0/88x31.png)](http://creativecommons.org/licenses/by-nc/4.0/)
