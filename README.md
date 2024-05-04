# Hot and Cold

> Work in progress, is not runnable

Hot or cold. Your goal is the guess a secret integer between 1 and N . You repeatedly
guess integers between 1 and N . After each guess you learn if it equals the secret integer (and
the game stops); otherwise (starting with the second guess), you learn if the guess is hotter
(closer to) or colder (farther from) the secret number than your previous guess.

```
# ./hot_cold <num>

===================================================
Welcome to the Hot-Cold Guess game, copyright 2024.
---------------------------------------------------

These are the game rules:
    * I’ll choose a random number in [1,num]. Your job is to guess that number.
    * From the second guess onward, I’ll tell you if your guess
        is hot (closer than the previous guess) or cold (farther from
        the previous guess).
    * If you wish to quit the game, just type in a negative number.
Good luck!
----------------------------------------------------
>>> Guess the number: 35
>>> Nop, try again: 18
>>> Nop, it’s hot though, try again: 27
>>> Nop, it’s getting cold, try again: 21
>>> Nop, it’s hot though, try again: 22
>>> Yeah, correct guess!
Thanks for playing
```

## Running:

### C++ version:

1. Make sure you have _cmake_ and a c++ compiler like _gcc_ or _clang_ installed.

2. Load the cmake project a make a _build_ directory:

```
# cmake -S cpp -B cpp/build
```

3. Compile the game:

```
# cmake --build cpp/build
```

4. Run and play! :joystick:

```
# ./cpp/build/hot_cold <number>
```

### Rust :crab: version:

1. Make sure you have _cargo_ installed.

2. Go to the project folder:

```
# cd rust/hot-n-cold
```

3. Build the project

```
# cargo build --release
```

4. Run and play! :joystick:

```
# cargo run -- <number>
```

Alternatively

```
# ./target/release/hot-n-cold <number>
```
