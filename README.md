# logt

Measuring the execution time of a longer running command is perfectly solved by the builtin `time` that most shells provide. But in case you wanted to measure the individual steps a command takes, you might want to log the time of each outputted line. That's what **logt** is for.

**logt** executes a wrapped CLI command and annotates each output line with a timestamp like this:

<table>
<tr>
<td>

**Original command**

</td>
<td>

**Wrapped command**

</td>
</tr>
<tr>
<td>

```
$ ./test.sh
Message 1
Message 2
Message 3
Message 4
```

</td>
<td>

```
$ logt ./test.sh 
[2025-03-09 21:00:57.385610431 +01:00] Message 1
[2025-03-09 21:00:57.385670051 +01:00] Message 2
[2025-03-09 21:00:57.385677761 +01:00] Message 3
[2025-03-09 21:00:57.385682501 +01:00] Message 4
```

</td>
</tr>
</table>

## Installation
Execute `cargo install logt`.

## Synopsis

```sh
logt [FLAGS] [--] PROGRAM [ARGS]
```

| Flag | Meaning |
|------|---------|
|  -r  | Instead of the absolute time, show the duration since the start of the program. |
|  -s  | For each line, show if it was emitted on stderr or stdout. |
|  -h  | Show a more detailed help. |

The `--` flag can be used to mark the end of **logt** flags. This would be useful in the unfortunate case that your program path or name starts with a hyphen, to distinguish it from a CLI option.

`PROGRAM` can be a path (absolute/relative) to an executable or just a file name, if the executable is installed in one of the directories in the [`PATH` variable](https://en.wikipedia.org/wiki/PATH_(variable)).

### Example

```
$ logt -rs ./test.sh
[stdout +0.000873863s] Message 1
[stderr +0.000909503s] Message 2
[stdout +0.000918523s] Message 3
[stderr +0.000922283s] Message 4
```

## Shortcomings
* Since `logt` is not a shell builtin, it can not directly execute bracket commands. `time (a && b)` would therefore very roughly translate to something like `logt sh -c 'a && b'`.