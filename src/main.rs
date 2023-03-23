use std::io;
use std::collections::HashMap;
use std::str;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

struct Asteroid {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();

    let width = parse_input!(inputs[0], i32);
    let height = parse_input!(inputs[1], i32);

    let mut times: [f64; 3] = [0.0; 3];
    times[0] = parse_input!(inputs[2], f64);
    times[1] = parse_input!(inputs[3], f64);
    times[2] = parse_input!(inputs[4], f64);

    let mut first_picture = Vec::<String>::new();
    let mut second_picture = Vec::<String>::new();

    for _ in 0..height as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();

        first_picture.push(inputs[0].trim().to_string());
        second_picture.push(inputs[1].trim().to_string());
    }

    let mut asteroids = HashMap::<char, Asteroid>::new();

    extract_list_to_asteroid(
        &first_picture,
        &mut asteroids,
        true
    );

    extract_list_to_asteroid(
        &second_picture,
        &mut asteroids,
        false
    );

    let mut third_picture = vec![vec!['.' as u8; width as usize]; height as usize];

    for (c, asteroid) in &asteroids {
        let x_velocity = (asteroid.x2 - asteroid.x1)/(times[1]-times[0]);
        let y_velocity = (asteroid.y2 - asteroid.y1)/(times[1]-times[0]);

        let new_x = (asteroid.x2 + x_velocity * (times[2] - times[1])).floor() as i32;
        let new_y = (asteroid.y2 + y_velocity * (times[2] - times[1])).floor() as i32;

        if new_x < 0 || width <= new_x || new_y < 0 || height <= new_y {
            continue;
        }

        let prev_val = third_picture[new_y as usize][new_x as usize];

        if prev_val != '.' as u8 && prev_val < *c as u8 {
            continue;
        }

        third_picture[new_y as usize][new_x as usize] = *c as u8;
    }

    for picture_line in third_picture {
        println!("{}", str::from_utf8(&picture_line).expect("failed to parse utf8 string"));
    }
}

fn extract_list_to_asteroid(
    picture: &Vec<String>,
    asteroids: &mut HashMap<char, Asteroid>,
    set_first_points: bool
) {
    for (y, str) in picture.iter().enumerate() {
        for (x, c) in str.chars().enumerate() {
            if c != '.' {
                let mut asteroid = asteroids.entry(c).or_insert(
                    Asteroid{
                        x1: -1.0,
                        y1: -1.0,
                        x2: -1.0,
                        y2: -1.0,
                    }
                );

                if set_first_points {
                    asteroid.x1 = x as f64;
                    asteroid.y1 = y as f64;
                } else {
                    asteroid.x2 = x as f64;
                    asteroid.y2 = y as f64;
                }
            }
        }
    }
}