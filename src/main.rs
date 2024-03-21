mod dram;

fn main() {
    // 1. DRAM setup
    let mut dram = Vec::<dram::Bank>::new();
    dram.push(dram::create_bank(3));
    dram.push(dram::create_bank(4));
    // cell A [1, 1, 1] = 3
    dram[0][0][0] = 3;
    // cell B [1, 3, 3] = 5
    dram[0][2][2] = 5;
    // cell C [1, 1, 2] = 1
    dram[0][0][1] = 1;
    // cell D [2, 4, 1] = 4
    dram[1][3][0] = 4;

    dram::print_dram(&dram);

    // 2. DRAM schedule setup
    let mut dram_schedule = Vec::<dram::Act>::new();
    dram_schedule.push(dram::Act::new(dram::Event::READ, 0, 0, 0));
    dram_schedule.push(dram::Act::new(dram::Event::READ, 0, 2, 2));
    dram_schedule.push(dram::Act::new(dram::Event::READ, 0, 0, 0));
    dram_schedule.push(dram::Act::new(dram::Event::READ, 0, 0, 0));
}