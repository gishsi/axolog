# axolog | Learning project

Have you ever looked through a Minecraft log file? Let's be honest here, it's a pain. Especially when making a modded server, with hundreds of log entries, all belonging to different Minecraft mods, types, and threads, it is no easy task finding the cause of why your friend texted you at 3 AM, saying the server crushed right when they were mining diamonds. It's always right when they were mining diamonds. What a coincidence.

Regardless of your friend's honesty, you gotta fix it. You just gotta do it. This utility tool aims to make the process of recognizing the root of the server's failure much easier by converting the unstructured .log file into a more structured JSON equivalent. 

# Usage

Usage: axolog.exe [OPTIONS] --path-to-file <PATH_TO_FILE>

Options:

      --path-to-file <PATH_TO_FILE>  Path to the log file
      --path-to-save <PATH_TO_SAVE>  Path to save the file with the structured data at. If empty, the formatted output in JSON form gets written to the console [default: ]
      --log-type <LOG_TYPE>          [default: all] [possible values: all, info, debug, warn, error, fatal, main, main-info, main-debug, main-warn, main-error]
      -h, --help                         Print help
      -V, --version                      Print version

Use `cargo doc --open` for more documentation

# Supported features

Just the basics: 
1. JSON output for the formatted logs
2. You can get logs from a specific type of a log: main, info, debug, warn, error, fatal, main-info, main-debug, main-error, main-warn (I will add more as I find them, but some are generated per server without a way of predicting them, e.g. pool-3-thread-1/DEBUG)

# Roadmap

- better visual of the output log
- other filetypes for output, e.g. CVS
