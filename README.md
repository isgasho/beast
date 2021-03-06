# **beast** - **be**nchmark **a**nalysis and **s**ummary **t**ool

![ci](https://github.com/bjob/beast/workflows/ci-chain/badge.svg) ![ci](https://github.com/bjob/beast/workflows/cd-chain/badge.svg) ![Latest Release](https://img.shields.io/github/release/bjob/beast.svg?style=flat)

``beast`` is a simple tool to execute and post-process a set of google benchmarks. The basic feature is plotting multiple benchmark results in one single plot (powered by `plotly`). It is also possible to configure access to a mongodb database and push results to it.

## **Installation**

Download the latest release from:

[![Download beast](https://img.shields.io/badge/dynamic/json.svg?label=download&url=https://api.github.com/repos/bjob/beast/releases/latest&query=$.assets[0].name&style=for-the-badge)](https://github.com/bjob/beast/releases/latest)

and install the ``.deb`` package via ``sudo dpkg -i``.

## **Usage example**

``beast`` will search for executables with a predefined regular expression pattern in your current working directory as a default. Check ``beast --help`` for an overview about possible options. We will use the C++ example from the repo directory to show the basic functionality (of course you will need to clone the repo for that):

``cd`` to the ``example_benchmark`` directory and call:

```bash
mkdir build
cd build
cmake ..
cmake --build . --target all
```

you will get some small benchmark executables which can be plotted with ``beast``:

![beast_on_examples](doc/beast_on_examples.gif)
