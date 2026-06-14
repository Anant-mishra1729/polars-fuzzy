# Polars Fuzzy

Fast fuzzy string matching for [Polars](https://pola.rs) DataFrames, powered by Rust.

[![PyPI](https://img.shields.io/pypi/v/polars-fuzzy)](https://pypi.org/project/polars-fuzzy/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## Installation

```bash
pip install polars-fuzzy
```

## Quick Start

```python
import polars as pl
import polars_fuzzy as pf

df = pl.DataFrame({"name": ["John Smith", "Jane Doe", "Benjamin", "Johnny", None]})

# Raw fuzzy score - higher is better, null means no match
print(df.with_columns(pf.fuzzy_score("name", "john").alias("score")))
# ┌────────────┬───────┐
# │ name       ┆ score │
# │ ---        ┆ ---   │
# │ str        ┆ u32   │
# ╞════════════╪═══════╡
# │ John Smith ┆ 114   │
# │ Jane Doe   ┆ null  │
# │ Benjamin   ┆ null  │
# │ Johnny     ┆ 114   │
# │ null       ┆ null  │
# └────────────┴───────┘

# Normalized score - 0.0 to 1.0, easier to threshold
print(df.with_columns(pf.fuzzy_score_normalized("name", "john").alias("score")))

# Filter - only rows that match
print(df.filter(pf.fuzzy_score("name", "john").is_not_null()))

# Sort by best match
print(
    df.with_columns(pf.fuzzy_score("name", "john").alias("score")).sort(
        "score", descending=True, nulls_last=True
    )
)
```

## Column vs Column

Useful for record linkage - matching names across two datasets.

```python
customers = pl.DataFrame({"name":   ["John Smith", "Jane Doe"]})
invoices  = pl.DataFrame({"client": ["Jon Smith",  "J. Doe"]})

customers.join(invoices, how="cross").with_columns(
    pf.fuzzy_score_normalized("name", "client").alias("score")
).filter(pl.col("score").is_not_null()).sort("score", descending=True)
```

## Fuzzy JOIN

```python
result = pf.fuzzy_join(
    customers, invoices,
    left_on="name",
    right_on="client",
    threshold=0.8,
)
```

## Options

```python
pf.fuzzy_score(
    "name",                 # haystack - column name or pl.Expr
    "john",                 # query - string literal, column name, or pl.Expr
    case_sensitive=False,   # default: False
    normalize=True,         # accent folding: "café" matches "cafe"
)
```

## Performance

- Zero heap allocations in the per-row hot loop
- Literal patterns compiled **once** per column, not per row
- Nulls propagated without entering the matcher
- Backed by [Nucleo](https://github.com/helix-editor/nucleo) - the fuzzy engine powering the Helix editor

## License

MIT
