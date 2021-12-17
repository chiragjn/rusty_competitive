Advent of Code 2021
---
https://adventofcode.com/2021

TODO:

- [ ] Fix Day 15 code 
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
for f in $(find . -name 'Cargo.toml' -exec dirname {} \; | sort); do cd $f; cargo r --release < input.txt; cd ..; done;
```
