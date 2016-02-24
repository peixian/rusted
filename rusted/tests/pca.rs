extern crate rusted;
use rusted::*;
use std::vec::Vec;

#[test] 
fn checkMeans() {
	let innerArrays = vec![0.0,1.7,2.0,4.5];
	let array: Vec<Vec<f64>> = vec![innerArrays; 3];
	let mut means: Vec<f64> = Vec::new();
	for i in 0..array.len() {
		means.push(rusted::mean(innerArrays))
	}
	assert_eq!(means[0], 8.2)
}