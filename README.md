
# Kafmap

Analysis the graph of kafka topic consumers and producers running on an kubernetes cluster.

## Requirements

Applications consuming or producing from a topic are assumed to follow a standard configuration convention that they are entirely specified by their associated ConfigMap with the following keys:

1. APPLICATION_ID
2. INPUT_TOPIC
3. OUTPUT_TOPIC

These config values are used to plot a graph of topics and the applications that produce and consume data to and from them.

## Usage

```
kafmap 1.0

USAGE:
    kafmap <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    graph    
    help     Prints this message or the help of the given subcommand(s)
```
