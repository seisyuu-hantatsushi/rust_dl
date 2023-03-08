/* -*- tab-width:4 -*- */
use num;

use crate::tensor::tensor_base::Tensor;

#[derive(Copy, Clone)]
enum Operator {
	MAX, MIN
}

impl<T> Tensor<T>
where T:num::Num+Clone+Copy+std::cmp::PartialOrd {

	fn compare_tuple(op: Operator, p:(&T,&T)) -> T {
		let (&m,&e) = p;
		match op {
			Operator::MAX => if m < e { e } else { m }
			Operator::MIN => if m > e { e } else { m }
		}
	}

	fn compare_values(op: Operator, a:&T, b:&T) -> T {
		match op {
			Operator::MAX => if a < b { *b } else { *a }
			Operator::MIN => if a > b { *b } else { *a }
		}
	}

	fn max_min_subtensor(v:&[T],src_shape:&[usize],dst_shape:&[usize], op:Operator) -> Tensor<T> {
		if dst_shape.len() > 2 {
			if dst_shape[0] == 1 {
				let stride = src_shape[1..].iter().fold(1,|p,&e| p*e);
				let mut strided_vecs = v.chunks(stride);
				let max_tensor_v = if let Some(first_v) = strided_vecs.nth(0) {
					strided_vecs.fold(first_v.to_vec(),
									  |max_v, v| {
										  let compare_tuple_max = |p| Self::compare_tuple(op, p);
										  max_v.iter().zip(v.iter()).map(compare_tuple_max).collect::<Vec<T>>()
									  })
				}
				else {
					panic!("no element in Tensor")
				};
				Self::max_min_subtensor(&max_tensor_v, &src_shape[1..], &dst_shape[1..],op)
			}
			else {
				let mut ts:Vec<Tensor<T>> = vec!();
				let stride = src_shape[1..].iter().fold(1,|prod, &e| prod*e);
				let strided_vec = v.chunks(stride);
				for sv in strided_vec {
					let t = Self::max_min_subtensor(sv, &src_shape[1..], &dst_shape[1..],op);
					ts.push(t);
				}
				let mut v:Vec<T> = vec!();
				for t in ts.iter() {
					v.extend(t.buffer());
				}
				fn new_reshaper(shape:&[usize]) -> Vec<usize> {
					if shape.len() > 2 {
						let mut v:Vec<usize> = vec!();
						if shape[0] != 1 {
							v.push(shape[0]);
						}
						v.extend(&new_reshaper(&shape[1..]));
						v
					}
					else {
						if shape[0] == 1 && shape[1] == 1 {
							vec![1]
						}
						else if shape[0] != 1 && shape[1] != 1 {
							shape.to_vec()
						}
						else {
							vec![shape[0]*shape[1]]
						}
					}
				}
				let new_shape = new_reshaper(&dst_shape);
				Tensor::from_vector(new_shape, v)
			}
		}
		else {
			if dst_shape[0] == 1 && dst_shape[1] == 1 {
				let s = v.into_iter().fold(num::zero(),|accum, e| accum+*e);
				Tensor::<T>::from_array(&[1,1], &[s])
			}
			else if dst_shape[0] == 1 {
				let mut strided_vecs = v.chunks(dst_shape[1]);
				let max_tensor_v = if let Some(first_v) = strided_vecs.nth(0) {
					strided_vecs.fold(first_v.to_vec(),
									  |mv, v| {
										  mv.iter().zip(v.iter())
											  .map(|p| Self::compare_tuple(op, p)).collect::<Vec<T>>() })
				}
				else {
					panic!("no element in Tensor")
				};
				Tensor::from_vector(vec![1,dst_shape[1]],max_tensor_v)
			}
			else if dst_shape[1] == 1 {
				let strided_vecs = v.chunks(src_shape[1]);
				let max_tensor_v =
					strided_vecs.map(|v| {
						let compare_value_max = |a:T,b:T| Self::compare_values(op, &a, &b);
						if let Some(m) = v.to_vec().into_iter().reduce(compare_value_max) {
							m
						}
						else {
							panic!("no element in Tensor")
						}
					}).collect::<Vec<T>>();
				Tensor::from_vector(vec![dst_shape[0],1],max_tensor_v)
			}
			else {
				Tensor::<T>::from_array(dst_shape, v)
			}
		}
	}

	fn max_min_in_axis(&self, axis:usize, op:Operator) -> Tensor<T> {
		if self.shape().len() == 0 {
			self.clone()
		}
		else if self.shape().len() == 1 {
			let compare_value_max = |a:T,b:T| Self::compare_values(op, &a, &b);
			let max = if let Some(m) = self.buffer().to_vec().into_iter().reduce(compare_value_max) {
				m
			}
			else {
				panic!("invalid source shape.");
			};
			Tensor::<T>::from_vector(vec!(), vec![max])
		}
		else {
			let dst_shape = {
				let mut v = self.shape().to_vec();
				v[axis] = 1;
				v
			};
			Self::max_min_subtensor(self.buffer(), self.shape(), &dst_shape, op)
		}
	}

	pub fn max_in_axis(&self, axis:usize) -> Tensor<T> {
		self.max_min_in_axis(axis, Operator::MAX)
	}

	pub fn min_in_axis(&self, axis:usize) -> Tensor<T> {
		self.max_min_in_axis(axis, Operator::MIN)
    }
}