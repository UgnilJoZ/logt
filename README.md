# logt

Measuring the execution time of a longer running command is perfectly solved by the bash builtin `time`. But in case you wanted to measure the individual steps a program takes, you might want to log the time of each outputted line. That's what **logt** is for.

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

## Flags

| Flag | Meaning |
|------|---------|
|  -r  | Instead of the absolute time, show the duration since the start of the program. |
|  -s  | For each line, show if it was emitted on stderr or stdout. |
|  -h  | Show a more detailed help. |

## Shortcomings
* Since `logt` is not a shell builtin, it can not directly execute bracket commands. `time (a && b)` would therefore very roughly translate to something like `logt sh -c 'a && b'`.