pub type Bank = Vec<Vec<i32>>;

pub enum Event {
    READ,
    WRITE,
    ADD
}
pub struct Act {
    pub event: Event,
    pub bank: usize,
    pub row: usize,
    pub col: usize
}
impl Act {
    pub fn new(e: Event, b: usize, r: usize, c: usize) -> Act {
        Act {
            event: e,
            bank: b,
            row: r,
            col: c,
        }
    }
}

pub fn create_bank(size: usize) -> Bank {
    vec![vec![0; size]; size]
}
pub fn print_dram(bank: &Vec<Bank>) {
    for a in 0..bank.len() {
        for b in 0..bank[a].len() {
            for c in 0..bank[a][b].len() {
                let value = bank[a][b][c];
                print!("{value}, ");
            }
            println!("");
        }
        println!("");
    }
}
pub fn run_schedule(schedule: &Vec<Act>) {

}