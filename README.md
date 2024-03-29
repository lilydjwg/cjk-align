This library provides a wrapper struct `CJKAlign` to align CJK and emoji characters
correctly on terminals. Despite its name, it works for other Unicode characters too
as supported by the unicode-width crate.

```rust
use cjk_align::CJKAlign;

assert_eq!(format!("{:6}", CJKAlign("你好")), "你好  ");
assert_eq!(format!("{:>6}", CJKAlign("你好")), "  你好");
assert_eq!(format!("{:^6}", CJKAlign("你好")), " 你好 ");
assert_eq!(format!("{:^7}", CJKAlign("你好")), "  你好 ");
```

To treat East Asian ambiguous width characters as double width, use
`CJKAlignWide` instead:

```rust
use cjk_align::{CJKAlign, CJKAlignWide};

assert_eq!(format!("{:8}", CJKAlign("“……”")), "“……”    ");
assert_eq!(format!("{:8}", CJKAlignWide("“……”")), "“……”");
```
