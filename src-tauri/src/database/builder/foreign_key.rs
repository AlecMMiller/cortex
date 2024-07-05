use super::table::TableIdentifier;

pub struct ForeignKeyBuilder<'a> {
    reference_table: TableIdentifier<'a>,
}

impl<'a> ForeignKeyBuilder<'a> {
    pub fn new(reference_table: TableIdentifier<'a>) -> ForeignKeyBuilder<'a> {
        ForeignKeyBuilder {
            reference_table,
        }
    }

    pub fn build(&self) -> String {
        // first the local definition of the column t
        let mut query = format!("{} TEXT NOT NULL,\n", self.reference_table.get());

        query.push_str(format!("FOREIGN KEY ({}) REFERENCES {}(id)", self.reference_table.get(), self.reference_table.get()).as_str());

        query
    }
}
