fn matrix() {
    const matrix: Vec<Vec<Vec<usize>>> = vec![
        vec![vec![0, 1, 2, 3], vec![0, 1, 2, 3, 4], vec![0, 1, 2, 3, 4]],
        vec![vec![0, 1, 2, 3, 4], vec![0, 1, 2, 3], vec![0, 1, 2, 3, 4]],
    ];
    let num = matrix[1][0][2];

    println!("{:?}", matrix);

    let tmp = [1, 2, 3];
    let i = 4;
    let tm = test(0, i, tmp);

    // функция возвращающая значение с цикла
    fn test(mut y: i32, i: i32, tmp: [i32; 3]) -> i32 {
        for x in tmp {
            if x <= i {y += x;println!("{y}")} 
            else {y += 10; }
            }
        return y; }

        let dict = std::collections::HashMap::from([
        (1, 3),(2, 31),
       ]);
}
