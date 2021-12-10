use std::collections::BTreeSet;

pub struct AuxVarManager {
	variable_offset: i32,
	free_variables: BTreeSet<i32>,
	remembered_variables: Option<Vec<i32>>,
}

impl AuxVarManager {
	pub fn new(first_free_variable: i32) -> AuxVarManager {
		AuxVarManager {
			variable_offset: first_free_variable,
			free_variables: BTreeSet::new(),
			remembered_variables: None,
		}
	}

	pub fn get_biggest_returned_aux_var(&self) -> i32 {
		self.variable_offset - 1
	}

	pub fn reset_aux_vars_to(&mut self, new_first_free_variable: i32) {
		self.variable_offset = new_first_free_variable;
		self.free_variables.retain(|&var| var < new_first_free_variable);
	}

	pub fn free_variables_range(&mut self, start: i32, end: i32) {
		for var in start..=end {
			self.free_variable(var);
		}
	}

	pub fn free_variables(&mut self, vars: &[i32]) {
		for var in vars {
			self.free_variable(*var);
		}
	}

	pub fn free_variable(&mut self, var: i32) {
		self.free_variables.insert(var);
	}

	pub fn get_variable(&mut self) -> i32 {
		if self.free_variables.is_empty() {
			let var = self.variable_offset;
			self.variable_offset += 1;

			if self.remembered_variables.is_some() {
				self.remembered_variables.as_mut().unwrap().push(var);
			}

			var
		} else {
			let var = self.free_variables.iter().next().unwrap().clone();
			self.free_variables.remove(&var);

			if self.remembered_variables.is_some() {
				self.remembered_variables.as_mut().unwrap().push(var);
			}

			var
		}
	}

	pub fn start_remember_returned_variables(&mut self) {
		self.remembered_variables = Some(Vec::new());
	}

	pub fn stop_remember_returned_variables(&mut self) -> Vec<i32> {
		match self.remembered_variables {
			Some(_) => {
				self.remembered_variables.take().unwrap()
			}
			None => Vec::new(),
		}
	}
}