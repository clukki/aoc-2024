# aoc-2024

_My entry on the Advent of Code 2024! (and learning rust throughout the project)_

## Usage

> [!WARNING]
> THIS PROJECT DOES NOT PROVIDE A FUNCTIONAL PROGRAM OUT-OF-THE-BOX! YOU MUST MODIFY THE _main.rs_ FILE!
> (the file currently has a "example" to run the day 1 part 2 program)

> [!NOTE]
> error handling on this project is _not_ great (unwarp madness)

Import the correct file according to the day that you want to run.

```rust
mod day1; // or any day you want (e.g. day2, day3,..)
```

You could keep the input file reading section already inside main.rs for convenience

Each module has 2 public functions, named `part1` and `part2`, corresponding to 2 part of each day's challenge

You could use it like this:

```rust
mod day1;

fn main() {
	println!("{}", day1::part1(input)) // variable "input" is not defined yet, you must define one in real code
}
```

Good luck and have fun on this season's AoC!!!
