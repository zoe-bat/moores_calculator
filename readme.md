# about
Tells you how many years you need to wait until your subatomic xeon crystal synchronizer has doubled in plasma inversion efficiency on the Goldberg-Moleman scale or whatever.

I made this instead of doing homework and because i wanted to learn rust. 🥴

# usage example
```
~/Code/Rust/moores_calculator/cargo run
How high is the value you want to grow?
10
How high do you need that value to be?
20
What's the growth rate per time unit? (in percent)
1
Right now your value is 10. With a growth rate of 1% you will have to wait 69.66078 time units until it has reached 20.
```

# build
* [install rust](https://www.rust-lang.org/tools/install)
* clone this repo `git clone https://github.com/zoe-bat/moores_calculator.git`
* cd to the folder where you downloaded it `cd moores_calculator`
* compile it `cargo build --release`
