use ahrs_fusion::NxpFusion;
use std::env;
use std::io::{self, BufRead};
use std::process::exit;

use micromath::{
    vector::Vector3d,
    Quaternion,
};

fn usage() {
    eprintln!("ahrs-csv FREQ");
    eprintln!("");
    eprintln!("STDIN is 6 comma-separated components of first accelerometer and then gyroscope.");
    eprintln!("");
    eprintln!("consider using e.g. xsv to format the CSV file beforehand.");
    eprintln!("");
    eprintln!("The filtered acceleration is output to stdout.");
}

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let freq: f32 = if args.len() != 2 {
        usage();
        exit(-1);
    } else {
        args[1].parse().unwrap()
    };

    eprintln!("frequency: {}", freq);
    eprintln!("setting up filter..");

    let mut filter = NxpFusion::new(freq);

    let input = io::BufReader::new(io::stdin());
    for buffer in input.lines() {
        let buffer = buffer.unwrap();
        let components = parse_line(&buffer);

        if components.len() != 6 {
            eprintln!("not 6 components in line: {}", buffer);
            exit(-1);
        }

        filter.update(
            components[0],
            components[1],
            components[2],
            components[3],
            components[4],
            components[5],
            0.0,
            0.0,
            0.0,
        );

        let q = filter.quaternion();
        let q = Quaternion::new(q[0], q[1], q[2], q[3]);
        let axl = Vector3d {
            x: components[0],
            y: components[1],
            z: components[2],
        };

        let axl = q.rotate(axl);
        println!("{},{},{}", axl.x, axl.y, axl.z);
    }
}

fn parse_line(buffer: &str) -> Vec<f32> {
    buffer
        .trim()
        .split(',')
        .map(|c| {
            c.parse::<f32>().expect(&format!("could not parse value: '{}' in line '{}'", c, buffer))
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_positive() {
        let buf = "1.0,2.0,3.0,4.0,5.0,6.0";
        let cmp = parse_line(&buf);
        assert_eq!(cmp, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    }

    #[test]
    fn parse_negative() {
        let buf = "1.0,-2.0,3.0,4.0,5.0,6.0";
        let cmp = parse_line(&buf);
        assert_eq!(cmp, [1.0, -2.0, 3.0, 4.0, 5.0, 6.0]);
    }

    #[test]
    fn parse_swift() {
        let buf = "8.451449,2.967100,4.603791,-4.062500,-8.937500,-2.718750";
        let cmp = parse_line(&buf);
        assert_eq!(cmp, [8.451449, 2.967100, 4.603791, -4.062500, -8.937500, -2.718750]);
    }
}
