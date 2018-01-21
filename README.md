The Synacor Challenge
=====================

It consists on implementing a emulator according to the spec (arch-spec) to be able to read challenge.bin

See [the official website](https://challenge.synacor.com/) for more details.

What's in here
--------------

### The emulator itself

`cargo run --release <path-to-binary>` will run your binary.

When using the challenge binary, you will not be able to save your adventure.
To circumvent this, save in a file the commands you used. My solution is in [adventure.txt](adventure.txt).

Then run the emulator with `cat adventure.txt - | cargo run --release <path-to-binary>`

### Coin challenge

A brute force solution (tests all the permutations) to the coin challenge is in [coin.rs](src/bin/coin.rs).

Run it with
`cargo run --bin coin`

Solution
--------

There is 8 codes to find.
1. Read the spec
2. Implement the basic intructions.
3. Implement all the instuction so that the program self-test pass. You now have a working VM.
4. Use your first item
5. In the twisty passage with the can
6. Solve the coin problem to get the teleporter
