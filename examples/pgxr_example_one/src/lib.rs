extern crate pgxr;

use pgxr::bindings::*;

#[no_mangle]
pub extern "C" fn Pg_magic_func() -> *const Pg_magic_struct
{
    &Pg_magic_struct{
        len: 28,
        version: 1000,
        funcmaxargs: 100,
        indexmaxkeys: 32,
        namedatalen: 64,
        float4byval: 1,
        float8byval: 1,
    }
}

#[no_mangle]
pub extern "C" fn pg_finfo_pgxr_example_one() -> *const Pg_finfo_record
{
    &Pg_finfo_record{
        api_version: 1
    }
}

#[no_mangle]
pub extern "C" fn pgxr_example_one(fcinfo: FunctionCallInfo) -> Datum
{
    1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
