# EasierMeow

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

