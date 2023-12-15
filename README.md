### Rust Thread Test

![Rust](https://img.shields.io/badge/language-Rust-orange)

This Rust Thread Test project is designed to explore multithreading in Rust by running a simple counter operation on multiple threads and measuring their performance. This isn't a great way to benchmark a CPU, but it most certainly counts to 100 million (on each logical processor) real fast!

## Running the Code

# Windows

To run the program on Windows, you can download and run the executable, if you want to actually read the output before it closes I recommend running it from the command line or powershell.

# Other (Mac/Linux)

You will need to build the executable on your platform.

## Building an executable

```bash
git clone https://github.com/seanpatrickduggan/rust_thread_test.git
```

```bash
cd rust_thread_test
```

```bash
cargo build
```

## Running the executable
```bash
cargo run
```


## Output
```bash
Sorting by cpu #:
CPU 0: Counter = 100000000, Time elapsed = 0.71498810 seconds
CPU 1: Counter = 100000000, Time elapsed = 0.71914690 seconds
CPU 2: Counter = 100000000, Time elapsed = 0.71630310 seconds
CPU 3: Counter = 100000000, Time elapsed = 0.71884270 seconds
CPU 4: Counter = 100000000, Time elapsed = 0.71752730 seconds
CPU 5: Counter = 100000000, Time elapsed = 0.72019880 seconds
CPU 6: Counter = 100000000, Time elapsed = 0.71637680 seconds
CPU 7: Counter = 100000000, Time elapsed = 0.75163640 seconds
CPU 8: Counter = 100000000, Time elapsed = 0.76221780 seconds
CPU 9: Counter = 100000000, Time elapsed = 0.71835520 seconds
CPU 10: Counter = 100000000, Time elapsed = 0.71793910 seconds
CPU 11: Counter = 100000000, Time elapsed = 0.71637010 seconds
CPU 12: Counter = 100000000, Time elapsed = 0.71823320 seconds
CPU 13: Counter = 100000000, Time elapsed = 0.74691950 seconds
CPU 14: Counter = 100000000, Time elapsed = 0.76425580 seconds
CPU 15: Counter = 100000000, Time elapsed = 0.71655880 seconds
CPU 16: Counter = 100000000, Time elapsed = 0.73102180 seconds
CPU 17: Counter = 100000000, Time elapsed = 0.72243820 seconds
CPU 18: Counter = 100000000, Time elapsed = 0.71451580 seconds
CPU 19: Counter = 100000000, Time elapsed = 0.73994710 seconds
CPU 20: Counter = 100000000, Time elapsed = 0.71902340 seconds
CPU 21: Counter = 100000000, Time elapsed = 0.73184010 seconds
CPU 22: Counter = 100000000, Time elapsed = 0.71550180 seconds
CPU 23: Counter = 100000000, Time elapsed = 0.71627700 seconds
Sorting by speed:
CPU 18: Counter = 100000000, Time elapsed = 0.71451580 seconds
CPU 0: Counter = 100000000, Time elapsed = 0.71498810 seconds
CPU 22: Counter = 100000000, Time elapsed = 0.71550180 seconds
CPU 23: Counter = 100000000, Time elapsed = 0.71627700 seconds
CPU 2: Counter = 100000000, Time elapsed = 0.71630310 seconds
CPU 11: Counter = 100000000, Time elapsed = 0.71637010 seconds
CPU 6: Counter = 100000000, Time elapsed = 0.71637680 seconds
CPU 15: Counter = 100000000, Time elapsed = 0.71655880 seconds
CPU 4: Counter = 100000000, Time elapsed = 0.71752730 seconds
CPU 10: Counter = 100000000, Time elapsed = 0.71793910 seconds
CPU 12: Counter = 100000000, Time elapsed = 0.71823320 seconds
CPU 9: Counter = 100000000, Time elapsed = 0.71835520 seconds
CPU 3: Counter = 100000000, Time elapsed = 0.71884270 seconds
CPU 20: Counter = 100000000, Time elapsed = 0.71902340 seconds
CPU 1: Counter = 100000000, Time elapsed = 0.71914690 seconds
CPU 5: Counter = 100000000, Time elapsed = 0.72019880 seconds
CPU 17: Counter = 100000000, Time elapsed = 0.72243820 seconds
CPU 16: Counter = 100000000, Time elapsed = 0.73102180 seconds
CPU 21: Counter = 100000000, Time elapsed = 0.73184010 seconds
CPU 19: Counter = 100000000, Time elapsed = 0.73994710 seconds
CPU 13: Counter = 100000000, Time elapsed = 0.74691950 seconds
CPU 7: Counter = 100000000, Time elapsed = 0.75163640 seconds
CPU 8: Counter = 100000000, Time elapsed = 0.76221780 seconds
CPU 14: Counter = 100000000, Time elapsed = 0.76425580 seconds
```

