// run-rustfix

#![allow(clippy::bind_instead_of_map)]

fn main() {
    let opt = Some(1);
    let bar = |_| {
        Some(1)
    };

    // Check `OPTION_MAP_OR_NONE`.
    // Single line case.
    let _ :Option<i32> = opt.map_or(None, |x| Some(x + 1));
    // Multi-line case.
    #[rustfmt::skip]
    let _ :Option<i32> = opt.map_or(None, |x| {
                        Some(x + 1)
                       });
    // function returning `Option`
    let _ :Option<i32> = opt.map_or(None, bar);
}
