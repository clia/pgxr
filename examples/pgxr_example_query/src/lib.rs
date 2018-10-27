extern crate pgxr;

use pgxr::bindings::*;
use std::os::raw::c_int;

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
extern "C" {
    pub static no_such_variable: c_int;
}

#[no_mangle]
pub extern "C" fn pg_finfo_pgxr_example_query() -> *const Pg_finfo_record
{
    &Pg_finfo_record{
        api_version: 1
    }
}

#[no_mangle]
pub extern "C" fn pgxr_example_query(fcinfo: FunctionCallInfo) -> Datum
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
