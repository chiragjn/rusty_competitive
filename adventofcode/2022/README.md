Advent of Code 2022
---
https://adventofcode.com/2022

Check clippy

```
for f in $(find . -name 'Cargo.toml' -exec dirname {} \; | sort); do cd $f; cargo clippy; cd ..; done;
```

Run all tests

```
for f in $(find . -name 'Cargo.toml' -exec dirname {} \; | sort); do cd $f; cargo t; cd ..; done;
```

Run for inputs

```
for f in $(find . -name 'Cargo.toml' -exec dirname {} \; | sort); do echo $f; cd $f; cargo build --release --quiet; /usr/bin/time -f "%es" cargo r --release --quiet < input.txt > /dev/null; cd ..; done;
```
