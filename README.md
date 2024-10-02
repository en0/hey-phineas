# Hey Phineas

When you don't know what to do today, ask Phineas.

## Quickstart

__Install__

```
cargo install --git https://github.com/en0/hey-phineas
```

__Run__

```
cat>>options.txt<<EOF
learn rust
play factorio
update arch
EOF

hey-phineas options.txt
```

## Why?

Because I want to learn rust and this was something to do.

## Future Enhancements

For this to really be useful, it needs to be a different program. But I would
settle for a way to pass the options in through a unix pipe.

Example:

```bash
echo -e "rewrite things in rust
Fix deadlock in factorio train system
Fix broken lid switch on arch install." | hey-phineas -f -
```

