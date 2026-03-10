# .contextus/

This directory documents the contextus layer hierarchy.

`contextus` is **Layer 0** — the root of the hierarchy. It has no parent layer.

## Layer Chain

```
contextus  (L0, this repo)  ← root, no parent
    └── contextus-claude    (L1)
        ├── contextus-claude-sh-dev  (L2a)
        └── contextus-claude-kw     (L2b)
```

Consumer projects place their upstream layer repos here:
```
my-project/
└── .contextus/
    ├── contextus-claude      → upstream L1
    └── contextus-claude-sh-dev → upstream L2a
```
