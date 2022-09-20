#![allow(unused_macros)]
#![allow(non_snake_case)]
// ----------------------------------------------------------------------------------------------------
// input macro by @tanakh https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
// ----------------------------------------------------------------------------------------------------
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

// ----------------------------------------------------------------------------------------------------
// main code
// ----------------------------------------------------------------------------------------------------
#[derive(Clone)]
struct Tree {
    location: usize,
    l: Option<usize>,
    r: Option<usize>,
}

impl Tree {
    fn new() -> Self {
        Self {
            location: 0,
            l: None,
            r: None,
        }
    }
}

#[derive(Clone, Copy, Default)]
struct Node<T> {
    id: usize,
    x: T,
    y: T,
}

struct KDTree2<T>
where
    T: Copy + Clone + Default + Ord,
{
    p: Vec<Node<T>>,
    node_id: usize,
    tree: Vec<Tree>,
}

impl<T> KDTree2<T>
where
    T: Copy + Clone + Default + Ord,
{
    pub fn new(point: Vec<(T, T)>) -> Self {
        let p = point
            .iter()
            .enumerate()
            .map(|(i, &(x, y))| Node { id: i, x, y })
            .collect::<Vec<_>>();
        Self {
            p,
            node_id: 0,
            tree: vec![Tree::new(); point.len()],
        }
    }

    pub fn construct(&mut self) {
        self.construct_inner(0, self.p.len(), 0);
    }

    fn construct_inner(&mut self, l: usize, r: usize, depth: usize) -> Option<usize> {
        if l >= r {
            return None;
        }
        if depth % 2 == 0 {
            self.p[l..r].sort_by_key(|Node { id: _, x, y: _ }| *x);
        } else {
            self.p[l..r].sort_by_key(|Node { id: _, x: _, y }| *y);
        };

        let mid = (l + r) / 2;
        let t = self.node_id;
        self.node_id += 1;
        self.tree[t].location = mid;
        self.tree[t].l = self.construct_inner(l, mid, depth + 1);
        self.tree[t].r = self.construct_inner(mid + 1, r, depth + 1);

        Some(t)
    }
}

fn main() {
    input! {};
}
