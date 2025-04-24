use alloy_sol_types::SolType;
use clap::Parser;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};

// Define the same MerkleTreeValues struct from the main program
use alloy_sol_types::sol;

sol! {
    struct MerkleTreeValues {
        uint32 leaf_count;
        uint32 verification_index;
        bytes32 root;
        bool verification_result;
        uint64 hash_operations;
    }
}

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const ANGELA_ELF: &[u8] = include_elf!("angela-program");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    execute: bool,

    #[arg(long)]
    prove: bool,

    #[arg(long, default_value = "128")]
    leaf_count: u32,

    #[arg(long, default_value = "42")]
    verification_index: u32,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&args.leaf_count);
    stdin.write(&args.verification_index);

    println!("leaf_count: {}", args.leaf_count);
    println!("verification_index: {}", args.verification_index);

    if args.execute {
        // Execute the program
        let (output, report) = client.execute(ANGELA_ELF, &stdin).run().unwrap();
        println!("Program executed successfully.");

        // Read the output.
        let decoded = MerkleTreeValues::abi_decode(output.as_slice()).unwrap();
        let MerkleTreeValues {
            leaf_count,
            verification_index,
            root,
            verification_result,
            hash_operations,
        } = decoded;

        println!("leaf_count: {}", leaf_count);
        println!("verification_index: {}", verification_index);
        println!("root: {:?}", root);
        println!("verification_result: {}", verification_result);
        println!("hash_operations: {}", hash_operations);

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(ANGELA_ELF);

        // Generate the proof
        let proof = client
            .prove(&pk, &stdin)
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
