Z Standard File System(ZSFS)
============================

A file system built on Zstandard compression algorithm.

```
| 4K head | 4K * n data block |
```

a linked list take 4k as a block, and the first 4k is the head, the second 4k is the data, the third 4k is the next block's head, and so on.