# On SP1’s Precompiles

### Introduction

With 2 lines of codes developers can reduce the cycle cost of programmable cryptography by at least 77%, with 59% faster execution. All thanks to SP1's precompiles. And it costs pennies on the new Prover Network.

SP1’s precompiles deliver critical improvements. They offer the optimal middle ground between custom circuits and generalizable zkVMs: combining safety and efficiency to improve the developer experience. 

In this blog, let’s go over their technical specifications, see them deliver real-world performance improvements, and conclude on a historical comparison. 

### It's One + The Other

An application developer looking to leverage programmable cryptography currently has two options: write a “bespoke” circuit using a DSL, where the only operations allowed are those defined in the circuit; or running their code on a zkVM, which can prove an arbitrarily complex set of instructions. The latter of which is compiled into a “common” instruction set and run through the zkVM’s proving system. This “choice” between the two is a false dichotomy, let’s explore why. 

### Developer's Dilemma

At first, developers looking to squeeze out efficiency from their computation intensive tasks, like hashing algorithms, might default to circuits. Indeed, these have been hand-optimized through clever math and coding tricks to deliver orders of magnitude faster performance than sequentially running each necessary step. 

The rest of their code however would require the same level of stringent detail, and development time starts to balloon.
This is where precompiles come in –– generalized circuits that stand adjacent to our zkVM –– where we may offload frequently used operations.

### Illustrative Toy Example

Now (for the purpose of illustration, not production) let’s run a program that heavily relies on one of these cryptographic libraries, SHA256. We'll construct a Merkle Tree from a given leaf data, generate a proof of inclusion for that specific leaf, verify the proof against the Merkle root, and encode and output public values for external verification.

We’ll run this benchmark on the Prover Network to establish the strongest foundation.

Here’s our first run:

![1st Run](Figures/1stRun.png "1st Run")

And here’s our second:

![2nd Run](Figures/2ndRun.png "2nd Run")

To be clear, here’s the breakdown:
- 1st Run (without precompile):
    - Gas: 3,022,143
    - Cycle limit: 1,892,952
    - time.busy: 87.7ms (execution time)

- 2nd Run (with precompile):
    - Gas: 1,991,440
    - Cycle limit: 428,045
    - time.busy: 36.2ms (execution time)

So the precompile version shows:
- Gas usage reduced from 3,022,143 to 1,991,440 (about 34% reduction)
- Cycle limit reduced from 1,892,952 to 428,045 (about 77% reduction)
- Execution time reduced from 87.7ms to 36.2ms (about 59% faster)

![Comparison](Figures/comparison.png "Comparison")

How was this significant performance improvement achieved? Two simple lines of code in the root Cargo.toml:
``` 
[patch.crates-io]
sha2-v0-10-8 = { git = "https://github.com/sp1-patches/RustCrypto-hashes", package = "sha2", tag = "patch-sha2-0.10.8-sp1-4.0.0" } 
```

The remainder of the code can be found at [github.com/ibansadowski/angela](https://github.com/ibansadowski/angela). 

Clearly, in terms of development speed and program execution speed, zkVMs with precompiles are in a league of their own.

### Technical Specifications

How exactly does SP1 achieve this? Let’s briefly cover SP1 and where precompiles fit in. First, some definitions:

- *Execution Trace*: A table representation, detailing every intermediate step or state transition of the computation. Each state transition or computational step is captured as a distinct row within these tables.

- *Constraints*: Logical rules or relationships that tie these steps, and other tables, together. They ensure consistency across tables by encoding the correctness of the computation.

Together, tables and constraints provide a structured representation of program execution, which allows the entire computation to be efficiently and cryptographically verifiable. The final result of generating a proof from a union of tables and constraints is a STARK proof. Now then.

Users write their program in Rust, which gets compiled down to RISC-V Assembly code. This code is then translated into an execution trace and its associated constraints.

![Execution Flow](Figures/Flow.png "Execution Flow")

Once the code execution encounters a precompile, it shortcuts out to an existing table. Instead of recomputing the whole trace itself, SP1 executes a syscall within the RISC-V runtime. This redirects the code execution to a highly-optimized, pre-established lookup table. These lookup tables are nothing more than custom circuits themselves! 

![Execution Flow with Precompile](Figures/PreFlow.png "Execution Flow with Precompile")

These circuits (specialized STARKs) accelerate the rate of computation, since the tables have been precomputed. After executing the precompile, SP1 reintegrates the results back into the main RISC-V execution trace. This thanks to the fact that precompiles can directly read/write memory. 
Finally, both the precompile trace and the main trace are bundled together into a unified STARK proof.

We can now guarantee that, assuming the precompile is written correctly, executing our SP1 program will yield the expected results. With orders of magnitude fewer cycles of tables and constraints being built.

SP1 gives you total freedom over what to precompile and what to run dynamically.

### Historical Note

We take it for granted these days that TCP/IP is the default networking protocol used for global communication. This was far from a given, with serious competition by the OSI Model, a more “elegant” technical solution supported by “stronger” standards. Yet as history shows, the high complexity of OSI lost out to the simple and practical TCP/IP model. We see some parallels here between the two “sides” of this current debate. And the truth isn’t so simple here, in particular *because* of precompiles. As we already see, critical crates have been patched and allow developers to improve their performance right out of the gate. It’s easy to imagine that further programs that are highly sought after will get precompiled as well. What’s most extensible is ofter most standardizable, and will ultimately “win” out on adoption.

