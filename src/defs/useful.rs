enum Ordering {
    Less,
    Equal,
    Greater,
}

pub fn binary_search(xs: &[i32], x: i32) -> bool {
    if xs.is_empty() {
        return false;
    }
    let mid = xs.len() / 2;
    let subslice = match xs[mid].cmp(&x) {
        std::cmp::Ordering::Less => &xs[mid + 1..],
        std::cmp::Ordering::Equal => return true,
        std::cmp::Ordering::Greater => &xs[..mid],
    };
    binary_search(subslice, x)
}

pub fn matrix() {
    // const matrix: Vec<Vec<Vec<usize>>> = vec![
    //     vec![vec![0, 1, 2, 3], vec![0, 1, 2, 3, 4], vec![0, 1, 2, 3, 4]],
    //     vec![vec![0, 1, 2, 3, 4], vec![0, 1, 2, 3], vec![0, 1, 2, 3, 4]],
    // ];
    // let num = matrix[1][0][2];

    let dict = std::collections::HashMap::from([(1, 3), (2, 31)]);

    // loop iterator with index[] !
    let mut tmp = vec![1, 2, 3];

    for i in 4..=6 {
        tmp.push(i);
    }

    let _vsop = vec![
        vec![vec![vec![4.4, 0.0, 0.0]], vec![vec![26087.9, 0.0, 0.0]]],
        vec![
            vec![vec![0.11737528961, 1.98357498767, 26087.90314157420]],
            vec![vec![0.00274646065, 3.95008450011, 26087.90314157420]],
        ],
    ];
    for val in &_vsop[0][0][0] {
        println!("{val}");
    }
}

// let mut tmp: HashMap<i32, HashMap<i32, &str>> = [(1, [(2, "2"), (1, "1")].into()), (2, [(1, "1"), (2, "2")].into())].into();
