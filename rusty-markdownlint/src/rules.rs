use crate::ruleset::CheckFn;

mod extensions;
mod md001;
mod md002;
mod md003;
mod md004;
mod md009;
mod md010;
mod md041;

crate fn get_rules<'a>() -> Vec<Box<CheckFn<'a>>> {
    vec![
        Box::new(md001::check),
        Box::new(md002::check),
        Box::new(md003::check),
        Box::new(md004::check),
        Box::new(md009::check),
        Box::new(md010::check),
        Box::new(md041::check),
    ]
}
