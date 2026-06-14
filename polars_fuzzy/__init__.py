from __future__ import annotations
from pathlib import Path
from typing import TYPE_CHECKING
import polars as pl
from polars.plugins import register_plugin_function
from polars_fuzzy._internal import __version__ as __version__

if TYPE_CHECKING:
    from polars.type_aliases import IntoExpr

LIB = Path(__file__).parent


def fuzzy_score(
    a: IntoExpr | str,
    b: IntoExpr | str,
    *,
    case_sensitive: bool = False,
    normalize: bool = True,
) -> pl.Expr:
    """
    Compute nucleo fuzzy score between two string columns or a column and a literal.

    Parameters
    ----------
    a : str | Expr
        Column name or expression (the haystack).
    b : str | Expr
        Column name, expression, or literal pattern (the query).
    case_sensitive : bool
        Default False.
    normalize : bool
        Accent folding — "café" matches "cafe". Default True.

    Examples
    --------
    >>> df.with_columns(pf.fuzzy_score("name", "john"))
    >>> df.with_columns(pf.fuzzy_score("name_a", "name_b"))
    >>> df.with_columns(pf.fuzzy_score(pl.col("name"), pl.lit("john")))
    """
    a = pl.col(a) if isinstance(a, str) else a
    is_literal = isinstance(b, str)
    b = pl.lit(b) if is_literal else b
    return register_plugin_function(
        args=[a, b],
        plugin_path=LIB,
        function_name="fuzzy_score",
        is_elementwise=True,
        kwargs={
            "case_sensitive": case_sensitive,
            "normalize": normalize,
            "is_literal": is_literal,
        },
    )


def fuzzy_score_normalized(
    a: IntoExpr | str,
    b: IntoExpr | str,
    *,
    case_sensitive: bool = False,
    normalize: bool = True,
) -> pl.Expr:
    """Normalized fuzzy score between 0.0 and 1.0."""
    a = pl.col(a) if isinstance(a, str) else a
    is_literal = isinstance(b, str)
    b = pl.lit(b) if is_literal else b
    return register_plugin_function(
        args=[a, b],
        plugin_path=LIB,
        function_name="fuzzy_score_normalized",
        is_elementwise=True,
        kwargs={
            "case_sensitive": case_sensitive,
            "normalize": normalize,
            "is_literal": is_literal,
        },
    )


def fuzzy_join(
    left_df: pl.DataFrame,
    right_df: pl.DataFrame,
    left_on: str,
    right_on: str,
    threshold: float = 0.8,
    *,
    case_sensitive: bool = False,
) -> pl.DataFrame:
    """Fuzzy between two polars columns."""

    return (
        left_df.join(right_df, how="cross").with_columns(
            fuzzy_score_normalized(
                pl.col(left_on), pl.col(right_on), case_sensitive=case_sensitive
            ).alias("_score")
        )
        # .filter(pl.col("_score") >= threshold)
    )
