use crate::ruleset::CheckFn;

mod extensions;
mod md001;
mod md002;
mod md003;
mod md004;

crate fn get_rules<'a>() -> Vec<Box<CheckFn<'a>>> {
    vec![
        Box::new(md001::check),
        Box::new(md002::check),
        Box::new(md003::check),
        Box::new(md004::check),
    ]
}
