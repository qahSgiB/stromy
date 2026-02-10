# How to run

1. install rust (https://rust-lang.org/)
2. run it `cargo run`

# Controls

Camera can is controled by mouse and keyboard.

 - WASD - rotate camera (also mouse)
 - IJKL + YH - move camera
 - UO - zoom camera (also mouse)

# Examples

In the folder `assets/lsystems` are some examples for you to try.

# L-system file format

There are two L-system file formats here.
First is the one taken as input by the rust application, this one is more complicated.
Second one is easier to use, and can be converted to first one by the python script `py/preprocess_grammar.py`.
Use `--help` option to see how it is used (it's simple).

Here is an example L-system file in the second simpler format (2d-tree example):
```
A 3.0 0.25


A
F p0 p1 * p1 c2
[
    Y c0
    A * p0 c1 * p1 c2 * p1 c2
]
[
    Y ! c0
    A * p0 c1 * p1 c2 * p1 c2
]


0.125 0.6 0.75


F F [ ] P Y R
```

First section is the axiom.
Second section is the rules.
Third sections are constants.
Fourth section are actions.

Each rule starts with the rule's left side. In the example there is one rule `A`.
Then there is the right side of the rule (the body).
One line of the body is one character from L-system alphabet followed by paramters applied to it.
Mathematical expressions are written in polish prefix notation.
For example line `A * p0 c1 * p1 c2 * p1 c2` is equivalent to `A(p0 * c1, p1 * c2, p1 * c2)` in normal notation.
Paramters taken by the rule are called: p0, p1, p2, ....

The third section is constant they are numbered from zero and can be referenced in rules by their names: c0, c1, c2, ....

The fourth sections translates characters to turtle commands.
In order the rule are specified we can specify action for the left side character of the rule.
Leave `X` to do nothing.
Than it is mandatory to add the six default actions at the end: `F [ ] P Y R`.
In our example the character `A` translates to action `F` and we have the default actions.