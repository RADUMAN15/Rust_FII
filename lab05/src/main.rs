use std::fs;
use std::io;
use std::thread::sleep;
use std::time::Duration;
const WIDTH: usize = 10;
const HEIGHT: usize = 10;

fn oldest_youngest() -> Result<(), io::Error> {
    let s: String = fs::read_to_string("text.txt")?;
    let mut array: [[&str; 3]; 4] = [[""; 3]; 4];
    let mut lines: usize = 0;
    let mut columns: usize;
    let mut res: (i32, usize, i32, usize) = (std::i32::MAX, 0, std::i32::MIN, 0);
    for line in s.lines() {
        columns = 0;
        for atributes in line.split(',') {
            array[lines][columns] = atributes.clone();
            //print!("{}", atributes);
            columns = columns + 1;
        }
        if array[lines][columns - 1].parse::<i32>().unwrap() < res.0 {
            res.0 = array[lines][columns - 1].parse::<i32>().unwrap();
            res.1 = lines;
        }
        if array[lines][columns - 1].parse::<i32>().unwrap() > res.2 {
            res.2 = array[lines][columns - 1].parse::<i32>().unwrap();
            res.3 = lines;
        }
        lines = lines + 1;
        //println!("");
    }
    println!(
        "The youngest student is: {:?}\nThe oldest student   is: {:?}",
        array[res.1], array[res.3]
    );
    //println!("{:?}", array);
    Ok(())
}

fn life() -> Result<(), io::Error> {
    let string_matrix: String = fs::read_to_string("start.txt")?;
    let mut matrix: [[bool; 10]; 10] = [[false; WIDTH]; HEIGHT];
    let mut i: usize = 0;
    let mut j: usize;
    for line in string_matrix.lines() {
        j = 0;
        for ch in line.chars() {
            matrix[i][j] = ch == '1';
            j += 1;
        }
        i += 1;
    }
    //print_matrix(&matrix);
    let mut generation = 0;
    loop {
        print_matrix(&matrix);
        matrix = next_generation(matrix);
        sleep(Duration::from_millis(500));
        generation += 1;

        if generation > 10 {
            break;
        }
    }
    //println!("{:?}", matrix);
    Ok(())
}
fn print_matrix(matrix: &[[bool; WIDTH]; HEIGHT]) {
    print!("\x1B[2J\x1B[H"); // Clear the terminal screen
    for i in 1..HEIGHT {
        for j in 1..WIDTH {
            print!("{}", matrix[i][j] as u8);
        }
        println!();
    }
}
fn next_generation(matrix: [[bool; WIDTH]; HEIGHT]) -> [[bool; WIDTH]; HEIGHT] {
    let mut new_state_matrix = [[false; WIDTH]; HEIGHT];

    for i in 1..HEIGHT {
        for j in 1..WIDTH {
            let neighbours = count_live_neighbours(&matrix, i, j);
            if matrix[i][j] == true {
                new_state_matrix[i][j] = neighbours == 2 || neighbours == 3;
            } else {
                new_state_matrix[i][j] = neighbours == 3;
            }
        }
    }
    new_state_matrix
}
fn count_live_neighbours(matrix: &[[bool; WIDTH]; HEIGHT], x: usize, y: usize) -> u32 {
    let d_i: [isize; 3] = [-1, 0, 1];
    let d_j: [isize; 3] = [-1, 0, 1];
    let mut num_of_neighbours = 0;
    for x_add in d_j {
        for y_add in d_i {
            if x_add == 0 && y_add == 0 {
                continue;
            }
            let new_x = x as isize + x_add;
            let new_y = y as isize + y_add;
            if new_x >= 0 && new_x < WIDTH as isize && new_y >= 0 && new_y < HEIGHT as isize {
                if matrix[new_x as usize][new_y as usize] == true {
                    num_of_neighbours += 1;
                }
            }
        }
    }
    num_of_neighbours
}

struct matrix {
    canvas: [[char; 50]; 10],
}
fn set_pixels(c: &mut matrix, pixels: &[(usize, usize, u8)]) {
    for pixel in pixels {
        c.canvas[pixel.0][pixel.1] = pixel.2 as char;
    }
}
fn new_canvas() -> matrix {
    let c: matrix = matrix {
        canvas: [[' '; 50]; 10],
    };
    c
}
fn print(c: matrix) {
    for x in c.canvas {
        for y in x {
            print!("{y}");
        }
        print!("\n");
    }
}

use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
struct Person {
    name: String,
    phone: String,
    age: u32,
}

fn min_max_age() -> Result<(), io::Error> {
    let ans: (String, String) = (String::from(" "), String::from(" "));
    let content = fs::read_to_string("person.json").unwrap();
    let mut students: [Person; 4]; // = [Person {name: String::from(""),phone: String::from(""),age: 0,}.clone(); 4];
    if let Ok(persons) = serde_json::from_str::<[Person; 4]>(&content) {
        students = persons;
    } else {
        eprintln!("Failed to parse JSON into array of persons.");
    }
    //print!("{:?}", students);
    let mut res: (u32, u32, u32, u32) = (0, 0, 999999, 0);
    let mut i = 0;
    /*for pers in students{ //imi crapa deoarece nu initializez bine array persons

        if pers.age < res.0{
            res.1 = i;
        }
        if pers.age > res.1{
            res.3 = i;
        }
        i = i + 1;
    }
    */
    //printez linia res.0 si linia res.1
    Ok(())
}

fn main() {
    //P1
    oldest_youngest().unwrap();

    //P2
    let mut canvas = new_canvas();
    let c = &mut canvas;

    set_pixels(c, &[(4, 25, 124), (3, 33, 124), (2, 24, 95), (4, 3, 95)]);
    set_pixels(c, &[(7, 2, 95), (4, 21, 124), (5, 16, 95)]);
    set_pixels(c, &[(4, 41, 124), (7, 1, 124), (5, 8, 92)]);
    set_pixels(c, &[(1, 31, 40), (2, 3, 95), (2, 41, 124)]);
    set_pixels(
        c,
        &[
            (2, 16, 95),
            (5, 35, 92),
            (6, 3, 95),
            (2, 11, 95),
            (5, 3, 95),
        ],
    );
    set_pixels(
        c,
        &[
            (2, 38, 95),
            (4, 9, 40),
            (3, 41, 124),
            (2, 37, 95),
            (2, 25, 124),
        ],
    );
    set_pixels(
        c,
        &[
            (5, 27, 124),
            (2, 27, 124),
            (4, 0, 124),
            (3, 35, 47),
            (2, 18, 95),
        ],
    );
    set_pixels(c, &[(4, 13, 124), (4, 37, 95), (4, 16, 40), (3, 6, 124)]);
    set_pixels(c, &[(7, 32, 47), (4, 20, 124), (5, 11, 95), (5, 42, 95)]);
    set_pixels(c, &[(5, 15, 92), (4, 34, 124), (4, 45, 41), (5, 24, 95)]);
    set_pixels(c, &[(4, 2, 40), (7, 3, 95), (2, 44, 95)]);
    set_pixels(
        c,
        &[
            (6, 30, 95),
            (5, 45, 95),
            (4, 31, 124),
            (4, 7, 124),
            (3, 43, 39),
        ],
    );
    set_pixels(c, &[(5, 17, 95), (1, 27, 124), (2, 5, 95)]);
    set_pixels(
        c,
        &[
            (3, 44, 95),
            (3, 19, 92),
            (5, 23, 95),
            (3, 8, 47),
            (2, 10, 95),
        ],
    );
    set_pixels(c, &[(6, 6, 124), (5, 19, 47), (3, 24, 95), (3, 27, 124)]);
    set_pixels(
        c,
        &[
            (3, 10, 95),
            (4, 44, 95),
            (2, 9, 95),
            (0, 32, 95),
            (5, 2, 95),
        ],
    );
    set_pixels(c, &[(6, 2, 95), (7, 31, 95), (1, 25, 124), (2, 36, 95)]);
    set_pixels(
        c,
        &[
            (3, 46, 92),
            (5, 25, 44),
            (1, 43, 124),
            (5, 46, 47),
            (3, 15, 47),
        ],
    );
    set_pixels(c, &[(4, 17, 95), (2, 23, 95), (3, 39, 92)]);
    set_pixels(c, &[(4, 47, 124), (2, 45, 95), (3, 37, 95)]);
    set_pixels(
        c,
        &[
            (5, 44, 95),
            (2, 2, 95),
            (5, 10, 95),
            (5, 9, 95),
            (4, 43, 124),
        ],
    );
    set_pixels(c, &[(4, 38, 41), (2, 17, 95), (0, 26, 95)]);
    set_pixels(c, &[(4, 18, 41), (7, 5, 47), (5, 41, 124), (5, 33, 124)]);
    set_pixels(c, &[(5, 12, 47), (5, 22, 92), (6, 33, 124), (5, 31, 124)]);
    set_pixels(
        c,
        &[
            (4, 40, 124),
            (3, 3, 95),
            (4, 4, 124),
            (6, 31, 47),
            (3, 4, 96),
        ],
    );
    set_pixels(c, &[(0, 42, 95), (5, 18, 95), (4, 27, 124)]);
    set_pixels(
        c,
        &[
            (3, 12, 92),
            (2, 32, 95),
            (5, 37, 95),
            (5, 26, 95),
            (5, 39, 47),
        ],
    );
    set_pixels(c, &[(3, 25, 96), (4, 14, 124), (4, 33, 124), (3, 1, 47)]);
    set_pixels(
        c,
        &[
            (5, 36, 95),
            (7, 30, 95),
            (6, 4, 47),
            (4, 24, 95),
            (1, 32, 95),
        ],
    );
    set_pixels(c, &[(3, 22, 47), (4, 23, 40), (5, 6, 124)]);
    set_pixels(c, &[(1, 33, 41), (1, 41, 124), (7, 29, 124)]);
    set_pixels(c, &[(4, 6, 124), (5, 38, 95), (3, 31, 124), (7, 4, 95)]);
    set_pixels(c, &[(4, 11, 41), (4, 10, 95), (5, 1, 92)]);
    set_pixels(c, &[(2, 43, 124), (3, 17, 95), (5, 4, 44), (4, 36, 40)]);
    set_pixels(c, &[(5, 43, 46)]);

    print(canvas);

    //P3
    min_max_age().unwrap();

    //BONUS
    //life().unwrap();
}
