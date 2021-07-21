use structopt::StructOpt;

use rusqlite::Result;

pub mod debtrecord;
pub mod database;

#[derive(StructOpt, Debug)]
#[structopt(name = "rusty-debt", about = "A CLI-based debt-tracker")]
enum Cli {
    #[structopt(name = "add")]
    Add {
        creditor: String,
        debtor: String,
        amount: i64,
        #[structopt(default_value = "N/A")]
        description: String
    },

    #[structopt(name = "remove")]
    Remove {
        index: i64,
    },

    #[structopt(name = "reset")]
    Reset,

    #[structopt(name = "print")]
    Print
}

fn main() -> Result<()> {
    let mut db: database::Database = database::Database::new();
    db.initialize()?;

    match Cli::from_args() {
        Cli::Add { creditor, debtor, amount, description } => {
            let entry = db.add_entry(creditor, debtor, amount, description)?;
            print!("Added a new debt entry:\n\t{}", entry);
        },
        Cli::Remove { index } => {
            let entry = db.remove_entry(index)?;
            print!("Removed debt entry:\n\t{}", entry);
        },
        Cli::Reset => {
            db.reset_database()?;
            println!("Database reset!");
        },
        Cli::Print => {
            let result = db.get_entries()?;

            if result.len() == 0 {
                println!("No debt recorded in database!");
            }
            else {
                for entry in result {
                    println!("{}", entry);
                }
            }
        }
    }

    Ok(())
}
