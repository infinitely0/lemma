use std::mem;

pub fn split_at<T: PartialEq + Clone>(elems: Vec<T>, stop: T) -> (Vec<T>, Vec<T>) {
    let pred = |t: &T| *t == stop;
    split(elems, pred)
}

// `pred` implements `Fn` so it can accept a closure that captures a variable
pub fn split<T: Clone>(elems: Vec<T>, pred: impl Fn(&T) -> bool) -> (Vec<T>, Vec<T>) {
    let mut split = elems.split(pred);
    let head = match split.next() {
        Some(h) => h.to_vec(),
        None => Vec::new(),
    };
    let tail = match split.next() {
        Some(t) => t.to_vec(),
        None => Vec::new(),
    };
    (head, tail)
}

pub fn consume_while<T, F>(elems: &mut Vec<T>, mut pred: F) -> Vec<T>
where
    F: FnMut(&T) -> bool,
{
    let index = elems.iter().position(|t| !pred(t)).unwrap_or(elems.len());
    let mut head = elems.split_off(index);
    mem::swap(elems, &mut head);
    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_at_ok() {
        let elems = vec![1, 2, 3, 4, 5];
        let (head, tail) = split_at(elems, 3);
        assert_eq!(head, vec![1, 2]);
        assert_eq!(tail, vec![4, 5]);
    }

    #[test]
    fn split_ok() {
        let elems = vec![1, 2, 3, 4, 5];
        let (head, tail) = split(elems, |t| *t == 3);
        assert_eq!(head, vec![1, 2]);
        assert_eq!(tail, vec![4, 5]);
    }

    #[test]
    fn consume_while_ok() {
        let mut elems = vec![1, 2, 3, 4, 5];
        let head = consume_while(&mut elems, |t| *t < 3);
        assert_eq!(elems, vec![3, 4, 5]);
        assert_eq!(head, vec![1, 2]);
    }
}
