use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufReader, prelude::*};

use crate::geometry::point::Point3D;

pub struct Obj<'a> {
    pub vertices: Vec<Point3D>,
    pub faces: Vec<Vec<usize>>,
    pub path: &'a str,
}

impl Obj<'_> {
    pub fn new(path: &str) -> Obj<'_> {
        Obj {
            vertices: Vec::new(),
            faces: Vec::new(),
            path, //path: path
        }
    }

    pub fn load_obj_file(&mut self) -> io::Result<()> {
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
        //

        let mut point_creation_vs: Vec<f32> = Vec::new();
        let mut point_creation_fs: Vec<usize> = Vec::new();

        for val in values.iter() {
            match line.chars().next() {
                Some('v') => {
                    let parsed_val: f32 = val.parse().unwrap();
                    point_creation_vs.push(parsed_val)
                }
                Some('f') => {
                    let parsed_val: usize = val.parse().unwrap();
                    point_creation_fs.push(parsed_val);
                }
                _ => (),
            }
        }

        // Ajout du point 3d dans la liste des vertices que si c'est v
        if !point_creation_vs.is_empty() {
            let point = Point3D::from_vec(&point_creation_vs);
            self.vertices.push(point);
        }

        if !point_creation_fs.is_empty() {
            self.faces.push(point_creation_fs);
        }
    }
}

impl Display for Obj<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Obj [
            path: {}
            vertices: {:#?} ({})
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
