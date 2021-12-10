use crate::weighted_lit::WeightedLit;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Comparator {
	LEQ,
	GEQ,
	BOTH,
}

#[derive(Clone, Debug)]
pub struct PBConstraint {
	leq: i64,
	geq: i64,
	weighted_literals: Vec<WeightedLit>,
	comparator: Comparator,
	conditionals: Vec<i32>,
}

impl PBConstraint {
	pub fn new_both(literals: &[WeightedLit], less_eq: i64, greater_eq: i64) -> PBConstraint {
		PBConstraint {
			leq: less_eq,
			geq: greater_eq,
			weighted_literals: literals.to_vec(),
			comparator: Comparator::BOTH,
			conditionals: Vec::new(),
		}
	}

	pub fn new_comp(
		literals: &[WeightedLit],
		comparator: Comparator,
		bound: i64,
	) -> PBConstraint {
		PBConstraint {
			leq: if comparator == Comparator::LEQ || comparator == Comparator::BOTH {
				bound
			} else {
				0
			},
			geq: if comparator == Comparator::GEQ || comparator == Comparator::BOTH {
				bound
			} else {
				0
			},
			weighted_literals: literals.to_vec(),
			comparator: comparator,
			conditionals: Vec::new(),
		}
	}

	pub fn new() -> PBConstraint {
		PBConstraint {
			leq: 0,
			geq: 0,
			weighted_literals: Vec::new(),
			comparator: Comparator::LEQ,
			conditionals: Vec::new(),
		}
	}

	pub fn add_conditional(&mut self, literal: i32) {
		self.conditionals.push(literal);
	}

	pub fn add_conditionals(&mut self, literals: &[i32]) {
		self.conditionals.extend(literals);
	}

	pub fn clear_conditionals(&mut self) {
		self.conditionals.clear();
	}

	pub fn get_conditionals(&self) -> &Vec<i32> {
		&self.conditionals
	}

	pub fn get_n(&self) -> usize {
		self.weighted_literals.len()
	}

	pub fn set_comparator(&mut self, comparator: Comparator) {
		self.comparator = comparator;
	}

	pub fn get_comparator(&self) -> Comparator {
		self.comparator
	}

	pub fn set_geq(&mut self, geq: i64) {
		self.geq = geq;
	}

	pub fn set_leq(&mut self, leq: i64) {
		self.leq = leq;
	}

	pub fn get_max_sum(&self) -> i64 {
		let mut max_sum = 0;
		for lit in &self.weighted_literals {
			if lit.weight >= 0 {
				max_sum += lit.weight;
			}
		}
		max_sum
	}

	pub fn get_min_sum(&self) -> i64 {
		let mut min_sum = 0;
		for lit in &self.weighted_literals {
			if lit.weight < 0 {
				min_sum += lit.weight;
			}
		}
		min_sum
	}

	pub fn get_geq_constraint(&self) -> PBConstraint {
		assert_eq!(self.comparator, Comparator::BOTH);
		let mut c =
			PBConstraint::new_comp(&self.weighted_literals, Comparator::GEQ, self.geq);
		c.add_conditionals(&self.conditionals);
		c
	}

	pub fn get_leq_constraint(&self) -> PBConstraint {
		assert_eq!(self.comparator, Comparator::BOTH);
		let mut c =
			PBConstraint::new_comp(&self.weighted_literals, Comparator::LEQ, self.leq);
		c.add_conditionals(&self.conditionals);
		c
	}

	pub fn get_geq(&self) -> i64 {
		self.geq
	}

	pub fn get_leq(&self) -> i64 {
		self.leq
	}

	pub fn get_weighted_literals(&self) -> &Vec<WeightedLit> {
		&self.weighted_literals
	}

	pub fn get_weighted_literals_mut(&mut self) -> &mut Vec<WeightedLit> {
		&mut self.weighted_literals
	}
}
