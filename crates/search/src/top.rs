use std::cmp::Ordering;

pub struct TopResults<T, F: Fn(&T, &T) -> Ordering> {
	items: Vec<T>,
	limit: usize,
	compare: F,
}

impl<T, F: Fn(&T, &T) -> Ordering> TopResults<T, F> {
	pub fn new(limit: usize, compare: F) -> Self {
		Self {
			items: Vec::with_capacity(limit.min(512)),
			limit,
			compare,
		}
	}

	pub fn add(&mut self, item: T) {
		if self.limit == 0 {
			return;
		}
		if self.items.len() >= self.limit
			&& self
				.items
				.last()
				.is_none_or(|last| (self.compare)(&item, last) != Ordering::Less)
		{
			return;
		}
		let position = self
			.items
			.partition_point(|existing| (self.compare)(&item, existing) != Ordering::Less);
		self.items.insert(position, item);
		self.items.truncate(self.limit);
	}

	pub fn into_vec(self) -> Vec<T> {
		self.items
	}
}
