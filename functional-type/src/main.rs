#![recursion_limit = "1024"]

use functional_type::{
    bool::*,
    generate_unsigned_integer,
    integer::{signed::*, unsigned::*},
    invalid::Invalid,
    operators::*,
    type_traits::{PrivateDiv, ToTypeDisplayWrapper},
};

pub fn main() {
    println!(
        "fun: {:?}",
        <If<IsGreaterOrEqual<Invalid, U1>, Diff<U1, U1>, U1>>::display()
    );

    println!("sum: {}", <Sum<U1023, U1023>>::display());
    println!("diff: {}", <Diff<U1023, U1023>>::display());
    println!("mul: {}", <Product<U1023, U1023>>::display());
    println!("div: {}", <Divide<U1000, U10>>::display());
    println!("div: {}", <ShiftLeft<U10, U10>>::display());
    println!("rem: {}", <Remainder<U234, U10>>::display());
    println!("pow: {}", <Pow<U10, U10>>::display());

    // println!(
    //     "TEST: {}",
    //     <U1023 as PrivateDiv<U768, U8>>::Output::display()
    // );

    println!("sint: {}", <Sum<N3, P2>>::display());
    println!("sint: {}", <Sum<P3, N3>>::display());

    println!(
        "big: {}",
        <generate_unsigned_integer!(18446744073709551615)>::display()
    );
    println!(
        "big: {}",
        <Divide<generate_unsigned_integer!(18446744073709551615), U10>>::display()
    );
}
