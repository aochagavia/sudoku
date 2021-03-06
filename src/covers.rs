use entry::Entry;
use sudoku::Sudoku;
use std::mem;

/// Contains all the information to represent the sparse exact cover matrix
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Covers {
	/// List of entries that could still be entered
	pub entries: Vec<Entry>,
	/// Count of how many entries are still able to satisfy a certain constraint
    pub possibilities_count: Vec<u8>,
	/// Storage for whether a constraint has been met already
	///
	/// A possibility count of 0 would otherwise be ambiguous
	/// (it could mean 'impossible' or 'already set')
	/// Note: could be a BitVec, improvements are miniscule but measurable
	pub covered: Vec<bool>,
}

impl Covers {
	pub fn new() -> Covers {
		Covers {
			entries: (0..9*9*9)
				.map(|i| Entry { cell: (i / 9) as u8, num: (i % 9) as u8 + 1 } )
				.collect::<Vec<Entry>>(),
			// idx = row/col/field/cell * 9 + (num - 1) + offset, offset = 81 for col, 162 for field, 243 for cells
			possibilities_count: vec![9; 324],
			covered: vec![false; 324],
		}
	}

	pub fn from_sudoku(sudoku: &Sudoku) -> Covers {
		let mut covers = Covers::new();

		// Note: entries containing 0 are ignored
		let entries = sudoku.iter()
							.enumerate()
							.flat_map(|(i, num)| num.map(|n| Entry { cell: i as u8, num: n }));

		for entry in entries {
			covers.remove_impossible(entry);
		}

		covers
	}

	#[inline]
	pub fn is_empty(&self) -> bool { self.entries.is_empty() }

	pub fn remove_impossible(&mut self, new_entry: Entry) {
		// don't override a cell
		if self.covered[new_entry.cell_constraint()] {
			return
		}

		self.covered[new_entry.row_constraint()] = true;
		self.covered[new_entry.col_constraint()]  = true;
		self.covered[new_entry.field_constraint()] = true;
		self.covered[new_entry.cell_constraint()] = true;
		let mut entries = mem::replace(&mut self.entries, vec![] );
		entries.retain(|old_entry| {
			if old_entry.conflicts_with(new_entry) { // remove old_entry
				self.possibilities_count[old_entry.row_constraint()] -= 1;
				self.possibilities_count[old_entry.col_constraint()] -= 1;
				self.possibilities_count[old_entry.field_constraint()] -= 1;
				self.possibilities_count[old_entry.cell_constraint()] -= 1;
				false
			} else {
				true
			}
		});
		self.entries = entries;
	}
}
