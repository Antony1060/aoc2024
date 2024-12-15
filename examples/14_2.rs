use image::ImageBuffer;

const INPUT: &str = include_str!("../input.txt");

const DIR: &str = "./day14_images";

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

    for i in 0..10000 {
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

        let mut buffer = ImageBuffer::new(SPACE_W as u32, SPACE_H as u32);

        for pixel in buffer.pixels_mut() {
            *pixel = image::Rgb([0u8, 0u8, 0u8]);
        }

        for robot in &robots {
            buffer.put_pixel(robot.x as u32, robot.y as u32, image::Rgb([255, 255, 255]));
        }

        buffer.save(format!("{}/second{}.png", DIR, i)).unwrap();
    }
}
