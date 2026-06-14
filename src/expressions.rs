#![allow(clippy::unused_unit)]
use polars::prelude::*;
use polars_arrow::legacy::utils::CustomIterTools;
use pyo3_polars::derive::polars_expr;
use serde::Deserialize;

use crate::algorithms::nucleo::NucleoMatcher;

#[derive(Deserialize, Debug)]
pub struct FuzzyScoreKwargs {
    pub case_sensitive: bool,
    pub normalize: bool,
    pub is_literal: bool, // To check if the second argument is column or literal
}

#[polars_expr(output_type=UInt32)]
fn fuzzy_score(inputs: &[Series], kwargs: FuzzyScoreKwargs) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;
    let cb = inputs[1].str()?;

    let mut matcher = NucleoMatcher::new();
    let out: UInt32Chunked = if kwargs.is_literal {
        //. --- Column vs literal ---
        let pattern_str = cb
            .get(0)
            .ok_or_else(|| PolarsError::ComputeError("empty pattern".into()))?;
        let pattern =
            NucleoMatcher::compile_pattern(pattern_str, kwargs.case_sensitive, kwargs.normalize);
        ca.into_iter()
            .map(|a| a.and_then(|s| matcher.score(s, &pattern)))
            .collect_trusted()
    } else {
        // ---Column vs column ---
        ca.into_iter()
            .zip(cb.into_iter())
            .map(|(a, b)| match (a, b) {
                (Some(a), Some(b)) => {
                    let pattern =
                        NucleoMatcher::compile_pattern(b, kwargs.case_sensitive, kwargs.normalize);
                    matcher.score(a, &pattern)
                },
                _ => None,
            })
            .collect_trusted()
    };

    Ok(out.into_series())
}

#[polars_expr(output_type=Float32)]
fn fuzzy_score_normalized(inputs: &[Series], kwargs: FuzzyScoreKwargs) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;
    let cb = inputs[1].str()?;

    let mut matcher = NucleoMatcher::new();
    let out: Float32Chunked = if kwargs.is_literal {
        //. --- Column vs literal ---
        let pattern_str = cb
            .get(0)
            .ok_or_else(|| PolarsError::ComputeError("empty pattern".into()))?;
        let pattern =
            NucleoMatcher::compile_pattern(pattern_str, kwargs.case_sensitive, kwargs.normalize);
        ca.into_iter()
            .map(|a| a.and_then(|s| matcher.score_normalized(s, &pattern)))
            .collect_trusted()
    } else {
        // ---Column vs column ---
        ca.into_iter()
            .zip(cb.into_iter())
            .map(|(a, b)| match (a, b) {
                (Some(a), Some(b)) => {
                    let pattern =
                        NucleoMatcher::compile_pattern(b, kwargs.case_sensitive, kwargs.normalize);
                    matcher.score_normalized(a, &pattern)
                },
                _ => None,
            })
            .collect_trusted()
    };

    Ok(out.into_series())
}
