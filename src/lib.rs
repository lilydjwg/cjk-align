//! This library provides a wrapper struct [CJKAlign] to align CJK and emoji characters
//! correctly on terminals. Despite its name, it works for other Unicode characters too
//! as supported by the unicode-width crate.
//!
//! ```
//! use cjk_align::CJKAlign;
//!
//! assert_eq!(format!("{:6}", CJKAlign("你好")), "你好  ");
//! assert_eq!(format!("{:>6}", CJKAlign("你好")), "  你好");
//! assert_eq!(format!("{:^6}", CJKAlign("你好")), " 你好 ");
//! assert_eq!(format!("{:^7}", CJKAlign("你好")), "  你好 ");
//! ```
//!
//! To treat East Asian ambiguous width characters as double width, use
//! [CJKAlignWide] instead:
//!
//! ```
//! use cjk_align::{CJKAlign, CJKAlignWide};
//!
//! assert_eq!(format!("{:8}", CJKAlign("“……”")), "“……”    ");
//! assert_eq!(format!("{:8}", CJKAlignWide("“……”")), "“……”");
//! ```

use std::fmt::{Display, Formatter, Error, Alignment};

use unicode_width::UnicodeWidthStr;

pub struct CJKAlign<'a>(pub &'a str);
impl<'a> Display for CJKAlign<'a> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    StringWidth::fmt(self, f)
  }
}

pub struct CJKAlignWide<'a>(pub &'a str);
impl<'a> Display for CJKAlignWide<'a> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    StringWidth::fmt(self, f)
  }
}

trait StringWidth {
  fn width(&self) -> usize;
  fn str(&self) -> &str;

  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    let w = f.width().unwrap_or(0);
    if w == 0 {
      write!(f, "{}", self.str())
    } else {
      let self_width = self.width();
      let pad = w.saturating_sub(self_width);
      match f.align() {
        Some(Alignment::Left) | None => write!(f, "{}{}", self.str(), " ".repeat(pad)),
        Some(Alignment::Right) => write!(f, "{}{}", " ".repeat(pad), self.str()),
        Some(Alignment::Center) => {
          write!(f, "{}{}{}", " ".repeat(pad - pad / 2), self.str(), " ".repeat(pad / 2))
        },
      }
    }
  }
}

impl<'a> StringWidth for CJKAlign<'a> {
  fn width(&self) -> usize {
    UnicodeWidthStr::width(self.0)
  }

  fn str(&self) -> &str {
    self.0
  }
}

impl<'a> StringWidth for CJKAlignWide<'a> {
  fn width(&self) -> usize {
    UnicodeWidthStr::width_cjk(self.0)
  }

  fn str(&self) -> &str {
    self.0
  }
}
