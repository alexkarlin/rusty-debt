#[derive(Debug)]
pub struct DebtRecord {
    pub id: i64,
    pub debtor: String,
    pub creditor: String,
    pub amount: i64,
    pub description: String
}

impl std::fmt::Display for DebtRecord {
    fn fmt(&self, record: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(record, "{}. {} -> {}: {} SEK ({})", self.id, self.debtor, self.creditor, self.amount, self.description)?;
        Ok(())
    }
}