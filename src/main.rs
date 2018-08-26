extern crate rustml;

mod sparse_symmetric_matrix;
mod sparse_row_matrix;
mod vector;
mod preconditioner;
mod solver;

use sparse_symmetric_matrix::{SparseSymmetricMatrix, Entry};
use vector::Vector;
use solver::solver;

fn main() {
    let m = SparseSymmetricMatrix::new(&vec![
        Entry { x: 0, y: 0, v: 1.0 },
        Entry { x: 0, y: 1, v: 5.0 },
        Entry { x: 0, y: 2, v: 6.0 },
        Entry { x: 1, y: 1, v: 2.0 },
    ]);
    let v: Vector = Vector(vec![5.0, 6.0, 7.0]);
    let result = solver(&m, &v);
    assert_eq!(result.completed, true);
    assert_eq!(result.iterations, 2);
    assert_eq!(result.best_guess.0[0], 1.1666674087694608);
    assert_eq!(result.best_guess.0[1], 0.0833110800778692);
    assert_eq!(result.best_guess.0[2], 0.5694629884317245);
}

#[cfg(test)]
mod tests {
    use rustml::octave::*;
    use std::process::Command;

    use sparse_symmetric_matrix::{SparseSymmetricMatrix, Entry};
    use sparse_row_matrix::SparseRowMatrix;
    use vector::Vector;

    #[test]
    fn test_sparse_multiplication() {
        let m = SparseSymmetricMatrix::new(&vec![
            Entry { x: 0, y: 0, v: 1.5 },
            Entry { x: 0, y: 1, v: 5.5 },
            Entry { x: 0, y: 2, v: 6.5 },
            Entry { x: 1, y: 1, v: 2.5 },
            Entry { x: 1, y: 2, v: 8.5 },
            Entry { x: 2, y: 2, v: 9.5 },
        ]);
        let v: Vector = Vector(vec![3.0, 2.0, 1.0]);
        let v2 = v.0.iter().cloned().collect::<Vec<f64>>();
        let srm = SparseRowMatrix::new(&m);
        let result = srm.apply(&v);

        let eval_str = format!("disp({:?} * $$')", m);
        let s = builder().add_vector(&eval_str, &v2);

        let filename = "test.octave";
        assert!(s.run(filename).is_ok());
        let output = Command::new("octave").arg(filename).output().expect(
            "octave failed to start",
        );
        let final_string = format!(
            "   {:.4}\n   {:.4}\n    {:.4}\n",
            result.0[0],
            result.0[1],
            result.0[2]
        );
        println!("{}", final_string);
        assert_eq!(final_string.as_bytes(), output.stdout.as_slice());
    }
}
