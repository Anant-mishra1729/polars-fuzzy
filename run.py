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
