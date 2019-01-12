# Introduction

Read from stdin, count all characters, English words and Chinese characters.

Used in shell:

```bash
echo "Hello! 你好！" | sumch
```

Output:

```
all: 10
en: 1
cn: 2
```

Used with pandoc:

```bash
pandoc test.md -t plain | sumch
```

Used in vim:

```vimscript
nnoremap <LocalLeader>t :execute "! pandoc " . expand("%:p") . " -t plain \| sumch" <CR>
```

# Install

```bash
cargo build --release
```
