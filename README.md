# workle
A Wordle solver written in Rust. I have started making it a sort of tradition where each language I learn I write a wordle solver in it as one of my first projects, and this is no exception. While my last wordle solver [worple](https://github.com/TrojanPinata/worple) gave a exact answer and had a nice gui, I decided to return to my roots and make a nice CLI program which gives all possible answers. This lets the user decide what they think is best instead of making a assumption. This solver has no tricks, and simply brute forces the answer by narrowing down what legal five letter word the solution can be from the inputs. This one also accounts for most (if not all!) edge cases.

![w o r k l e](https://i.imgur.com/DT3FH2Q.png)

## How to use
This is a Rust program, so if you want to compile then run `cargo build` and `cargo run`

If you just want to run it and are on windows, then download the release and possible.txt file and run the .exe file in a terminal. If you get a `The system cannot find the file specified` error then possible.txt is not in the same directory as the .exe.

(or don't and look through the code and compile it yourself bc you really shouldn't be running random programs from the internet)

