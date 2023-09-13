# [WIP] axolog | Learning project

> Note: This is very much still a work in progress. I am a new Rust programmer and I will refine this code as I learn more about both this programming language and Minecraft logs.


**Motivation**: when creating a Minecraft modded server I found looking through Minecraft crash logs a huge pain. This utility tool aims to make this process easier.

# Usage

Usage: axolog.exe [OPTIONS] --path-to-file <PATH_TO_FILE>

Options:

      --path-to-file <PATH_TO_FILE>  Path to the log file
      --path-to-save <PATH_TO_SAVE>  Path to save the file with the structured data at. If empty, the formatted output in JSON form gets written to the console [default: ]
      --log-type <LOG_TYPE>          [default: all] [possible values: all, info, debug, warn, error, fatal, main, main-info, main-debug, main-warn, main-error]
      -h, --help                         Print help
      -V, --version                      Print version

# Supported features

Just the basics: 
1. JSON output for the formatted logs
2. You can get logs from a specific type of a log: main, info, debug, warn, error, fatal, main-info, main-debug, main-error, main-warn (I will add more as I see them, but some are generated per server without a way of predicting them that's clear to me now, e.g. pool-3-thread-1/DEBUG)

# Roadmap

- better visual of the output log when the description has some newlines
- more types
- other filetypes for output, e.g. CVS
