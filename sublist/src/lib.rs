#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sublist<T: PartialEq>(fst: &[T], snd: &[T]) -> bool {
    let size: usize = fst.len();
    (fst.is_empty() && !snd.is_empty()) || snd.windows(size).any(|xs| xs == fst)
}

fn is_superlist<T: PartialEq>(fst: &[T], snd: &[T]) -> bool {
    let size: usize = snd.len();
    (!fst.is_empty() && snd.is_empty()) || fst.windows(size).any(|xs| xs == snd)
}

pub fn sublist<T: PartialEq>(fst: &[T], snd: &[T]) -> Comparison {
    if fst == snd {
        Comparison::Equal
    } else if is_sublist(fst, snd) {
        Comparison::Sublist
    } else if is_superlist(fst, snd) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}
