use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct Statistic {
	pub num_bdd_gates_encodings: usize,
	pub num_bdd_clause_encodings: usize,
	pub num_card_encodings: usize,
	pub num_adder_encodings: usize,
	pub num_amo_encodings: usize,
	pub num_clause: usize,
	pub num_trivial: usize,
	pub num_amo: usize,
	pub num_amk: usize,
	pub num_pb: usize,
}

impl Statistic {
	pub fn new() -> Statistic {
		Statistic {
			num_bdd_gates_encodings: 0,
			num_bdd_clause_encodings: 0,
			num_card_encodings: 0,
			num_adder_encodings: 0,
			num_amo_encodings: 0,
			num_clause: 0,
			num_trivial: 0,
			num_amo: 0,
			num_amk: 0,
			num_pb: 0,
		}
	}

	pub fn to_string(&self) -> String {
		let mut s = String::new();
		s.push_str("c PBLib statistic\n");
		s.push_str(&format!("amo: {}", self.num_amo));
		s.push_str(&format!("amk: {}", self.num_amk));
		s.push_str(&format!("pb: {}", self.num_pb));
		s.push_str(&format!("clause: {}", self.num_clause));
		s.push_str(&format!("trivial: {}\n", self.num_trivial));
		s
	}

	pub fn to_string_relative(&self) -> String {
		let sum: f64 =
			(self.num_amo + self.num_amk + self.num_pb + self.num_clause + self.num_trivial) as f64
				/ 100.0;
		let mut s = String::new();
		s.push_str("c PBLib statistic\n");
		s.push_str(&format!("amo: {:.2}", (self.num_amo as f64) / sum));
		s.push_str(&format!("amk: {:.2}", (self.num_amk as f64) / sum));
		s.push_str(&format!("pb: {:.2}", (self.num_pb as f64) / sum));
		s.push_str(&format!("clause: {:.2}", (self.num_clause as f64) / sum));
		s.push_str(&format!(
			"trivial: {:.2}\n",
			(self.num_trivial as f64) / sum
		));
		s
	}
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum BIMANDER_M_IS {
	N_HALF,
	N_SQRT,
	FIXED,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum PB2CNF_AMO_Encoder {
	BEST,
	NESTED,
	BDD,
	BIMANDER,
	COMMANDER,
	KPRODUCT,
	BINARY,
	PAIRWISE,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum PB2CNF_AMK_Encoder {
	BEST,
	BDD,
	CARD,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum PB2CNF_PB_Encoder {
	BEST,
	BDD,
	SWC,
	SORTINGNETWORKS,
	ADDER,
	BINARY_MERGE,
}

#[derive(Debug, Clone)]
pub struct PBConfigClass {
	pub pb_encoder: PB2CNF_PB_Encoder,
	pub amk_encoder: PB2CNF_AMK_Encoder,
	pub amo_encoder: PB2CNF_AMO_Encoder,
	pub bimander_m_is: BIMANDER_M_IS,
	pub bimander_m: i32,
	pub k_product_minimum_lit_count_for_splitting: i32,
	pub k_product_k: i32,
	pub commander_encoding_k: i32,
	pub max_clauses_per_constraint: i64,
	pub use_formula_cache: bool,
	pub print_used_encodings: bool,
	pub check_for_dup_literals: bool,
	pub use_gac_binary_merge: bool,
	pub binary_merge_no_support_for_single_bits: bool,
	pub use_recursive_bdd_test: bool,
	pub use_real_robdds: bool,
	pub use_watch_dog_encoding_in_binary_merger: bool,

	pub debug_value: String,
	pub just_approximate: bool,
	pub approximate_max_value: i64,
	pub cmd_line_options: BTreeSet<String>,

	pub config_name: String,
}

impl PBConfigClass {
	pub fn new() -> PBConfigClass {
		PBConfigClass {
			pb_encoder: PB2CNF_PB_Encoder::BEST,
			amk_encoder: PB2CNF_AMK_Encoder::BEST,
			amo_encoder: PB2CNF_AMO_Encoder::BEST,
			bimander_m_is: BIMANDER_M_IS::N_HALF,
			bimander_m: 3,
			k_product_minimum_lit_count_for_splitting: 10,
			k_product_k: 2,
			commander_encoding_k: 3,
			max_clauses_per_constraint: 1000000,
			use_formula_cache: false,
			print_used_encodings: false,
			check_for_dup_literals: false,
			use_gac_binary_merge: false,
			binary_merge_no_support_for_single_bits: true,
			use_recursive_bdd_test: false,
			use_real_robdds: true,
			use_watch_dog_encoding_in_binary_merger: false,

			debug_value: String::new(),
			just_approximate: false,
			approximate_max_value: 1000,
			cmd_line_options: BTreeSet::new(),

			config_name: String::new(),
		}
	}
}
