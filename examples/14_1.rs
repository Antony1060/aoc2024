const INPUT: &str = include_str!("../input.txt");

const SPACE_W: isize = 101;
const SPACE_H: isize = 103;

#[derive(Debug)]
struct Robot {
    x: isize,
    y: isize,
    v_x: isize,
    v_y: isize,
}

fn main() {
    let mut robots = INPUT
        .lines()
        .map(|it| {
            let mut parts = it
                .split(" ")
                .map(|it| it[2..].split(",").collect::<Vec<_>>());
            let pos = parts.next().unwrap();
            let vel = parts.next().unwrap();

            Robot {
                x: pos[0].parse().unwrap(),
                y: pos[1].parse().unwrap(),
                v_x: vel[0].parse().unwrap(),
                v_y: vel[1].parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    for _ in 0..100 {
        robots = robots
            .into_iter()
            .map(|mut robot| {
                robot.x += robot.v_x;
                robot.y += robot.v_y;

                if robot.x < 0 {
                    robot.x += SPACE_W;
                }

                if robot.y < 0 {
                    robot.y += SPACE_H;
                }

                robot.x %= SPACE_W;
                robot.y %= SPACE_H;

                robot
            })
            .collect();
    }

    let q1 = robots
        .iter()
        .filter(|robot| robot.x < (SPACE_W / 2) && robot.y < (SPACE_H / 2))
        .count();
    let q2 = robots
        .iter()
        .filter(|robot| robot.x > (SPACE_W / 2) && robot.y < (SPACE_H / 2))
        .count();
    let q3 = robots
        .iter()
        .filter(|robot| robot.x < (SPACE_W / 2) && robot.y > (SPACE_H / 2))
        .count();
    let q4 = robots
        .iter()
        .filter(|robot| robot.x > (SPACE_W / 2) && robot.y > (SPACE_H / 2))
        .count();

    let result = q1 * q2 * q3 * q4;

    println!("{}", result);
}
