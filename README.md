![CircleCI](https://circleci.com/gh/Asone/b39wc.svg?style=svg)
# BIP 39 Words Checker


The current project is a minimalist personal project in order to produce software learning and mastering rust. 

It is basically a software to check weither the words provided in a 24 mnemonic seed are all from the official seed dictionaries provided in [bitcoin core](https://github.com/bitcoin/bips/tree/master/bip-0039). 

It has no expectation to be used in production nor provide an optimal code. Only to provide a small sample of functional rust software

A few options are available : 

````
USAGE:
    b39wc [FLAGS] [OPTIONS] [seed]

FLAGS:
    -h, --help          Prints help information
    -s, --skip-count    Skips count of words number
    -V, --version       Prints version information

OPTIONS:
    -d, --dictionaries <d>    Dictionaries files to use (separated with coma)

ARGS:
    <seed>    The 24 words for seed generation

````