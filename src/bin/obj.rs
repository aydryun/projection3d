use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::io::{self, BufReader, prelude::*};
use std::time::Instant;

struct Obj<'a> {
    pub vertices: Vec<f32>,
    pub faces: Vec<f32>,
    pub path: &'a str,
}

fn main() {
    println!("un : {}", 1);

    // let obj = read_obj_file("./assets/cube.obj");
    // let obj = read_obj_file("./assets/cheburashka.obj");
    // let obj = buffer_read("./assets/cheburashka.obj");
    // let obj = buffer_read("./assets/cube.obj");

    // let mut obj = Obj::new("./assets/cube.obj");
    let mut obj = Obj::new("./assets/cheburashka.obj");

    // benchmark de la vitesse de lecture des vertices & faces
    let start = Instant::now();
    obj.buffer_read_obj_file().unwrap();
    let finish = start.elapsed();

    println!("obj: {}", obj);
    println!("durée de lecture: {:?}", finish);
}

fn read_obj_file(path: &str) -> Result<String, Box<dyn Error>> {
    let message: String = fs::read_to_string(path)?;
    Ok(message)
}

impl Obj<'_> {
    pub fn new(path: &str) -> Obj<'_> {
        Obj {
            vertices: Vec::new(),
            faces: Vec::new(),
            path: path,
        }
    }

    fn buffer_read_obj_file(&mut self) -> io::Result<()> {
        let file = File::open(self.path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            self.parse_line(&line?);
        }

        Ok(())
    }

    fn parse_line(&mut self, line: &str) {
        if line.starts_with('#') || line.is_empty() {
            return;
        }

        let mut values: Vec<&str> = line.split_whitespace().collect();
        values.remove(0); // rm 'v' ou 'f'

        for val in values.iter() {
            let parsed_val: f32 = val.parse().unwrap();

            match line.chars().next() {
                Some('v') => self.vertices.push(parsed_val),
                Some('f') => self.faces.push(parsed_val),
                _ => (),
            }
        }
    }
}

impl Display for Obj<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Obj [
            path: {}
            vertices: {:?} ({})
            faces: {:?} ({})
            ]",
            self.path,
            self.vertices,
            self.vertices.len(),
            self.faces,
            self.faces.len()
        )
    }
}
// fn buffer_read(path: &str) -> io::Result<()> {
//     let file = File::open(path)?;
//     let reader = BufReader::new(file);
//
//     for line in reader.lines() {
//         let line_vec = parse_line(&line?);
//
//         if let Some(line_vec) = line_vec {
//             println!("line_ecs: {:?}", line_vec);
//         }
//     }
//
//     Ok(())
// }
//
// fn parse_line(line: &str) -> Option<Vec<f32>> {
//     let mut line_vls: Vec<f32> = vec![];
//     if line.starts_with('#') || line.is_empty() {
//         return None;
//     }
//
//     if line.starts_with('v') || line.starts_with('f') {
//         let mut values: Vec<&str> = line.split_whitespace().collect();
//
//         for val in values.iter() {
//             let parsed_val: f32 = val.parse().unwrap();
//             line_vls.push(parsed_val);
//         }
//     }
//     Some(line_vls)
// }
