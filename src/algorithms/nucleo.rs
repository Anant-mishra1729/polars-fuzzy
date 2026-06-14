use nucleo_matcher::pattern::{CaseMatching, Normalization, Pattern};
use nucleo_matcher::{Config, Matcher, Utf32Str};

pub struct NucleoMatcher {
    matcher: Matcher,
    buf: Vec<char>,
}

impl NucleoMatcher {
    pub fn new() -> Self {
        Self {
            matcher: Matcher::new(Config::DEFAULT),
            buf: Vec::with_capacity(256),
        }
    }

    pub fn compile_pattern(query: &str, case_sensitive: bool, normalize: bool) -> Pattern {
        Pattern::parse(
            query,
            if case_sensitive {
                CaseMatching::Respect
            } else {
                CaseMatching::Ignore
            },
            if normalize {
                Normalization::Smart
            } else {
                Normalization::Never
            },
        )
    }

    pub fn score(&mut self, haystack: &str, pattern: &Pattern) -> Option<u32> {
        let haystack_utf32 = Utf32Str::new(haystack, &mut self.buf);
        pattern.score(haystack_utf32, &mut self.matcher)
    }

    pub fn score_normalized(&mut self, word: &str, pattern: &Pattern) -> Option<f32> {
        let word_utf32 = Utf32Str::new(&word, &mut self.buf);
        let raw = pattern.score(word_utf32, &mut self.matcher)?;
        let max_len = word_utf32.len().max(1) as f32;
        Some((raw as f32 / max_len).min(1.0))
    }
}
