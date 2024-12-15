const INPUT: &str = include_str!("../input.txt");

fn count_xmas(matrix: &[Vec<char>], x: usize, y: usize) -> u32 {
    let bx = matrix.len();
    let by = matrix[x].len();

    let mut sum = 0;

    // some awful code incoming
    if x + 2 < bx
        && y + 2 < by
        && (matrix[x][y] == 'M'
            && matrix[x + 1][y + 1] == 'A'
            && matrix[x + 2][y + 2] == 'S'
            && matrix[x + 2][y] == 'M'
            && matrix[x][y + 2] == 'S')
    {
        sum += 1;
    }

    if x + 2 < bx
        && y + 2 < by
        && (matrix[x][y] == 'S'
            && matrix[x + 1][y + 1] == 'A'
            && matrix[x + 2][y + 2] == 'M'
            && matrix[x + 2][y] == 'S'
            && matrix[x][y + 2] == 'M')
    {
        sum += 1;
    }

    if x + 2 < bx
        && y + 2 < by
        && (matrix[x][y] == 'S'
            && matrix[x + 1][y + 1] == 'A'
            && matrix[x + 2][y + 2] == 'M'
            && matrix[x + 2][y] == 'M'
            && matrix[x][y + 2] == 'S')
    {
        sum += 1;
    }

    if x + 2 < bx
        && y + 2 < by
        && (matrix[x][y] == 'M'
            && matrix[x + 1][y + 1] == 'A'
            && matrix[x + 2][y + 2] == 'S'
            && matrix[x + 2][y] == 'S'
            && matrix[x][y + 2] == 'M')
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
