# pgxr
Writing PostgreSQL extension functions using Rust.

![](https://benchmarksgame-team.pages.debian.net/benchmarksgame/download/fast-programs-different-programming-languages.svg)

Use the Fastest, Efficient, Safe, Enjoyable language, to write In-Database programs, for the World's Most Advanced Open Source Relational Database!

# Code Example

```rust
#[macro_use]
extern crate pgxr;

use pgxr::bindings::*;

PG_MODULE_MAGIC!();

PG_FUNCTION_INFO_V1!(pg_finfo_pgxr_example_one);

#[no_mangle]
pub extern "C" fn pgxr_example_one(_fcinfo: FunctionCallInfo) -> Datum
{
    1
}

```

# Usage

```bash
git clone https://github.com/clia/pgxr.git
cd pgxr/examples/pgxr_example_one
cargo build --release
```

Then run `pg_config`

Find `PKGLIBDIR`, such as `/usr/lib/postgresql/10/lib`

```bash
sudo cp target/release/libpgxr_example_one.so /usr/lib/postgresql/10/lib
```

```bash
sudo su - postgres
psql
```

```sql
CREATE FUNCTION pgxr_example_one(integer) RETURNS integer
     AS 'libpgxr_example_one.so', 'pgxr_example_one'
     LANGUAGE C STRICT;
```

```sql
select pgxr_example_one(1);
```

That is.

Have fun!
