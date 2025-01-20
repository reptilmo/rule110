
# rule110

### Program created by Kostya Leshenko for CS423 HW2.

A very simple implementation of [Rule 110](https://en.wikipedia.org/wiki/Rule_110).
The program prints ten rows starting with
```
*.*..*..
```
the subsequent nine rows are generated using Rule 110 automaton rules.

The program is able to take one command line parameter to specify the starting row.
For example:
```
$ ./rule110 "..**..*."
```
It's very important to use single or double quotes when specifying the paramter,
otherwise the command shell will attempt to interpret it as special characters.

## Building:
Use cargo build :-)

## Documentation:
Use cargo doc :-)
