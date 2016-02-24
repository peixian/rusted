/// Does Principal Component Analysis on the matrix
///
/// # Examples
/// use rusted::*;
/// let arr1: [i32, 3] = [1,2,3]
/// let arr2: [i32, 3] = [4,5,6]
/// let PCA: [i32, 2] = rusted::PCA(arr1, arr2)

use std::vec::Vec;

pub fn pca(array: &Vec<Vec<f64>>) {
	let mut means: Vec<f64> = Vec::new();
	for i in 0..array.len() {
		means.push(mean(&array[i]));
	}
}

pub fn mean(array: &Vec<f64>) -> f64 {
	let mut sum: f64 = 0.0;
	for i in 0..array.len() {
		sum += array[i];
	}
	return sum/(array.len() as f64);
}