Advent of Code 2021
---
https://adventofcode.com/2021

TODO:

- [ ] Optimize Day 12
- [ ] Optimize Day 15
- [ ] Optimize Day 17

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
