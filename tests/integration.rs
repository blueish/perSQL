extern crate rexpect;

use rexpect::spawn;
use rexpect::errors::*;


#[cfg(test)]
mod integration {

    use super::*;

    fn call_initial_quit() -> Result<()> {
        let mut p = spawn("./target/debug/persql", Some(200))?;

        p.send_line(".exit")?;
        p.exp_eof()?;
        Ok(())
    }

    #[test]
    fn run_quit() {
        call_initial_quit().unwrap_or_else(|e| panic!("test failed with: {}", e));
    }

    fn insert_one_row() -> Result<()> {
        let mut p = spawn("./target/debug/persql", Some(200))?;

        p.exp_regex("persql>")?;
        p.send_line("insert 1 john example@example.com")?;
        p.exp_regex("Executed")?;

        p.exp_regex("persql>")?;
        p.send_line("select")?;
        p.exp_regex(".*1.*john.*example@example.com")?;

        p.send_line(".exit")?;
        p.exp_eof()?;
        Ok(())
    }

    #[test]
    fn test_insert_one() {
        insert_one_row().unwrap_or_else(|e| panic!("test failed with: {}", e));
    }

    fn full_table_err() -> Result<()> {
        let mut p = spawn("./target/debug/persql", Some(200))?;

        for i in 0..(1400 - 1) {
            p.exp_regex("persql>")?;
            p.send_line(format!("insert {} john example@example.com", i).as_str())?;
            p.exp_regex("Executed")?;
        }

        // Expect once the table is full to error
        p.exp_regex("persql>")?;
        p.send_line(format!("insert {} john example@example.com", 1400).as_str())?;
        p.exp_regex("Table is full")?;

        p.send_line(".exit")?;
        p.exp_eof()?;
        Ok(())
    }

    #[test]
    fn run_full_table() {
        full_table_err().unwrap_or_else(|e| panic!("test failed with: {}", e));
    }

}
