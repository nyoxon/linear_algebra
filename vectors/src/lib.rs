#![allow(dead_code, unused_imports, unused_variables, unused_mut)]

use std::ops::{Add, Sub, Mul};
use std::f64::consts::PI;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    pub components: Vec<f64>,
}

impl Vector {
    pub fn new(numbers: &[f64]) -> Self {
        assert!(numbers.len() > 0, "tried to create a vector with no components");

        let mut components: Vec<f64> = vec![];

        for &number in numbers.iter() {
            components.push(number);
        }

        Self {
            components,
        }
    }

    pub fn create_with_vec(numbers: Vec<f64>) -> Self {
        Self {
            components: numbers,
        }
    }

    pub fn zero(size: usize) -> Vector {
        let result = (0..size)
                    .map(|i| 0.0)
                    .collect::<Vec<f64>>();

        Vector::new(&result)
    }

    pub fn is_zero(&self) -> bool {
        self.components
            .iter()
            .all(|&i| i == 0.0)
    }

    pub fn dimension(&self) -> (usize, usize) {
        (1, self.components.len())
    }

    pub fn element(&self, position: usize) -> Option<f64> {
        assert!(position > 0, "position must be greater than zero");
        assert!(position <= self.size(), "position must be less or equal than
            vector size");

        match self.components.get(position - 1) {
            Some(&element) => Some(element),
            None => None,
        }
    }

    pub fn sign(&self, position: usize) -> f64 {
        if let Some(element) = self.element(position) {
            if element >= 0.0 {
                return 1.0;
            } else {
                return -1.0;
            }
        } else {
            panic!("bla blu tomou no cu");
        }
    }

    pub fn canonical(index: usize, size: usize) -> Vector {
        assert!(size > 0);
        assert!(index > 0);
        assert!(index <= size);

        let mut e = Vector::zero(size);
        e.change_element(index, 1.0);

        e
    }

    pub fn change_element
    (
        &mut self, 
        position: usize, 
        element: f64
    )
    {
        assert!(position > 0, "position must be greater than zero"); 
        assert!(position <= self.size(), "position must be less or equal than
            vector size");

        self.components[position - 1] = element;
    }

    pub fn size(&self) -> usize {
        self.components.len()
    }

    pub fn inverse(&self) -> Self {
        self * (-1.0)
    }

    fn check_sizes(&self, other: &Self) -> bool {
        self.components.len() == other.components.len()
    }

    pub fn dot_product(&self, other: &Self) -> f64 {
        let length = self.components.len();

        assert!(self.check_sizes(other),
        "tried to calculate the dot product of vectors of different sizes");

        self.components.iter()
            .zip(other.components.iter())
            .map(|(num1, num2)| num1 * num2)
            .sum::<f64>()
    }

    pub fn magnitude(&self) -> f64 {
        self.dot_product(self).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        let mut components = self.components
                        .iter()
                        .map(|num| num / magnitude)
                        .collect::<Vec<_>>();

        Self::new(&components)
    }

    pub fn angle(&self, other: &Self) -> f64 {
        assert!(self.check_sizes(other),
        "tried to calculate the angle between vectors of different sizes");

        (self.dot_product(other) / 
        (self.magnitude() * other.magnitude())).acos()
    }

    pub fn angle_degrees(&self, other: &Self) -> f64 {
        self.angle(other) * 180.0 / PI
    }

    pub fn proj(&self, other: &Self) -> Self {
        other * ( (self.dot_product(other)) / (other.dot_product(other)))
    }

    pub fn max_index(&self, init: usize, fin: usize) -> usize {
        assert!(fin <= self.size());

        let mut max = 0.0;
        let mut index = 0;

        for i in init..fin {
            if self.components[i].abs() > max {
                max = self.components[i].abs();
                index = i;
            }
        }

        index
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, value) in self.components.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:.5}", value)?;
        }
        write!(f, "]")
    }
}

impl Add<&Vector> for &Vector {
    type Output = Vector;

    fn add(self, other: &Vector) -> Vector {
        let length = self.components.len();

        if length != other.components.len() {
            panic!("tried to add vectors of different sizes");
        }

        let mut result = vec![];

        for i in 0..length {
            result.push(self.components[i] + other.components[i]);
        }

        Vector::new(&result)
    }
}

impl Sub<&Vector> for &Vector {
    type Output = Vector;

    fn sub(self, other: &Vector) -> Vector {
        let length = self.components.len();

        if length != other.components.len() {
            panic!("tried to sub vectors of different sizes");
        }

        let mut result = vec![];

        for i in 0..length {
            result.push(self.components[i] - other.components[i]);
        }

        Vector::new(&result)
    }
}

impl Mul<f64> for &Vector {
    type Output = Vector;

    fn mul(self, other: f64) -> Vector {
        let length = self.components.len();
        let mut result = vec![];

        for i in 0..length {
            result.push(self.components[i] * other);
        }

        Vector::new(&result)
    }
}

impl Mul<&Vector> for f64 {
    type Output = Vector;

    fn mul(self, other: &Vector) -> Vector {
        let length = other.components.len();
        let mut result = vec![];

        for i in 0..length {
            result.push(self * other.components[i]);
        }

        Vector::new(&result)
    }
}