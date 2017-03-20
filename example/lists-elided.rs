// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cmp;
use std::iter::Peekable;

use syntax::codemap::{self, CodeMap, BytePos};

use Indent;
use comment::{FindUncommented, rewrite_comment, find_comment_end};
use config::Config;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
/// Formatting tactic for lists. This will be cast down to a
/// DefinitiveListTactic depending on the number and length of the items and
/// their comments.
pub enum ListTactic {
    // One item per row.
    Vertical,
    // All items on one row.
    Horizontal,
    // Try Horizontal layout, if that fails then vertical.
    HorizontalVertical,
    // HorizontalVertical with a soft limit of n characters.
    LimitedHorizontalVertical(usize),
    // Pack as many items as possible per row over (possibly) many rows.
    Mixed,
}

impl_enum_decodable!(ListTactic, Vertical, Horizontal, HorizontalVertical, Mixed);

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum SeparatorTactic {
    Always,
    Never,
    Vertical,
}

impl_enum_decodable!(SeparatorTactic, Always, Never, Vertical);

impl SeparatorTactic {
    pub fn from_bool(b: bool) -> SeparatorTactic {
        if b {
            SeparatorTactic::Always
        } else {
            SeparatorTactic::Never
        }
    }
}

pub struct ListFormatting<'a> {
    pub tactic: DefinitiveListTactic,
    pub separator: &'a str,
    pub trailing_separator: SeparatorTactic,
    pub indent: Indent,
    pub width: usize,
    // Non-expressions, e.g. items, will have a new line at the end of the list.
    // Important for comment styles.
    pub ends_with_newline: bool,
    pub config: &'a Config,
}

pub fn format_fn_args<I>(items: I, width: usize, offset: Indent, config: &Config) -> Option<String>
where
    I: Iterator<Item = ListItem>,
{
    // ...
}

pub fn format_item_list<I>(
    items: I,
    width: usize,
    offset: Indent,
    config: &Config,
) -> Option<String>
where
    I: Iterator<Item = ListItem>,
{
    list_helper(items, width, offset, config, ListTactic::HorizontalVertical)
}

pub fn list_helper<I>(
    items: I,
    width: usize,
    offset: Indent,
    config: &Config,
    tactic: ListTactic,
) -> Option<String>
where
    I: Iterator<Item = ListItem>,
{
    let item_vec: Vec<_> = items.collect();
    let tactic = definitive_tactic(&item_vec, tactic, width);
    let fmt = ListFormatting {
        tactic: tactic,
        separator: ",",
        trailing_separator: SeparatorTactic::Never,
        indent: offset,
        width: width,
        ends_with_newline: false,
        config: config,
    };

    write_list(&item_vec, &fmt)
}

impl AsRef<ListItem> for ListItem {
    fn as_ref(&self) -> &ListItem {
        self
    }
}

pub struct ListItem {
    // None for comments mean that they are not present.
    pub pre_comment: Option<String>,
    // Item should include attributes and doc comments. None indicates a failed
    // rewrite.
    pub item: Option<String>,
    pub post_comment: Option<String>,
    // Whether there is extra whitespace before this item.
    pub new_lines: bool,
}

impl ListItem {
    pub fn is_multiline(&self) -> bool {
        // ...
    }

    pub fn has_line_pre_comment(&self) -> bool {
        self.pre_comment.as_ref().map_or(false, |comment| comment.starts_with("//"))
    }

    pub fn from_str<S: Into<String>>(s: S) -> ListItem {
        ListItem {
            pre_comment: None,
            item: Some(s.into()),
            post_comment: None,
            new_lines: false,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
/// The definitive formatting tactic for lists.
pub enum DefinitiveListTactic {
    Vertical,
    Horizontal,
    Mixed,
}

pub fn definitive_tactic<I, T>(items: I, tactic: ListTactic, width: usize) -> DefinitiveListTactic
where
    I: IntoIterator<Item = T> + Clone,
    T: AsRef<ListItem>,
{
    let pre_line_comments =
        items.clone().into_iter().any(|item| item.as_ref().has_line_pre_comment());

    let limit = match tactic {
        _ if pre_line_comments => return DefinitiveListTactic::Vertical,
        ListTactic::Mixed => return DefinitiveListTactic::Mixed,
        ListTactic::Horizontal => return DefinitiveListTactic::Horizontal,
        ListTactic::Vertical => return DefinitiveListTactic::Vertical,
        ListTactic::LimitedHorizontalVertical(limit) => ::std::cmp::min(width, limit),
        ListTactic::HorizontalVertical => width,
    };

    let (sep_count, total_width) = calculate_width(items.clone());
    let sep_len = ", ".len(); // FIXME: make more generic?
    let total_sep_len = sep_len * sep_count.checked_sub(1).unwrap_or(0);
    let real_total = total_width + total_sep_len;
}

// Format a list of commented items into a string.
// TODO: add unit tests
pub fn write_list<I, T>(items: I, formatting: &ListFormatting) -> Option<String>
where
    I: IntoIterator<Item = T>,
    T: AsRef<ListItem>,
{
    let tactic = formatting.tactic;
    let sep_len = formatting.separator.len();

    // Now that we know how we will layout, we can decide for sure if there
    // will be a trailing separator.
    let trailing_separator = needs_trailing_separator(formatting.trailing_separator, tactic);
    let mut result = String::new();
    let mut iter = items.into_iter().enumerate().peekable();

    let mut line_len = 0;
    let indent_str = &formatting.indent.to_string(formatting.config);
    while let Some((i, item)) = iter.next() {
        let item = item.as_ref();
        let inner_item = try_opt!(item.item.as_ref());
        let first = i == 0;
        let last = iter.peek().is_none();
        let separate = !last || trailing_separator;
        let item_sep_len = if separate { sep_len } else { 0 };

        // Item string may be multi-line. Its length (used for block comment alignment)
        // Should be only the length of the last line.
        let item_last_line = if item.is_multiline() {
            inner_item.lines().last().unwrap_or("")
        } else {
            inner_item.as_ref()
        };
        let mut item_last_line_width = item_last_line.len() + item_sep_len;
        if item_last_line.starts_with(indent_str) {
            item_last_line_width -= indent_str.len();
        }

        match tactic {
            DefinitiveListTactic::Horizontal if !first => {
                result.push(' ');
            }
            DefinitiveListTactic::Vertical if !first => {
                result.push('\n');
                result.push_str(indent_str);
            }
            DefinitiveListTactic::Mixed => {
                let total_width = total_item_width(item) + item_sep_len;

                // 1 is space between separator and item.
                if line_len > 0 && line_len + 1 + total_width > formatting.width {
                    result.push('\n');
                    result.push_str(indent_str);
                    line_len = 0;
                }

                if line_len > 0 {
                    result.push(' ');
                    line_len += 1;
                }

                line_len += total_width;
            }
            _ => {}
        }

        result.push_str(&inner_item[..]);

        if separate {
            result.push_str(formatting.separator);
        }

        if !last && tactic == DefinitiveListTactic::Vertical && item.new_lines {
            result.push('\n');
        }
    }

    Some(result)
}

pub struct ListItems<'a, I, F1, F2, F3>
where
    I: Iterator,
{
    codemap: &'a CodeMap,
    inner: Peekable<I>,
    get_lo: F1,
    get_hi: F2,
    get_item_string: F3,
    prev_span_end: BytePos,
    next_span_start: BytePos,
    terminator: &'a str,
}

impl<'a, T, I, F1, F2, F3> Iterator for ListItems<'a, I, F1, F2, F3>
where
    I: Iterator<Item = T>,
    F1: Fn(&T) -> BytePos,
    F2: Fn(&T) -> BytePos,
    F3: Fn(&T) -> Option<String>,
{
    type Item = ListItem;

    fn next(&mut self) -> Option<Self::Item> {
        // ...
    }
}

// Creates an iterator over a list's items with associated comments.
pub fn itemize_list<'a, T, I, F1, F2, F3>(
    codemap: &'a CodeMap,
    inner: I,
    terminator: &'a str,
    get_lo: F1,
    get_hi: F2,
    get_item_string: F3,
    prev_span_end: BytePos,
    next_span_start: BytePos,
) -> ListItems<'a, I, F1, F2, F3>
where
    I: Iterator<Item = T>,
    F1: Fn(&T) -> BytePos,
    F2: Fn(&T) -> BytePos,
    F3: Fn(&T) -> Option<String>,
{
    ListItems {
        codemap: codemap,
        inner: inner.peekable(),
        get_lo: get_lo,
        get_hi: get_hi,
        get_item_string: get_item_string,
        prev_span_end: prev_span_end,
        next_span_start: next_span_start,
        terminator: terminator,
    }
}

fn needs_trailing_separator(
    separator_tactic: SeparatorTactic,
    list_tactic: DefinitiveListTactic,
) -> bool {
    match separator_tactic {
        SeparatorTactic::Always => true,
        SeparatorTactic::Vertical => list_tactic == DefinitiveListTactic::Vertical,
        SeparatorTactic::Never => false,
    }
}

/// Returns the count and total width of the list items.
fn calculate_width<I, T>(items: I) -> (usize, usize)
where
    I: IntoIterator<Item = T>,
    T: AsRef<ListItem>,
{
    // ...
}

fn total_item_width(item: &ListItem) -> usize {
    // ...
}

fn comment_len(comment: Option<&str>) -> usize {
    match comment {
        Some(s) => {
            let text_len = s.trim().len();
            if text_len > 0 {
                // We'll put " /*" before and " */" after inline comments.
                text_len + 6
            } else {
                text_len
            }
        }
        None => 0,
    }
}
