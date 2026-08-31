pub(crate) struct Inventory {
    items: Vec<String>,
}

impl Inventory {
    pub(crate) fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    pub(crate) fn add_item(&mut self, item: String) {
        self.items.push(item);
    }
}

impl std::fmt::Display for Inventory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Inventory: {:?}", self.items)
    }
}
