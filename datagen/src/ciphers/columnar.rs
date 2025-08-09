use utils::vector_initialise;

pub struct Key<const COLUMNS: usize> {
    pub column_order: [usize; COLUMNS],
}

impl<const COLUMNS: usize> Key<COLUMNS> {
    #[must_use]
    pub fn new() -> Key<{ COLUMNS }> {
        let sample: Vec<usize> = (0..COLUMNS).collect();

        Self {
            column_order: vector_initialise::<COLUMNS, usize>(&sample),
        }
    }

    /// To decipher a text written by columns, written by rows
    #[must_use]
    pub fn encipher(&self, pt: &[u8]) -> Vec<u8> {
        let mut columns = vec![vec![]; COLUMNS];
        for (i, &c) in pt.iter().enumerate() {
            columns[i % COLUMNS].push(c);
        }

        let mut sorted_columns = vec![vec![]; COLUMNS];
        for (i, &c) in self.column_order.iter().enumerate() {
            sorted_columns[c].clone_from(&columns[i]);
        }

        sorted_columns.into_iter().flatten().collect()
    }
}
