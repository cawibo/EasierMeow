# EasierMeow
My first time writing code in Rust. Preq. knowledge include, and is mostly limited to, reading the documentation up-and-until structs. It has been quite the experience having to wade through the last couple of years' heated discussions and spontaneous deprecations. I set out to make at least something; and I did.

## What is EasierMeow
EasierMeow was made to make 'quick Kattis sessions' easier to manage. In its current state there is functinality for:
* Downloading and setting up testcases for a given problem.
* Automatic testing of downloaded testcases.

## Known Issues
* Testing currently supports only Python.
* Creates Python main-file either way.
* Will fail if problem name is different from the problem url. 

## How to use
* ``mjau init <problem>`` to download a problem.
* ``mjau test`` to run all testcases.

## How to set up [Windows only]
1. Create .exe-file from project using Cargo.
2. Set PATH to .exe-file

