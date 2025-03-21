#![allow(dead_code, unused_imports, unused_variables, 
         unused_mut, non_snake_case, deprecated)]

extern crate vectors;

use vectors::*;
use std::ops::{Add, Sub, Mul};
use std::fmt;
use std::cmp::min;
use std::rc::Rc;
use rand::Rng;
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    rows: Vec<Vector>,
    columns: Vec<Vector>,
}

impl Matrix {
    pub fn new(numbers: &[&[f64]]) -> Self {
        if numbers.len() == 0 {
            panic!("tried to create a matrix with zero rows");
        }

        let mut rows: Vec<Vector> = vec![];
        let fixed_size = numbers[0].len();

        for &row in numbers.iter() {
            assert!(row.len() == fixed_size,
            "tried to create a matrix using rows of different sizes");

            let mut components = vec![];

            for &number in row.iter() {
                components.push(number);
            }

            rows.push(Vector::new(&components));
        }

        let columns = Self::create_columns(numbers);

        Self {
            rows,
            columns,
        }
    }

    pub fn ortogonal_projector(&self, vector: bool) -> Self {
        if vector {
            return &(&self.transpose() * self) *
            (1.0 / (self * &self.transpose()).element(1, 1).unwrap());
        }

        let s = Spaces::new();

        let range_dimension = s.column_dimension(self);
        let (_, n) = self.dimension();

        if range_dimension != n {
            panic!("there is no ortogonal projector");
        }

        let tranpose = self.transpose();
        let t_m = &tranpose * self;
        let t_m_inverse = t_m.inverse();

        self * &( &t_m_inverse * &self.transpose())
    }

    pub fn zero(rows: usize, columns: usize) -> Self {
        let result = (0..rows)
                .map(|i| Vector::zero(columns))
                .collect::<Vec<Vector>>();

        Self::create_with_vectors(&result)
    }

    pub fn is_zero(&self) -> bool {
        for row in self.rows.iter() {
            if !row.is_zero() {
                return false;
            }
        }

        true
    }

    pub fn identity(size: usize) -> Self {
        let mut result = vec![];

        for i in 0..size {
            let mut components = vec![];

            for j in 0..size {
                if i == j {
                    components.push(1.0);
                } else {
                    components.push(0.0);
                }
            }

            result.push(Vector::new(&components));
        }

        Matrix::create_with_vectors(&result)
    }

    pub fn sign(&self, i: usize, j: usize) -> f64 {
        assert!(i > 0 && j > 0);
        let (m, n) = self.dimension();
        assert!(i <= m && j <= n);

        let element = self.element(i, j).unwrap();

        if element == 0.0 {
            return 0.0;
        } else if element < 0.0 {
            return -1.0;
        } else {
            return 1.0;
        }
    }

    pub fn create_with_vectors(rows: &[Vector]) -> Self {
        let mut columns: Vec<Vector> = vec![];
        let row_length = rows[0].size();

        for j in 0..row_length {
            let mut components = vec![];

            for i in 0..rows.len() {
                components.push(
                    rows[i].element(j+1).expect(
                    "tried to create a matrice using rows of different sizes"
                    )
                )
            }

            columns.push(Vector::new(&components));
        }

        Self {
            rows: rows.to_vec(),
            columns,
        }
    }

    fn create_columns(numbers: &[&[f64]]) -> Vec<Vector> {
        let mut columns: Vec<Vector> = vec![];
        let row_length = numbers[0].len();

        for j in 0..row_length {
            let mut components = vec![];

            for i in 0..numbers.len() {
                components.push(numbers[i][j]);
            }

            columns.push(Vector::new(&components));
        }

        columns
    }

    pub fn sub_matrix(&self, (i_i, i_e) : (usize, usize), (j_i, j_e) : (usize, usize)) -> Matrix {
        let (m, n) = (i_e - i_i + 1, j_e - j_i + 1);
        let mut sub_matrix = Matrix::zero(m, n);

        for i in i_i..=i_e {
            for j in j_i..=j_e {
                if let Some(element) = self.element(i, j) {
                    sub_matrix.change_element(i - i_i + 1, j - j_i + 1, element);
                }
            }
        }

        sub_matrix
    }

    pub fn sub_vector(&self, (i_i, i_e): (usize, usize), k: usize) -> Vector {
        let m = i_e - i_i + 1;
        let mut sub_vec = Vector::zero(m);

        for i in i_i..=i_e {
            if let Some(element) = self.element(i, k) {
                sub_vec.change_element(i - i_i + 1, element);
            }
        }

        sub_vec
    }

    pub fn multiply_by_sub
    (   &mut self, 
        (i_i, i_e): (usize, usize),
        (j_i, j_e): (usize, usize),
        other: &Matrix,       
    )
    {
        let sub_matrix = self.sub_matrix((i_i, i_e), (j_i, j_e));
        let result = other * &sub_matrix;

        for i in i_i..=i_e {
            for j in j_i..=j_e {
                if let Some(element) = result.element(i - i_i + 1, j - j_i + 1) {
                    self.change_element(i, j, element);
                }
            }
        }
    }

    pub fn subtract_by_sub(
        &mut self, 
        (i_i, i_e): (usize, usize),
        (j_i, j_e): (usize, usize),
        other: &Matrix,       
    )
    {   
        for i in i_i..=i_e {
            for j in j_i..=j_e {
                let element_self = self.element(i, j).unwrap();
                let element_other = other.element(i - i_i + 1, j - j_i + 1).unwrap();

                self.change_element(i, j, element_self - element_other);
            }
        }
    }

    fn change_rows_columns(&mut self) {
        let mut rows: Vec<Vector> = vec![];
        let mut columns: Vec<Vector> = vec![];
        let (m, n) = self.dimension();

        for j in 1..=n {
            let mut components = vec![];

            for i in 1..=m {
                components.push(self.element(i, j).unwrap());
            }

            columns.push(Vector::new(&components));
        }

        for i in 1..=m {
            let mut components = vec![];

            for j in 1..=n {
                components.push(self.element(i, j).unwrap());
            }

            rows.push(Vector::new(&components));
        }

        self.rows = rows;
        self.columns = columns;
    }

    pub fn get_line(&self, position: usize) -> Option<Vector> {
        assert!(position > 0, "position must be greater than zero");

        match self.rows.get(position - 1) {
            Some(&ref vector) => Some(vector.clone()),
            None => None,
        }
    }

    pub fn get_column(&self, position: usize) -> Option<Vector> {
        assert!(position > 0, "position must be greater than zero");

        match self.columns.get(position -1) {
            Some(&ref vector) => Some(vector.clone()),
            None => None,
        }
    }

    pub fn change_column(&mut self, position: usize, column: Vector) {
        let (m, n) = self.dimension();
        assert!(column.size() == m);

        for i in 1..=m {
            self.change_element(i, position, column.element(i).unwrap());
        }

    }

    pub fn dimension(&self) -> (usize, usize) {
        (self.rows.len(), self.rows[0].size())
    }

    pub fn transpose(&self) -> Self {
        let cloned = self.clone();

        let result = cloned.columns.into_iter()
            .map(|vector| vector)
            .collect::<Vec<Vector>>();

        Matrix::create_with_vectors(&result)
    }

    pub fn element(&self, row: usize, column: usize) -> Option<f64> {
        let (r, c) = self.dimension();

        assert!(row > 0 && column > 0, 
                "either row or column is non-positive");

        assert!(row <= r && column <= c,
                "either row or column is greater than the limit");

        if let Some(vector) = self.get_line(row) {
            if let Some(number) = vector.element(column) {
                return Some(number);
            } else {
                return None;
            }
        } else {
            return None
        }
    }

    pub fn get_sub_column(&self, init: usize, end: usize, column: usize) -> Vector {
        let (m, n) = self.dimension();
        assert!(init <= end && init > 0  && end > 0 && column > 0);
        assert!(init <= m && end <= m && column <= n);

        let mut vector = Vector::zero(end - init + 1);

        for i in init..=end {
            let element = self.element(i, column).unwrap();
            vector.change_element(i + 1 - init, element);
        }

        vector
    }

    pub fn create_with_diagonal(matrices: &[Matrix]) -> Matrix {
        let mut m = 0;
        let mut n = 0;

        for matrix in matrices {
            let (x, y) = matrix.dimension();
            m += x;
            n += y;
        }

        let mut new_matrix = Matrix::zero(m, n);

        let (mut x, mut y) = (1, 1);
        for i in 0..matrices.len() {
            let (x1, y1) = matrices[i].dimension();

            new_matrix.subtract_by_sub((x, x + x1 - 1), (y, y + y1 - 1), 
            &(-1.0*&matrices[i]));
            x += x1;
            y += y1;
        }

        new_matrix
    }

    pub fn change_element
    (
        &mut self, 
        row: usize, 
        column: usize,
        element: f64,
    )
    {
        let (r, c) = self.dimension();

        assert!(row > 0 && column > 0, 
                "either row or column is non-positive");

        assert!(row <= r && column <= c,
                "either row or column is greater than the limit");

        self.rows[row - 1].change_element(column, element);
        self.change_rows_columns();
    }   

    pub fn inverse(&self) -> Matrix {
        let (m, n) = self.dimension();

        if m != n {
            panic!("matrix is not square");
        }

        let eliminator = Eliminator::new();
        let spaces = Spaces::new();

        if spaces.null_dimension(self) != 0 {
            panic!("matrix is singular");
        }

        let mut cloned = self.clone();
        let (_, inverse) = eliminator.rref(&mut cloned);

        inverse
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")?;
        for (i, vector) in self.rows.iter().enumerate() {
            if i == self.rows.len() - 1 {
                write!(f, "{}", vector)?;
            } else {
                write!(f, "{}\n", vector)?;
            }
        }
        write!(f, "")
    }
}

impl Add<&Matrix> for &Matrix {
    type Output = Matrix;

    fn add(self, other: &Matrix) -> Matrix {
        let length = self.rows.len();

        if length != other.rows.len() {
            panic!("tried to add matrices of different sizes");
        }

        let result = self.rows.iter()
                    .zip(other.rows.iter())
                    .map(|(vector1, vector2)| vector1 + vector2)
                    .collect::<Vec<Vector>>();

        Matrix::create_with_vectors(&result)
    }
}

impl Sub<&Matrix> for &Matrix {
    type Output = Matrix;

    fn sub(self, other: &Matrix) -> Matrix {
        let length = self.rows.len();

        if length != other.rows.len() {
            panic!("tried to sub matrices of different sizes");
        }

        let result = self.rows.iter()
                    .zip(other.rows.iter())
                    .map(|(vector1, vector2)| vector1 - vector2)
                    .collect::<Vec<Vector>>();

        Matrix::create_with_vectors(&result)
    }
}

impl Mul<f64> for &Matrix {
    type Output = Matrix;

    fn mul(self, other: f64) -> Matrix {
        let result = self.rows.iter()
                    .map(|vector| vector * other)
                    .collect::<Vec<Vector>>();

        Matrix::create_with_vectors(&result)
    }
}

impl Mul<&Matrix> for f64 {
    type Output = Matrix;

    fn mul(self, other: &Matrix) -> Matrix {
        let result = other.rows.iter()
                    .map(|vector| vector * self)
                    .collect::<Vec<Vector>>();
                    
        Matrix::create_with_vectors(&result)
    }
}

impl Mul<&Vector> for &Matrix {
    type Output = Vector;

    fn mul(self, other: &Vector) -> Self::Output {
        let matrix_columns = self.dimension().1;
        let vector_size = other.size();

        assert!(vector_size == matrix_columns, 
                "invalid multiplication");

        let result = self.rows.iter()
                     .map(|vector| vector.dot_product(&other))
                     .collect::<Vec<f64>>();

        Vector::create_with_vec(result)
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, other: &Matrix) -> Self::Output {
        let (r1, c1) = self.dimension();
        let (r2, c2) = other.dimension();

        assert!(c1 == r2, "invalid multiplication");

        let result = other.columns.iter()
                     .map(|vector| self * vector)
                     .collect::<Vec<Vector>>();

        (Matrix::create_with_vectors(&result)).transpose()
    }
}

pub struct Eliminator {}

impl Eliminator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn swap_rows
    (
        &self,
        row1: usize,
        row2: usize,
        matrix: &mut Matrix,
    )
    {
        let (rows, _) = matrix.dimension();

        assert!(row1 <= rows && row2 <= rows && row1 > 0 && row2 > 0);

        let mut permutation_matrix = Matrix::identity(rows);
        permutation_matrix.change_element(row1, row1, 0.0);
        permutation_matrix.change_element(row1, row2, 1.0);
        permutation_matrix.change_element(row2, row2, 0.0);
        permutation_matrix.change_element(row2, row1, 1.0);

        let cloned = matrix.clone();

        *matrix = &permutation_matrix * &cloned;
    }

    pub fn multiply_row_by_number
    (
        &self,
        row: usize,
        factor: f64,
        matrix: &mut Matrix,
    )
    {
        let (rows, _) = matrix.dimension();

        assert!(row <= rows);
        assert!(row > 0);

        let mut elimination_matrix = Matrix::identity(rows);
        elimination_matrix.change_element(row, row, factor);

        let cloned = matrix.clone();

        *matrix = &elimination_matrix * &cloned;
    }

    pub fn multiply_rows // re = re - factor * ru
    (   
        &self,
        re: usize, 
        ru: usize,
        fu: f64,
        matrix: &mut Matrix,
    )
    {   
        let (rows, _) = matrix.dimension();

        assert!(re <= rows && ru <= rows);
        assert!(re > 0 && ru > 0);
        assert!(re != ru);

        let mut elimination_matrix = Matrix::identity(rows);
        elimination_matrix.change_element(re, ru, -fu);

        let cloned = matrix.clone();

        *matrix = &elimination_matrix * &cloned;
    }

    pub fn row_echelon_form
    (
        &self,
        matrix: &mut Matrix
    ) -> (Vec<(usize, usize)>, Matrix)
    {
        let mut h = 1;
        let mut k = 1;

        let (m, n) = matrix.dimension();
        let mut leading_ones = vec![];
        let mut to_inverse = Matrix::identity(m);

        while h <= m && k <= n {
            let column = matrix.get_column(k).unwrap();
            let mut index = 0;

            for i in (h-1)..n {
                if m < n && i == n - 1 {
                    break;
                }

                if column.components[i] != 0.0 {
                    index = i + 1;
                    break;
                }
            }

            match index {
                0 => k += 1,
                _ => {
                    self.swap_rows(index, h, matrix);

                    if m == n {
                        self.swap_rows(index, h, &mut to_inverse);
                    }

                    let pivot = matrix.element(h, k).unwrap();
                    leading_ones.push((h, k));

                    for a in (h + 1)..=m {
                        let factor = matrix.element(a, k).unwrap()
                                     / pivot;

                        self.multiply_rows(a, h, factor, matrix);

                        self.multiply_rows(a, h, factor, &mut to_inverse);
                    }

                    h += 1;
                    k += 1;
                }
            }
        }

        (leading_ones, to_inverse)
    }

    pub fn rref(&self, matrix: &mut Matrix) -> (Vec<(usize, usize)>, Matrix) {
        let (mut leading_ones, mut to_inverse) = self.
                            row_echelon_form(matrix);
        let pivot_indexes = leading_ones.clone();
        let (m, n) = matrix.dimension();

        for _ in 0..(leading_ones.len()) {
            let most_right_index = leading_ones.pop();
            let Some((i, j)) = most_right_index else { panic!("") };

            let pivot = matrix.element(i, j).unwrap();

            for a in 1..i {
                let element_above = matrix.element(a, j).unwrap();
                let factor = element_above / pivot;

                self.multiply_rows(a, i, factor, matrix);

                self.multiply_rows(a, i, factor, &mut to_inverse);
            }

            self.multiply_row_by_number(i, 1.0 / pivot, matrix);

            self.multiply_row_by_number(i, 1.0 / pivot, &mut to_inverse);
        }

        (pivot_indexes, to_inverse)
    }
}

pub struct Spaces {}

impl Spaces {
    pub fn new() -> Self {
        Self {}
    }

    fn build_identity(&self, matrix: &mut Matrix) {

    }

    fn check_identity
    (
        &self, 
        matrix: &mut Matrix,
        pivot_indexes: &Vec<(usize, usize)>
    )
    {
        let (bi, bj) = pivot_indexes[0];
        let (bi, bj) = (bi as i64, bj as i64);

        for (i, j) in pivot_indexes.iter() {
            let (i, j) = (*i as i64, *j as i64);
            if bi == i && bj == j {
                continue;
            }

            if (bj - j).abs() != (bi - i).abs() {
                self.build_identity(matrix);
            }
        }
    }

    pub fn null_dimension(&self, matrix: &Matrix) -> usize {
        matrix.dimension().1 - self.column_dimension(matrix)
    }

    pub fn column_dimension(&self, matrix: &Matrix) -> usize {
        self.column_space(matrix).len()
    }

    pub fn null_space(&self, matrix: &Matrix) -> Vec<Vector> {
        let mut eliminator = Eliminator::new();
        let mut cloned_matrix = matrix.clone();
        let (m, n) = cloned_matrix.dimension();
        let (pivot_indexes, _) = eliminator.rref(&mut cloned_matrix);

        let r = pivot_indexes.len(); // dimension of the column space

        if n - r == 0 {
            return vec![];
        }
        
        self.check_identity(&mut cloned_matrix, &pivot_indexes);

        let mut F = Matrix::zero(n, n - r);

        for j in (r+1)..=n {
            for i in 1..=r {
                let row_index = i;
                let column_index = j - r;

                let element = -cloned_matrix.element(i, j).unwrap();
                F.change_element(row_index, column_index, element);
            } 
        }

        for i in (r+1)..=n {
            for j in 1..=(n - r) {
                if (i - r) == j {
                    F.change_element(i, j, 1.0);
                }
            }
        }

        F.columns
    }

    pub fn column_space(&self, matrix: &Matrix) -> Vec<Vector> {
        let mut eliminator = Eliminator::new();
        let mut cloned = matrix.clone();
        let (pivots, _) = eliminator.row_echelon_form(&mut cloned);
        let mut basis = vec![];

        for (_, j) in pivots {
            basis.push(matrix.get_column(j).unwrap());
        }  

        basis
    }
}

pub struct Solver {}

impl Solver {
    pub fn new() -> Self {
        Self {}
    }

    fn concatenate(&self, a: &Matrix, b: &Matrix) -> Matrix {
        let (r1, c1) = a.dimension();
        let (r2, c2) = b.dimension();

        assert!(r1 == r2, "matrices with different dimensions");

        let mut new = Matrix::zero(r1, c1 + c2);

        for i in 1..=r1 {
            for j in 1..=(c1 + c2) {
                if j <= c1 {
                    let element = a.element(i, j).unwrap();
                    new.change_element(i, j, element);
                } else {
                    let element = b.element(i, j - c1).unwrap();
                    new.change_element(i, j, element);
                }
            }
        }

        new
    }

    pub fn solve(&self, matrix: &Matrix, b: &Vector) -> Vector {
        let (m, n) = matrix.dimension();
        assert!(m == b.size());

        let b_as_matrix = Matrix::create_with_vectors(&[b.clone()]).transpose();
        let mut augmented_matrix = self.concatenate(matrix, &b_as_matrix);

        let eliminator = Eliminator::new();
        let (_, _) = eliminator.rref(&mut augmented_matrix);

        let mut solution = Vector::zero(n);
        let part_solution = augmented_matrix.get_column(n + 1).unwrap();

        for i in 1..=n {
            let element = part_solution.element(i).unwrap();
            solution.change_element(i, element);
        }

        assert!(matrix * &solution == *b, "there is no solution");

        solution
    }

    pub fn generic_solve(&self, matrix: &Matrix, b: &Vector) -> Vector {
        let spaces = Spaces::new();
        let null_basis = spaces.null_space(matrix);
        let particular_solution = self.solve(matrix, b);

        let mut rng = rand::thread_rng();
        let mut solution = particular_solution;

        if null_basis.len() > 0 {
            for vector in null_basis.iter() {
                let random_number = rng.gen_range(-1.0..1.0);

                solution = &solution + &(random_number * vector);
            }
        }

        solution
    }

    pub fn foward_substitution(&self, matrix: &Matrix, b: &Vector) -> Vector {
        // apenas para matrizes triangulares inferiores positivas definidas

        let n = matrix.dimension().0;
        let mut x = Vector::zero(n);

        for i in 1..=n {
            let mut sum = 0.0;
            let bi = b.element(i).unwrap();
            let aii = matrix.element(i, i).unwrap();

            if aii == 0.0 {
                panic!("bla bla bla");
            }

            for j in 1..=i-1 {
                let aij = matrix.element(i, j).unwrap();
                let xj = x.element(j).unwrap();
                sum += aij * xj;
            }

            x.change_element(i, (bi - sum) / aii);
        }

        x
    }

    pub fn backward_substitution(&self, matrix: &Matrix, b: &Vector) -> Vector {
        let n = matrix.dimension().1;
        let mut x = Vector::zero(n);
        let mut i = n;

        loop {
            if i <= 0 {
                break x
            }

            let mut sum = 0.0;
            let bi = b.element(i).unwrap();
            let uii = matrix.element(i, i).unwrap();

            if uii == 0.0 {
                panic!("bla bla bla");
            }

            for j in (i+1)..=n {
                let uij = matrix.element(i, j).unwrap();
                let xj = x.element(j).unwrap();
                sum += uij * xj;
            }

            x.change_element(i, (bi - sum) / uii);
            i -= 1;
        }
    }

    pub fn jacobi
    // apenas para matrizes quadradas
    (
        &self, 
        matrix: &Matrix, 
        b: &Vector, 
        error: f64,
        max_iter: usize,
    ) -> Vector {
        let (m, n) = matrix.dimension();
        assert!(m == n);

        let mut x = Vector::zero(n);
        let mut x_new = x.clone();

        for _ in 0..max_iter {
            x_new.components.par_iter_mut()
            .enumerate().for_each(|(i, x_new_i)| {
                let mut sum = 0.0;
                let i = i + 1;

                for j in 1..=n {
                    if i != j {
                        sum += matrix.element(i, j).unwrap() 
                        * x.element(j).unwrap();
                    }
                }

                let element = (b.element(i).unwrap() - sum) 
                / matrix.element(i, i).unwrap();
                *x_new_i = element;
            });

            if x.components.iter().zip(&x_new.components)
                .map(|(xi, xni)| (xi - xni).abs()).sum::<f64>() < error {
                break;
            }

            x = x_new.clone();
        }

        x
    }

    pub fn gauss_seidel
    // apenas para matrizes quadradas
    (
        &self,
        matrix: &Matrix,
        b: &Vector,
        error: f64,
        max_iter: usize,
    ) -> Vector
    {
        let (m, n) = matrix.dimension();
        assert!(m == n);

        let mut x = Vector::zero(n);
        let mut x_new = x.clone();

        for _ in 0..max_iter {
            for i in 1..=n {
                let mut sum = 0.0;

                for j in 1..=n {
                    if i != j {
                        sum += matrix.element(i, j).unwrap()
                               * x_new.element(j).unwrap();
                    }
                }

                let element = (b.element(i).unwrap() - sum)
                / matrix.element(i, i).unwrap();
                x_new.change_element(i, element);
            }

            if x.components.iter().zip(&x_new.components)
                .map(|(xi, xni)| (xi - xni).abs()).sum::<f64>() < error {
                break;
            }

            x = x_new.clone();
        }

        x
    }
}

pub struct Decomposer {}

pub struct Houlseholder {
    R: Matrix,
    vectors: Vec<Matrix>,
}

impl Houlseholder {
    fn new(R: Matrix, vectors: Vec<Matrix>) -> Self {
        Self {
            R,
            vectors,
        }
    }

    pub fn get_q(self) -> Matrix {
        let (m, n) = self.R.dimension();
        let mut q_transpose = Matrix::identity(m);

        for k in 1..=min(n, m) {
            let ort_proj = self.vectors[k-1].ortogonal_projector(true);
            let I = Matrix::identity(m - k + 1);
            let F = &I - &(2.0 * &ort_proj);
            let qk = if k == 1 {
                F
            } else {
                Matrix::create_with_diagonal(&[Matrix::identity(k-1),
                     F])
            };

            q_transpose = &qk * &q_transpose;
        }

        q_transpose.transpose()
    }

    pub fn get_r(&self) -> Matrix {
        self.R.clone()
    }
}

impl Decomposer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn gram_schmidt(&self, vectors: Vec<Vector>) -> Vec<Vector> {
        let n = vectors.len();
        let mut ort_vectors: Vec<Vector> = vec![];

        for i in 0..n {
            let actual_vector = &vectors[i];
            let mut ort_vec = vectors[i].clone();

            for j in 0..i {
                if i == 0 {
                    continue;
                }

                let qj = &ort_vectors[j];
                let dot = qj.dot_product(&actual_vector);
                let proj = dot * qj;
                ort_vec = &ort_vec - &proj;
            }

            ort_vec = ort_vec.normalize();
            ort_vectors.push(ort_vec);
        }

        ort_vectors
    }

    pub fn classical_gs_naive(&self, matrix: &Matrix) -> (Matrix, Matrix) {
        let (m, n) = matrix.dimension();
        let mut Q = matrix.clone();
        let mut R = Matrix::zero(n, n);

        for j in 1..=n {
            let mut v = Q.get_column(j).unwrap();

            for i in 1..=j-1 {
                let w = Q.get_column(i).unwrap();
                let new_element = w.dot_product(&matrix.get_column(j)
                                   .unwrap());
                R.change_element(i, j, new_element);

                v = &v - &(R.element(i, j).unwrap() * &w);
            }

            R.change_element(j, j, v.magnitude());
            Q.change_column(j, v.normalize());
        }

        (Q, R)
    }

    pub fn modified_gs(&self, matrix: &Matrix) -> (Matrix, Matrix) {
        let (m, n) = matrix.dimension();
        let mut Q = matrix.clone();
        let mut R = Matrix::zero(n, n);

        for j in 1..=n {
            let v = Q.get_column(j).unwrap();
            R.change_element(j, j, v.magnitude());
            Q.change_column(j, v.normalize());

            let w = Q.get_column(j).unwrap();
            for i in (j+1)..=n {
                R.change_element(j, i, w.dot_product(&matrix.get_column(i)
                                .unwrap()));
                Q.change_column(i, &Q.get_column(i).unwrap() - 
                    &(R.element(j, i).unwrap() * &w));
            }
        }

        (Q, R)
    }

    pub fn householder(&self, matrix: &Matrix) -> Houlseholder {
        // muito mais ineficiente que classical_gs_naive
        // parece que as funções que lidam com sub matrizes
        // não são eficientes o bastante
        // porem modified_gs nao lida com sub matrizes e mesmo
        // assim tem uma eficiencia parecida com householder
        // então é escolher entre estabilidade numerica ou eficiencia

        let (m, n) = matrix.dimension();
        let mut R = matrix.clone();
        let mut vectors: Vec<Matrix> = vec![];

        for k in 1..=min(n, m) {
            let x = matrix.sub_vector((k, m), k);
            let c = x.sign(1) * x.magnitude();
            let mut vk = &(c * &Vector::canonical(1, m - k + 1)) + &x;
            vk = vk.normalize();

            let mut proj = R.sub_matrix((k, m), (k, n));
            let vk_matrix = Matrix::create_with_vectors(&[vk]);
            proj = 2.0 * &(&vk_matrix.transpose() * &(&(vk_matrix) * &proj));

            R.subtract_by_sub((k, m), (k, n), &proj);
            vectors.push(vk_matrix);
        }

        let k = Houlseholder::new(R, vectors);
        k
    }


    pub fn QR_nyoxon(&self, matrix: &Matrix) -> (Matrix, Matrix) {
        // tá errada
        let mut vectors = vec![];

        for i in 1..=(matrix.dimension().1) {
            vectors.push(matrix.get_column(i).unwrap());
        }

        let g_schmidt_result = self.gram_schmidt(vectors);
        let Q = Matrix::create_with_vectors(&g_schmidt_result);
        let R = &Q * matrix;

        (Q.transpose(), R)
    }

    pub fn LU(&self, matrix: &Matrix) -> (Matrix, Matrix) {
        // procurar um metodo mais eficiente talvez
        let eliminator = Eliminator::new();
        let mut mutable_matrix = matrix.clone();
        let (_, E) = eliminator.row_echelon_form(&mut mutable_matrix);

        (E.inverse(), mutable_matrix)
    }

    pub fn cholesky(&self, matrix: &Matrix) -> Matrix {
        // para matrizes positivas semi definidas
        // como nao tenho um algoritmo para verificar isso
        // apenas verifica se a matriz é simetrica
        assert!(*matrix == matrix.transpose());

        let n = matrix.dimension().0;
        let mut L = Matrix::zero(n, n);

        for i in 1..=n {
            for j in 1..=i {
                let mut sum  = 0.0;
                if i == j {
                    let aii = matrix.element(i, i).unwrap();
                    for k in 1..=i-1 {
                        let lik = L.element(i, k).unwrap();
                        sum += lik * lik;
                    }
                    let new_element = (aii - sum).sqrt();
                    L.change_element(i, i, new_element);
                } else {
                    let aij = matrix.element(i, j).unwrap();
                    let ljj = L.element(j, j).unwrap();
                    for k in 1..=j-1 {
                        let lik = L.element(i, k).unwrap();
                        let ljk = L.element(j, k).unwrap();
                        sum += lik * ljk;
                    }
                    let new_element = (aij - sum) / ljj;
                    L.change_element(i, j, new_element); 
                }
            }
        }

        L
    }
}

pub struct LeastSquares {}

impl LeastSquares {
    pub fn new() -> Self {
        Self {}
    }

    pub fn via_cholesky(&self, matrix: &Matrix, b: &Vector) -> Vector {
        // calculate the least squares solution
        // using the cholesky factorization of the matrix.

        let aTa = &matrix.transpose() * matrix;
        let aTb = &matrix.transpose() * b;

        let decomposer = Decomposer::new();
        let solver = Solver::new();
        let cholesky = decomposer.cholesky(&aTa);
        let w = solver.foward_substitution(&cholesky, &aTb);
        let x = solver.backward_substitution(&cholesky.transpose(), &w);

        x
    }

    pub fn via_qr(&self, matrix: &Matrix, b: &Vector) -> Vector {
        // calculate the least squares solution
        // using its reduced qr factorization

        let decomposer = Decomposer::new();
        let solver = Solver::new();
        let (Q, R) = decomposer.classical_gs_naive(&matrix);
        let qTb = &Q.transpose() * b;
        let x = solver.backward_substitution(&R, &qTb);

        x
    }
}