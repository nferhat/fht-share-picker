# fht-share-picker

A simple application that opens up when [fht-compositor](https://github.com/nferhat/fht-compositor) needs to select an output to start a screencast session.

## Why in a separate repo?

This used to live inside the [fht-compositor](https://github.com/nferhat/fht-compositor) tree as a workspace package, but due to dependency conflicts (with the `web-sys` package), cargo can't resolve this specific package.

So, instead of messing with keeping a fork for this, I'd rather keep it in another repository.

## Usage

```bash
fht-share-picker <program-name>
```
