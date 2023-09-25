/// Creates a [HashMap] with the specified key-value pairs.
///
///
/// [HashMap]: std::collections::HashMap
///
/// # Examples
///
/// ```
/// use mapro::map;
///
/// let m = map!{
///     "one" => 1,
///     "two" => 2
/// };
///
/// assert_eq!(m["one"], 1);
/// assert_eq!(m["two"], 2);
/// ```
#[macro_export]
macro_rules! map {
    ( ) => {{ std::collections::HashMap::new() }};
    ( $($key:expr => $val:expr), * $(,)? ) => {{
        std::collections::HashMap::from([
            $( ($key, $val), )*
        ])
    }};
}

/// Creates a [BTreeMap] with the specified key-value pairs.
///
/// [BTreeMap]:std::collections::BTreeMap
///
/// # Examples
///
/// ```
/// use mapro::bt_map;
///
/// let m = bt_map!{
///     "one" => 1,
///     "two" => 2
/// };
///
/// assert_eq!(m["one"], 1);
/// assert_eq!(m["two"], 2);
/// ```
#[macro_export]
macro_rules! bt_map {
    ( ) => {{ std::collections::BTreeMap::new() }};
    ( $($key:expr => $val:expr), * $(,)? ) => {{
        std::collections::BTreeMap::from([
            $( ($key, $val), )*
        ])
    }};
}

/// Creates a [HashSet] containing the specified elements.
///
/// [HashSet]:std::collections::HashSet
///
/// # Examples
///
/// ```
/// use mapro::set;
///
/// let s = set!{1, 2, 3};
///
/// assert!(s.contains(&1));
/// assert!(s.contains(&2));
/// assert!(s.contains(&3));
/// ```
#[macro_export]
macro_rules! set {
    ( ) => {{ std::collections::HashSet::new() }};
    ( $($elm:expr),* $(,)?) => {{
        std::collections::HashSet::from([
            $($elm,)*
        ])
    }};
}

/// Creates a [BTreeSet] containing the specified elements.
///
/// [BTreeSet]:std::collections::BTreeSet
///
/// # Examples
///
/// ```
/// use mapro::bt_set;
/// let s = bt_set!{1, 2, 3};
/// assert!(s.contains(&1));
/// assert!(s.contains(&2));
/// assert!(s.contains(&3));
/// ```
#[macro_export]
macro_rules! bt_set {
    ( ) => {{ std::collections::BTreeSet::new() }};
    ( $($elm:expr),* $(,)?) => {{
        std::collections::BTreeSet::from([
            $($elm,)*
        ])
    }};
}

/// Creates a [VecDeque] containing the specified elements.
///
/// [VecDeque]:std::collections::VecDeque
///
/// # Examples
///
/// ```
/// use mapro::vec_d;
///
/// let vd = vec_d!{1, 2, 3};
///
/// assert_eq!(vd[0], 1);
/// assert_eq!(vd[1], 2);
/// assert_eq!(vd[2], 3);
/// ```
#[macro_export]
macro_rules! vec_d {
    ( ) => {{ std::collections::VecDeque::new() }};
    ( $($elm:expr),* $(,)?) => {{
        std::collections::VecDeque::from([
            $($elm,)*
        ])
    }};
}

/// Creates a [BinaryHeap] containing the specified elements.
///
/// [BinaryHeap]:std::collections::BinaryHeap
///
/// # Examples
///
/// ```
/// use mapro::heap;
///
/// let mut heap = heap!{1, 2, 3};
///
/// assert_eq!(heap.pop(), Some(3));
/// assert_eq!(heap.pop(), Some(2));
/// assert_eq!(heap.pop(), Some(1));
/// ```
#[macro_export]
macro_rules! heap {
    ( ) => {{ std::collections::BinaryHeap::new() }};
        ( $($elm:expr),* $(,)?) => {{
        std::collections::BinaryHeap::from([
            $($elm,)*
        ])
    }};
}

mod tests {
    use super::*;
    use std::collections::*;

    #[test]
    fn test_map_macro() {
        let map: HashMap<(), ()> = map!();
        assert_eq!(map.len(), 0);

        let map = map!("a" => 1, "b" => 2);
        assert_eq!(map.get("a"), Some(&1));
        assert_eq!(map.get("b"), Some(&2));
    }

    #[test]
    fn test_bt_map_macro() {
        let map : BTreeMap<(), ()> = bt_map!();
        assert_eq!(map.len(), 0);

        let map = bt_map!("a" => 1, "b" => 2);
        assert_eq!(map.get("a"), Some(&1));
        assert_eq!(map.get("b"), Some(&2));
    }

    #[test]
    fn test_set_macro() {
        let set: HashSet<()> = set!();
        assert!(set.is_empty());

        let set = set!(1, 2, 3);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
    }

    #[test]
    fn test_bt_set_macro() {
        let set: BTreeSet<()> = bt_set!();
        assert!(set.is_empty());

        let set = bt_set!(1, 2, 3);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
    }

    #[test]
    fn test_vec_d_macro() {
        let vec: VecDeque<()> = vec_d!();
        assert!(vec.is_empty());

        let vec = vec_d!(1, 2, 3);
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
        assert_eq!(vec[2], 3);
    }

    #[test]
    fn test_heap_macro() {
        let heap: BinaryHeap<()> = heap!();
        assert!(heap.is_empty());

        let mut heap = heap!(1, 2, 3);
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(1));
    }
}
