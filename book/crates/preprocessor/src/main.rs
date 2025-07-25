//
// Copyright 2024, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

use std::env;
use std::fmt::Write;
use std::io;
use std::ops::RangeBounds;
use std::path::Path;

use clap::{Arg, Command};
use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use regex::{Captures, Regex};
use semver::{Version, VersionReq};

use x_preprocessor::{Step, Steps};

fn main() {
    let matches = Command::new("")
        .subcommand(Command::new("supports").arg(Arg::new("renderer").required(true)))
        .get_matches();

    let top_level_local_root = env::var("X_PREPROCESSOR_TOP_LEVEL_LOCAL_ROOT").unwrap();
    let last_step_rev = env::var("X_PREPROCESSOR_CODE_LAST_STEP_REV").unwrap();

    let steps = Steps::new_simple(top_level_local_root, &last_step_rev);

    let preprocessor = This {
        code_gh_root: env::var("X_PREPROCESSOR_CODE_GITHUB_ROOT").unwrap(),
        rustdoc_location: {
            let val = env::var("X_PREPROCESSOR_RUSTDOC_LOCATION_VALUE").unwrap();
            match env::var("X_PREPROCESSOR_RUSTDOC_LOCATION_KIND")
                .unwrap()
                .as_str()
            {
                "path" => RustdocLocation::Path(val),
                "url" => RustdocLocation::Url(val),
                _ => panic!(),
            }
        },
        manual_url: env::var("X_PREPROCESSOR_MANUAL_URL").unwrap(),
        steps,
    };

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        let renderer = sub_args.get_one::<String>("renderer").unwrap();
        assert!(preprocessor.supports_renderer(renderer));
    } else {
        handle_preprocessing(&preprocessor).unwrap();
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

struct This {
    code_gh_root: String,
    rustdoc_location: RustdocLocation,
    manual_url: String,
    steps: Steps,
}

enum RustdocLocation {
    Path(String),
    Url(String),
}

impl This {
    fn render_fragment_with_gh_link(&self, attrs: &str, link: &GitHubLink) -> String {
        let link_text = link.text();
        let url = self.gh_link_url(link, false);
        let fragment = link.fragment(&self.steps);

        let mut s = String::new();

        writeln!(&mut s, "<div class=\"fragment-with-gh-link\">").unwrap();

        writeln!(&mut s, "<div class=\"fragment-with-gh-link-link\">").unwrap();
        write!(&mut s, "<pre><code>").unwrap();
        write!(&mut s, "<a href=\"{url}\">{link_text}</a>").unwrap();
        write!(&mut s, "</code></pre>").unwrap();
        writeln!(&mut s, "").unwrap();
        writeln!(&mut s, "</div>").unwrap();

        writeln!(&mut s, "<div class=\"fragment-with-gh-link-fragment\">").unwrap();
        writeln!(&mut s, "").unwrap();
        writeln!(&mut s, "```{attrs}").unwrap();
        writeln!(&mut s, "{}", fragment).unwrap();
        writeln!(&mut s, "```").unwrap();
        writeln!(&mut s, "").unwrap();
        writeln!(&mut s, "</div>").unwrap();

        writeln!(&mut s, "</div>").unwrap();

        s
    }

    fn render_gh_link(&self, link: &GitHubLink) -> String {
        format!(
            "[{}]({})",
            link.text(),
            self.gh_link_url(
                link,
                self.steps.kind(link.step(), link.path()).is_directory()
            ),
        )
    }

    fn gh_link_url(&self, link: &GitHubLink, is_directory: bool) -> String {
        format!(
            "https://github.com/{}/{}/{}/{}",
            self.code_gh_root,
            if is_directory { "tree" } else { "blob" },
            self.steps.commit_hash(link.step()),
            link.url_suffix(),
        )
    }

    fn render_manual_link(&self, link: &ManualLink) -> String {
        link.render(&self.manual_url)
    }

    fn render_rustdoc_link(
        &self,
        chapter_path: &Path,
        config: &str,
        path: &str,
        text: &str,
    ) -> String {
        let target_suffix = match config {
            "root-task" => "",
            "microkit" => "-microkit",
            _ => panic!(),
        };

        let base = match &self.rustdoc_location {
            RustdocLocation::Path(path) => {
                let mut up = String::new();
                for _ in chapter_path.iter().skip(1) {
                    up.push_str("../");
                }
                format!("{up}{path}")
            }
            RustdocLocation::Url(url) => url.clone(),
        };

        format!("[{text}]({base}/{config}/aarch64-sel4{target_suffix}-unwind/doc/{path})",)
    }

    fn render_step_header(&self, step: &Step, text: &str) -> String {
        let long_rev = self.steps.commit_hash(step);
        let commit_link = self.step_commit_link(step);
        let mut s = String::new();
        // writeln!(&mut s, "## Step {step} [<i class=\"fa fa-github\"></i>]({commit_link}) `{short_rev}`").unwrap();
        // writeln!(&mut s, "## Step {step} `{short_rev}` [<i class=\"fa fa-github\"></i>]({commit_link})").unwrap();
        // writeln!(&mut s, "<span class=\"step-heading\">").unwrap();
        write!(&mut s, "## Step {step}{text}").unwrap();
        write!(&mut s, "&nbsp;").unwrap();
        write!(&mut s, "&nbsp;").unwrap();
        write!(&mut s, "&nbsp;").unwrap();
        write!(&mut s, "<span class=\"step-heading-clickable\" onclick=\"navigator.clipboard.writeText('{long_rev}')\">&nbsp;<i class=\"fa fa-copy\"></i>&nbsp;</span>").unwrap();
        write!(&mut s, "<a class=\"step-heading-clickable\" href=\"{commit_link}\">&nbsp;<i class=\"fa fa-github\"></i>&nbsp;</a>").unwrap();
        writeln!(&mut s, "").unwrap();
        // writeln!(&mut s, "</span>").unwrap();
        s
    }

    fn step_commit_link(&self, step: &Step) -> String {
        format!(
            "https://github.com/{}/commit/{}",
            self.code_gh_root,
            self.steps.commit_hash(step),
        )
    }
}

impl Preprocessor for This {
    fn name(&self) -> &str {
        "sel4-rust-tutorial"
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|section: &mut BookItem| {
            if let BookItem::Chapter(ref mut ch) = *section {
                {
                    let r = Regex::new("\\{\\{\\s*#fragment_with_gh_link\\s+\"(?<attrs>[^}]*)\"\\s+(?<link>.*?)\\s*\\}\\}").unwrap();
                    ch.content = r.replace_all(&ch.content, |captures: &Captures| {
                        self.render_fragment_with_gh_link(
                            captures.name("attrs").unwrap().as_str(),
                            &GitHubLink::parse(captures.name("link").unwrap().as_str()),
                        )
                    }).into_owned();
                }
                {
                    let r = Regex::new(r"\{\{\s*#gh_link\s+(?<link>.*?)\s*\}\}").unwrap();
                    ch.content = r.replace_all(&ch.content, |captures: &Captures| {
                        self.render_gh_link(
                            &GitHubLink::parse(captures.name("link").unwrap().as_str()),
                        )
                    }).into_owned();
                }
                {
                    let r = Regex::new(r"\{\{\s*#manual_link\s+(?<link>.*?)\s*\}\}").unwrap();
                    ch.content = r.replace_all(&ch.content, |captures: &Captures| {
                        self.render_manual_link(
                            &ManualLink::parse(captures.name("link").unwrap().as_str()).unwrap(),
                        )
                    }).into_owned();
                }
                {
                    let r = Regex::new(r"\{\{\s*#rustdoc_link\s+(?<config>.*?)\s+(?<path>.*?)\s+(?<text>.*?)\s*\}\}").unwrap();
                    ch.content = r.replace_all(&ch.content, |captures: &Captures| {
                        self.render_rustdoc_link(
                            ch.path.as_ref().unwrap(),
                            captures.name("config").unwrap().as_str(),
                            captures.name("path").unwrap().as_str(),
                            captures.name("text").unwrap().as_str(),
                        )
                    }).into_owned();
                }
                {
                    let r = Regex::new(r"\{\{\s*#step\s+(?<step>[0-9]+(\.[A-Z])?)(?<text>.*?)\}\}").unwrap();
                    ch.content = r.replace_all(&ch.content, |captures: &Captures| {
                        self.render_step_header(
                            &Step::parse(captures.name("step").unwrap().as_str()),
                            captures.name("text").unwrap().as_str(),
                        )
                    }).into_owned();
                }
                {
                    let r = Regex::new(r"\{\{\s*#rev_of_step_0(\s+(?<len>[0-9]+))?\s*\}\}").unwrap();
                    ch.content = r.replace_all(&ch.content, |captures: &Captures| {
                        let len = captures.name("len").map(|s| s.as_str().parse().unwrap()).unwrap_or(40);
                        format!("{}", &self.steps.commit_hash(&Step::parse("0"))[..len])
                    }).into_owned();
                }
                {
                    let r = Regex::new(r"\{\{\s*#rev_of_last_step(\s+(?<len>[0-9]+))?\s*\}\}").unwrap();
                    ch.content = r.replace_all(&ch.content, |captures: &Captures| {
                        let len = captures.name("len").map(|s| s.as_str().parse().unwrap()).unwrap_or(40);
                        format!("{}", &self.steps.commit_hash(self.steps.last_step())[..len])
                    }).into_owned();
                }
                {
                    let r = Regex::new(r"\{\{\s*#gh_repo_url\s*\}\}").unwrap();
                    ch.content = r.replace_all(&ch.content, |_captures: &Captures| {
                        format!("https://github.com/{}", self.code_gh_root)
                    }).into_owned();
                }
            }
        });

        Ok(book)
    }
}

#[derive(Debug)]
struct GitHubLink {
    text: Option<String>,
    step: Step,
    show_step: bool,
    hidden_path_part: Option<String>,
    visible_path_part: String,
    start: Option<usize>,
    end: Option<usize>,
}

impl GitHubLink {
    fn parse(s: &str) -> Self {
        let r = Regex::new(
            r"(?x)
            ^
            (\[(?<text>.*?)\]\s+)?
            (@(?<hide_step>-)?(?<step>.*?)\s+)?
            (\((?<hidden_path_part>.*?)\))?
            (?<visible_path_part>.*?)(:(?<start>\d+)(:(?<end>\d+))?)?
            $
        ",
        )
        .unwrap();
        let captures = r.captures(s).unwrap();
        let link = Self {
            text: captures.name("text").map(|m| m.as_str().to_owned()),
            step: captures
                .name("step")
                .map(|m| Step::parse(m.as_str()))
                .unwrap_or_default(),
            show_step: captures.name("step").is_some() && captures.name("hide_step").is_none(),
            hidden_path_part: captures
                .name("hidden_path_part")
                .map(|m| m.as_str().to_owned()),
            visible_path_part: captures
                .name("visible_path_part")
                .unwrap()
                .as_str()
                .to_owned(),
            start: captures.name("start").map(|m| m.as_str().parse().unwrap()),
            end: captures.name("end").map(|m| m.as_str().parse().unwrap()),
        };
        if link.show_step {
            assert!(!link.step.is_start());
        }
        if link.start.is_none() {
            assert!(link.end.is_none());
        }
        link
    }

    fn step(&self) -> &Step {
        &self.step
    }

    fn path(&self) -> String {
        let mut s = String::new();
        if let Some(hidden_path_part) = &self.hidden_path_part {
            write!(&mut s, "{hidden_path_part}").unwrap();
        }
        write!(&mut s, "{}", self.visible_path_part).unwrap();
        s
    }

    fn text(&self) -> String {
        if let Some(text) = &self.text {
            text.to_owned()
        } else {
            self.default_text()
        }
    }

    fn default_text(&self) -> String {
        let mut s = String::new();
        write!(&mut s, "{}", self.visible_path_part).unwrap();
        write!(&mut s, "{}", self.range_suffix()).unwrap();
        if self.show_step {
            write!(&mut s, " after {}", self.step()).unwrap();
        }
        s
    }

    fn url_suffix(&self) -> String {
        let mut s = self.path();
        if let Some(start) = &self.start {
            write!(&mut s, "#L{start}").unwrap();
            if let Some(end) = &self.end {
                write!(&mut s, "-L{end}").unwrap();
            }
        }
        s
    }

    fn fragment(&self, steps: &Steps) -> String {
        match (&self.start, &self.end) {
            (Some(start), Some(end)) => self.fragment_helper(steps, start..=end),
            (Some(start), None) => self.fragment_helper(steps, start..=start),
            (None, Some(end)) => self.fragment_helper(steps, ..=end),
            (None, None) => self.fragment_helper(steps, ..),
        }
    }

    fn fragment_helper(&self, steps: &Steps, bounds: impl RangeBounds<usize>) -> String {
        steps.fragment(self.step(), self.path(), bounds)
    }

    fn range_suffix(&self) -> String {
        let mut s = String::new();
        if let Some(start) = &self.start {
            write!(&mut s, ":{start}").unwrap();
            if let Some(end) = &self.end {
                write!(&mut s, ":{end}").unwrap();
            }
        }
        s
    }
}

#[derive(Debug)]
struct ManualLink {
    text: Option<String>,
    section: Option<String>,
    section_name: Option<String>,
}

impl ManualLink {
    fn parse(s: &str) -> Option<Self> {
        let r = Regex::new(
            r"(?x)
            ^
            (
                \[
                    (?<text>.*?)
                \]
                (\s+|$)
            )?
            (
                \#
                (?<section>.*?)
                (\s+|$)
            )?
            (
                \(
                    (?<section_name>.*?)
                \)
                (\s+|$)
            )?
            $
        ",
        )
        .unwrap();
        r.captures(s).map(|captures| Self {
            text: captures.name("text").map(|m| m.as_str().to_owned()),
            section: captures.name("section").map(|m| m.as_str().to_owned()),
            section_name: captures.name("section_name").map(|m| m.as_str().to_owned()),
        })
    }

    fn render(&self, url: &str) -> String {
        let text = self.text.clone().unwrap_or_else(|| {
            let mut s = format!("seL4 Reference Manual");
            if let Some(section) = &self.section {
                write!(&mut s, " § {section}").unwrap();
                if let Some(section_name) = &self.section_name {
                    write!(&mut s, " ({section_name})").unwrap();
                }
            }
            s
        });
        let fragment = if let Some(section) = &self.section {
            let ty = match section.chars().filter(|c| *c == '.').count() {
                0 => "chapter",
                1 => "section",
                2 => "subsection",
                3 => "subsubsection",
                _ => panic!(),
            };
            format!("#{ty}.{section}")
        } else {
            String::new()
        };
        format!("[{text}]({url}{fragment})")
    }
}
