fn matrix() {
    const matrix: Vec<Vec<Vec<usize>>> = vec![
        vec![vec![0, 1, 2, 3], vec![0, 1, 2, 3, 4], vec![0, 1, 2, 3, 4]],
        vec![vec![0, 1, 2, 3, 4], vec![0, 1, 2, 3], vec![0, 1, 2, 3, 4]],
    ];
    let num = matrix[1][0][2];

    println!("{:?}", matrix)
}
