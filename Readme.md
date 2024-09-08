# Brainfuck
You can run any brainfuck program by compiling and running the interpreter with the brainfuck program file as argument. Optimizations are usually not needed in programs that depend on input or don't run infinitely, as those programs usually finish within microseconds or milliseconds.

# Example performance
## Brainfuck.org
|example|performance|
|-|-|
|e| infinite|
|factorial| infinite|
|factorial2| infinite, but faster than factorial|
|fib| infinite|
|golden| infinite|
|impeccable| infinite|
|random| infinite|
|squares2| infinite|
|thuermorse| infinite|
|rot13| on input|
|squares| 4.525106ms|
|400genuine| 4.200764ms|
|dgenuine| 3.892332ms|
|sierpinski| 486.24µs|
|hello world| 113.5µs|
|jahb| 65.358µs|
|adding two values| 46.009µs|

## My own
|example|performance|
|-|-|
|prime generator| 7.370644ms|
