use mollusk_svm::Mollusk;
use mollusk_svm_bencher::MolluskComputeUnitBencher;
use solana_sdk::pubkey::Pubkey;
    

fn main() {
    let mut mollusk = Mollusk::new(&Pubkey::from(pvault::ID), "pvault");

    let mol_bench = MolluskComputeUnitBencher::new(mollusk);
    
    //mol_bench
    //   .bench()
    //    .must_pass(true)
    //    .out_dir("./benches")
    //    .execute();
}
