mod error;

use std::{env, fs, path::Path, process};

use nalgebra::DMatrix;

use error::{MatrixError, UnwrapOrDisplay};

const HELP: &str = "\
Usage: solve-matrix [OPTIONS] [SOURCE DATASET PATH] [TARGET DATASET PATH]

OPTIONS
    --help | -h     Show this help message
";

// Based on: https://github.com/colour-science/colour/blob/a13524cec1940468706ae65c1163c85b89661e28/colour/algebra/regression.py#L36-L72
fn solve_matrix(source: DMatrix<f64>, target: DMatrix<f64>) -> DMatrix<f64> {
    // Unwrap: Only fails if eps is negative:
    // https://docs.rs/nalgebra/latest/src/nalgebra/linalg/svd.rs.html#585
    target.transpose() * source.transpose().pseudo_inverse(1e-15).unwrap()
}

fn read_matrix(filename: &Path) -> Result<DMatrix<f64>, MatrixError> {
    let raw = fs::read_to_string(filename)?;
    let mut parsed = Vec::new();
    for line in raw.lines() {
        let mut parts = line.split_whitespace();
        for _ in 0..3 {
            parsed.push(
                parts
                    .next()
                    .ok_or(MatrixError::NotATriplet)?
                    .parse::<f64>()?,
            );
        }
    }
    Ok(DMatrix::from_vec(3, parsed.len() / 3, parsed).transpose())
}

fn main() {
    let mut args = env::args().skip(1).map(|arg| {
        if matches!(arg.as_str(), "--help" | "-h") {
            println!("{}", HELP);
            process::exit(0);
        }
        arg
    });

    let source_path = args
        .next()
        .ok_or("Missing source dataset")
        .unwrap_or_display();
    let target_path = args
        .next()
        .ok_or("Missing target dataset")
        .unwrap_or_display();

    let source = read_matrix(Path::new(&source_path)).unwrap_or_display();
    let target = read_matrix(Path::new(&target_path)).unwrap_or_display();

    let solve = solve_matrix(source, target);
    println!(
        "{} {} {}\n{} {} {}\n{} {} {}",
        solve[(0, 0)],
        solve[(0, 1)],
        solve[(0, 2)],
        solve[(1, 0)],
        solve[(1, 1)],
        solve[(1, 2)],
        solve[(2, 0)],
        solve[(2, 1)],
        solve[(2, 2)],
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::dmatrix;

    #[test]
    fn parse_matrix() {
        let actual = read_matrix(Path::new("datasets/source_small.txt")).unwrap();
        let expected = dmatrix![
            0.535, 0.382, 0.344;
            0.472, 0.510, 0.434;
            0.371, 0.388, 0.476;
            0.476, 0.525, 0.557;
            0.574, 0.474, 0.516;
            0.631, 0.597, 0.463;
        ];
        assert_eq!(actual, expected);
    }
}
