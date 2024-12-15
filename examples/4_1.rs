const INPUT: &str = include_str!("../input.txt");

fn count_xmas(matrix: &[Vec<char>], x: usize, y: usize) -> u32 {
    let bx = matrix.len();
    let by = matrix[x].len();

    let mut sum = 0;

    // some awful code incoming
    if x + 3 < bx
        && (matrix[x][y] == 'X'
            && matrix[x + 1][y] == 'M'
            && matrix[x + 2][y] == 'A'
            && matrix[x + 3][y] == 'S')
    {
        sum += 1;
    }

    if x as isize - 3 >= 0
        && (matrix[x][y] == 'X'
            && matrix[x - 1][y] == 'M'
            && matrix[x - 2][y] == 'A'
            && matrix[x - 3][y] == 'S')
    {
        sum += 1;
    }

    if y + 3 < by
        && (matrix[x][y] == 'X'
            && matrix[x][y + 1] == 'M'
            && matrix[x][y + 2] == 'A'
            && matrix[x][y + 3] == 'S')
    {
        sum += 1;
    }

    if y as isize - 3 >= 0
        && (matrix[x][y] == 'X'
            && matrix[x][y - 1] == 'M'
            && matrix[x][y - 2] == 'A'
            && matrix[x][y - 3] == 'S')
    {
        sum += 1;
    }

    if x + 3 < bx
        && y + 3 < by
        && (matrix[x][y] == 'X'
            && matrix[x + 1][y + 1] == 'M'
            && matrix[x + 2][y + 2] == 'A'
            && matrix[x + 3][y + 3] == 'S')
    {
        sum += 1;
    }

    if x as isize - 3 >= 0
        && y as isize - 3 >= 0
        && (matrix[x][y] == 'X'
            && matrix[x - 1][y - 1] == 'M'
            && matrix[x - 2][y - 2] == 'A'
            && matrix[x - 3][y - 3] == 'S')
    {
        sum += 1;
    }

    if x + 3 < bx
        && y as isize - 3 >= 0
        && (matrix[x][y] == 'X'
            && matrix[x + 1][y - 1] == 'M'
            && matrix[x + 2][y - 2] == 'A'
            && matrix[x + 3][y - 3] == 'S')
    {
        sum += 1;
    }

    if x as isize - 3 >= 0
        && y + 3 < by
        && (matrix[x][y] == 'X'
            && matrix[x - 1][y + 1] == 'M'
            && matrix[x - 2][y + 2] == 'A'
            && matrix[x - 3][y + 3] == 'S')
    {
        sum += 1;
    }

    if sum > 0 {
        println!("{x}, {y} = {sum}");
    }

    sum
}

fn main() {
    let matrix = INPUT
        .split("\n")
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sum = 0;

    for (i, line) in matrix.iter().enumerate() {
        for (j, _) in line.iter().enumerate() {
            sum += count_xmas(&matrix, i, j);
        }
    }

    println!("{}", sum);
}
