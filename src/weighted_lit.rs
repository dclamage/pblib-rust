use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub struct WeightedLit {
	pub lit: i32,
	pub weight: i64,
}

impl WeightedLit {
	pub fn new(lit: i32, weight: i64) -> WeightedLit {
		WeightedLit {
			lit: lit,
			weight: weight,
		}
	}

	pub fn comp_variable_asc(lhs: &WeightedLit, rhs: &WeightedLit) -> Option<Ordering> {
		Some(lhs.weight.cmp(&rhs.weight))
	}

	pub fn comp_variable_des(lhs: &WeightedLit, rhs: &WeightedLit) -> Option<Ordering> {
		Some(rhs.weight.cmp(&lhs.weight))
	}

	pub fn comp_variable_des_var(lhs: &WeightedLit, rhs: &WeightedLit) -> Option<Ordering> {
		Some(rhs.lit.cmp(&lhs.lit))
	}
}

impl PartialEq for WeightedLit {
	fn eq(&self, other: &WeightedLit) -> bool {
		self.lit == other.lit
	}
}

impl PartialOrd for WeightedLit {
	fn partial_cmp(&self, other: &WeightedLit) -> Option<Ordering> {
		Some(self.lit.cmp(&other.lit))
	}
}
