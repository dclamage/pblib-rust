use crate::pb_constraint::*;
use crate::weighted_lit::*;

#[derive(Clone, Copy, Debug)]
pub enum PBType {
	DONTCARE,
	AMO,
	AMK,
	PB,
}

#[derive(Clone, Debug)]
pub struct SimplePBConstraint {
	base: PBConstraint,
	pb_type: PBType,
	max_sum: i64,
	max_weight: i64,
}

impl SimplePBConstraint {
	pub fn new_both(
		max_sum: i64,
		max_weight: i64,
		pb_type: PBType,
		literals: &[WeightedLit],
		less_eq: i64,
		greater_eq: i64,
	) -> SimplePBConstraint {
		assert_eq!(max_weight <= max_sum, true);
		SimplePBConstraint {
			base: PBConstraint::new_both(literals, less_eq, greater_eq),
			pb_type: pb_type,
			max_sum: max_sum,
			max_weight: max_weight,
		}
	}

	pub fn new_comp(
		max_sum: i64,
		max_weight: i64,
		pb_type: PBType,
		literals: &[WeightedLit],
		comparator: Comparator,
		bound: i64
	) -> SimplePBConstraint {
		assert_eq!(comparator, Comparator::LEQ);
		assert_eq!(max_weight <= max_sum, true);
		SimplePBConstraint {
			base: PBConstraint::new_comp(literals, comparator, bound),
			pb_type: pb_type,
			max_sum: max_sum,
			max_weight: max_weight,
		}
	}

	pub fn get_type(&self) -> PBType {
		self.pb_type
	}

	pub fn get_max_sum(&self) -> i64 {
		self.max_sum
	}

	pub fn get_max_weight(&self) -> i64 {
		self.max_weight
	}

	pub fn add_conditional(&mut self, literal: i32) {
		self.base.add_conditional(literal)
	}

	pub fn add_conditionals(&mut self, literals: &[i32]) {
		self.base.add_conditionals(literals)
	}

	pub fn clear_conditionals(&mut self) {
		self.base.clear_conditionals()
	}

	pub fn get_conditionals(&self) -> &Vec<i32> {
		self.base.get_conditionals()
	}

	pub fn get_n(&self) -> usize {
		self.base.get_n()
	}

	pub fn set_comparator(&mut self, comparator: Comparator) {
		self.base.set_comparator(comparator)
	}

	pub fn get_comparator(&self) -> Comparator {
		self.base.get_comparator()
	}

	pub fn set_geq(&mut self, geq: i64) {
		self.base.set_geq(geq)
	}

	pub fn set_leq(&mut self, leq: i64) {
		self.base.set_leq(leq)
	}

	pub fn get_min_sum(&self) -> i64 {
		self.base.get_min_sum()
	}

	pub fn get_geq_constraint(&self) -> PBConstraint {
		self.base.get_geq_constraint()
	}

	pub fn get_leq_constraint(&self) -> PBConstraint {
		self.base.get_leq_constraint()
	}

	pub fn get_geq(&self) -> i64 {
		self.base.get_geq()
	}

	pub fn get_leq(&self) -> i64 {
		self.base.get_leq()
	}

	pub fn get_weighted_literals(&self) -> &Vec<WeightedLit> {
		self.base.get_weighted_literals()
	}

	pub fn get_weighted_literals_mut(&mut self) -> &mut Vec<WeightedLit> {
		self.base.get_weighted_literals_mut()
	}
}
