// Copyright 2016 The Rustw Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Syntax highlighting.

use std::collections::HashMap;
use std::fmt::Display;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::str;
use std::time::Instant;

use rustdoc::html::highlight::{self, Classifier, Class};
use span;
use syntax::parse;
use syntax::parse::lexer::{self, TokenAndSpan};
use syntax::codemap::{CodeMap, Loc};

use analysis::AnalysisHost;

type Span = span::Span<span::ZeroIndexed>;

pub fn highlight<'a>(
    analysis: &'a AnalysisHost, project_path: &'a Path, file_name: String, file_text: String,
) -> String {
    debug!("highlight `{}` in `{}`", file_text, file_name);
    let sess = parse::ParseSess::new();
    let fm = sess.codemap().new_filemap(file_name.clone(), None, file_text);

    let mut out = Highlighter::new(analysis, project_path, sess.codemap());

    let t_start = Instant::now();

    let mut classifier = Classifier::new(lexer::StringReader::new(&sess.span_diagnostic, fm),
                                         sess.codemap());
    classifier.write_source(&mut out).unwrap();

    let time = t_start.elapsed();

    String::from_utf8_lossy(&out.buf).into_owned()
}

pub fn custom_highlight<H: highlight::Writer + GetBuf>(
    file_name: String, file_text: String, highlighter: &mut H,
) -> String {
    debug!("custom_highlight `{}` in `{}`", file_text, file_name);
    let sess = parse::ParseSess::new();
    let fm = sess.codemap().new_filemap(file_name.clone(), None, file_text);

    let mut classifier = Classifier::new(lexer::StringReader::new(&sess.span_diagnostic, fm),
                                         sess.codemap());
    classifier.write_source(highlighter).unwrap();

    String::from_utf8_lossy(highlighter.get_buf()).into_owned()
}

struct Highlighter<'a> {
    buf: Vec<u8>,
    analysis: &'a AnalysisHost,
    codemap: &'a CodeMap,
    project_path: &'a Path,
    path_cache: HashMap<String, PathBuf>,
}

impl<'a> Highlighter<'a> {
    fn new(
        analysis: &'a AnalysisHost, project_path: &'a Path, codemap: &'a CodeMap,
    ) -> Highlighter<'a> {
        Highlighter {
            buf: vec![],
            analysis: analysis,
            codemap: codemap,
            project_path: project_path,
            path_cache: HashMap::new(),
        }
    }

    fn get_link(&self, span: &Span) -> Option<String> {
        self.analysis.goto_def(span).ok().and_then(|def_span| {
            if span == &def_span {
                None
            } else {
                Some(loc_for_span(&def_span, self.project_path))
            }
        })
    }

    fn span_from_locs(&mut self, lo: &Loc, hi: &Loc) -> Span {
        let file_path = self.path_cache.entry(lo.file.name.clone()).or_insert_with(|| {
            Path::new(&lo.file.name).canonicalize().unwrap()
        });
        Span::new(span::Row::new_one_indexed(lo.line as u32).zero_indexed(),
                  span::Row::new_one_indexed(hi.line as u32).zero_indexed(),
                  span::Column::new_zero_indexed(lo.col.0 as u32),
                  span::Column::new_zero_indexed(hi.col.0 as u32),
                  file_path.clone())
    }
}

pub fn write_span(
    buf: &mut Vec<u8>,
    klass: Class,
    extra_class: Option<String>,
    text: String,
    src_link: bool,
    extra: HashMap<String, String>,
) -> io::Result<()> {
    write!(buf, "<span class='{}", klass.rustdoc_class())?;
    if let Some(s) = extra_class {
        write!(buf, " {}", s)?;
    }
    if src_link {
        write!(buf, " src_link")?;
    }
    write!(buf, "'")?;
    for (k, v) in &extra {
        // Some values need escaping.
        if k == "title" {
            write!(buf, " {}='", k)?;
            for c in v.chars() {
                push_char(buf, c)?;
            }
            write!(buf, "'")?;
        } else {
            write!(buf, " {}='{}'", k, v)?;
        }
    }
    write!(buf, ">{}</span>", text)
}

fn push_char(buf: &mut Vec<u8>, c: char) -> io::Result<()> {
    match c {
        '>' => write!(buf, "&gt;"),
        '<' => write!(buf, "&lt;"),
        '&' => write!(buf, "&amp;"),
        '\'' => write!(buf, "&#39;"),
        '"' => write!(buf, "&quot;"),
        '\n' => write!(buf, "<br>"),
        _ => write!(buf, "{}", c),
    }
}

fn loc_for_span(span: &Span, project_path: &Path) -> String {
    let file_name = Path::new(&span.file).strip_prefix(project_path)
                                         .ok()
                                         .unwrap_or(&span.file)
                                         .to_str()
                                         .unwrap();
    format!("{}:{}:{}:{}:{}",
            file_name,
            span.range.row_start.one_indexed().0,
            span.range.col_start.one_indexed().0,
            span.range.row_end.one_indexed().0,
            span.range.col_end.one_indexed().0)
}


macro_rules! maybe_insert {
    ($h: expr, $k: expr, $v: expr) => {
        if let Some(v) = $v {
            $h.insert($k.to_owned(), v);
        }
    }
}

impl<'a> highlight::Writer for Highlighter<'a> {
    fn enter_span(&mut self, klass: Class) -> io::Result<()> {
        write!(self.buf, "<span class='{}'>", klass.rustdoc_class())
    }

    fn exit_span(&mut self) -> io::Result<()> {
        write!(self.buf, "</span>")
    }

    fn string<T: Display>(
        &mut self, text: T, klass: Class, tas: Option<&TokenAndSpan>,
    ) -> io::Result<()> {
        let text = text.to_string();

        match klass {
            Class::None => write!(self.buf, "{}", text),
            Class::Ident => {
                match tas {
                    Some(t) => {
                        let lo = self.codemap.lookup_char_pos(t.sp.lo);
                        let hi = self.codemap.lookup_char_pos(t.sp.hi);
                        let span = &self.span_from_locs(&lo, &hi);
                        let title = match (ty, docs) {
                            (Some(t), Some(d)) => Some(format!("{}\n\n{}", t, d)),
                            (Some(t), _) => Some(t),
                            (_, Some(d)) => Some(d),
                            (None, None) => None,
                        };
                        let mut link = self.get_link(span);
                        let doc_link = self.analysis.doc_url(span).ok();
                        let src_link = self.analysis.src_url(span).ok();

                    }
                }
            }
            Class::Op if text == "*" => {
                match tas {
                    Some(t) => {
                        let lo = self.codemap.lookup_char_pos(t.sp.lo);
                        let hi = self.codemap.lookup_char_pos(t.sp.hi);
                        let span = &self.span_from_locs(&lo, &hi);
                        let mut extra = HashMap::new();
                        extra.insert("location".to_owned(), format!("{}:{}", lo.line, lo.col.0 + 1));
                        maybe_insert!(extra, "title", self.analysis.show_type(span).ok());
                        let css_class = Some(" glob".to_owned());

                        write_span(&mut self.buf, Class::Op, css_class, text, false, extra)
                    }
                    None => write_span(&mut self.buf, Class::Op, None, text, false, HashMap::new()),
                }
            }
            klass => write_span(&mut self.buf, klass, None, text, false, HashMap::new()),
        }
    }
}

impl BasicHighlighter {
    pub fn new() -> BasicHighlighter {
        BasicHighlighter {
            buf: vec![],
            spans: vec![],
        }
    }

    pub fn span(
        &mut self, start: u32, end: u32, klass: String, id: String, def_span: Option<Span>,
    ) {
        self.spans.push(SpanSpan {
            start_byte: start,
            end_byte: end,
            klass: klass,
            id: id,
            def_span: def_span,
        });
    }
}

impl highlight::Writer for BasicHighlighter {
    fn enter_span(&mut self, klass: Class) -> io::Result<()> {
        write!(self.buf, "<span class='{}'>", klass.rustdoc_class())
    }

    fn exit_span(&mut self) -> io::Result<()> {
        write!(self.buf, "</span>")
    }

    fn string<T: Display>(
        &mut self, text: T, klass: Class, tas: Option<&TokenAndSpan>,
    ) -> io::Result<()> {
        let text = text.to_string();

        let mut extra_class = None;
        let mut id = None;
        let mut link = None;
        if let Some(tas) = tas {
            let lo = tas.sp.lo.0;
            let hi = tas.sp.hi.0;
            for s in &self.spans {
                if s.start_byte == lo && s.end_byte == hi {
                    extra_class = Some(s.klass.clone());
                    id = Some(s.id.clone());
                    link = s.def_span.as_ref().map(|sp| loc_for_span(sp, &Path::new("")));
                }
            }
        }

        let has_link = link.is_some();
        let mut extra = HashMap::new();
        maybe_insert!(extra, "id", id);
        maybe_insert!(extra, "link", link);
        write_span(&mut self.buf, klass, extra_class, text, has_link, extra)
    }
}
