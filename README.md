# GTI

GTI is a command-line tool that aims to poke fun at people who misspell Git. It's inspired by `sl` and `gti` from [rwos](https://github.com/rwos/gti). This project is a personal endeavor and is meant for entertainment purposes.

## What does it do?

GTI is a troll tool that provides a command-line interface that mimics the functionality of Git but with a twist. It's designed to help people who misspell Git commands.

## How to Build and Install

To build GTI, you can use the following command:
```
cargo build --release
```

And to install GTI, you can use the following command:
```
cargo install --path .
```

### Autocompletions

For autocompletions in Bash or Zsh the completions need to be installed into the right directories like:

For <b>Bash</b>
```
cp src/completions/gti.bash ~/.local/share/bash-completion/completions/gti
```

For <b>Zsh</b>
```
cp src/completions/gti.zsh ~/.local/share/zsh/site-functions/gti
```
