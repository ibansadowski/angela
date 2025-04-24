# On SP1’s Precompiles

### Introduction

With two lines of code, developers can reduce the cycle cost of verifiable compute by at least 77% and cut execution time by 59%. All thanks to SP1's precompiles. And it costs pennies on the new Prover Network.

SP1’s precompiles deliver critical improvements to zk development. They offer the optimal middle ground between custom circuits and general-purpose zkVMs, combining safety and efficiency to improve the developer experience. 

In this blog, we'll go over their technical specifications and see them deliver real-world performance improvements. 

### It's One + The Other

An application developer looking to leverage programmable cryptography currently has two options: write a “bespoke” circuit using a DSL, where the only operations allowed are those defined in the circuit; or run their code on a zkVM, which can prove an arbitrarily complex set of instructions. This “choice” between the two is a false dichotomy; let’s explore why. 

### Developer Dilemma

Developers looking to maximize efficiency for their computation-heavy tasks, such as hashing algorithms, might default to circuits. Custom circuits are hand-optimized through clever math and coding tricks to deliver orders of magnitude faster performance, compared to sequentially running each necessary step in a zkVM. 

The rest of their code, however, would require the same level of stringent detail, and development time balloons.
This is where precompiles — generalized circuits that stand adjacent to our zkVM — enable them to offload frequently used operations.

### Illustrative Toy Example

Now (for the purpose of illustration, not production) let’s run a program that uses a common cryptographic algorithm, SHA-256. We'll construct a Merkle tree from a given leaf, generate a proof of inclusion for that leaf, verify the proof against the Merkle root, and encode and output public values for external verification.

We’ll run this on the Prover Network to establish the strongest foundation for our benchmarks.

Here’s our first run:

![1st Run](Figures/1stRun.png "1st Run")

And here’s our second:

![2nd Run](Figures/2ndRun.png "2nd Run")

So the precompile version shows:
- Gas usage reduced from 3,022,143 to 1,991,440 (about 34% reduction)
- Cycle limit reduced from 1,892,952 to 428,045 (about 77% reduction)
- Execution time reduced from 87.7ms to 36.2ms (about 59% faster)

![Comparison](Figures/comparison.png "Comparison")

How was this significant performance improvement achieved? **Two simple lines of code** in the root ```Cargo.toml```:
``` 
[patch.crates-io]
sha2-v0-10-8 = { git = "https://github.com/sp1-patches/RustCrypto-hashes", package = "sha2", tag = "patch-sha2-0.10.8-sp1-4.0.0" } 
```
This is the power of precompiles.

The remainder of the code can be found at [github.com/ibansadowski/angela](https://github.com/ibansadowski/angela). A list of existing precompiles and their associated patched crates can be found here: [Patched Crates](https://docs.succinct.xyz/docs/sp1/optimizing-programs/precompiles#patched-crates). They include tiny-keccak, BLS12-381, and RSA.

Clearly, in terms of development speed and program execution speed, zkVMs with precompiles are in a league of their own.

### Technical Specifications

How exactly does SP1 achieve this? Let’s briefly cover SP1 and where precompiles fit in. First, some definitions:

- *Execution Trace*: A table representation, detailing every intermediate step or state transition of the computation. Each state transition or computational step is captured as a distinct row within these tables.

- *Constraints*: Logical rules or relationships that tie these steps, and other tables, together. They ensure consistency across tables by encoding the correctness of the computation.

Together, tables and constraints capture a static snapshot of the program's execution. From that snapshot, we can efficiently and cryptographically verify the computation, yielding a STARK proof of correctness.

From the developer's point of view, using SP1 feels like regular programming: you write your program in Rust, it gets compiled down to RISC-V assembly code, and SP1 translates that into an execution trace and its corresponding constraints.

![Execution Flow](Figures/Flow.png "Execution Flow")

Once the code execution encounters a precompile, it short-circuits out to an existing table. Instead of recomputing the whole trace itself, SP1 executes a syscall within the RISC-V runtime. This redirects the code execution to a highly-optimized, pre-established lookup table. These lookup tables are nothing more than custom circuits themselves! 

![Execution Flow with Precompile](Figures/PreFlow.png "Execution Flow with Precompile")

These circuits (specialized STARKs) accelerate the rate of computation, since the tables have been pre-computed. After executing the precompile, SP1 reintegrates the results back into the main RISC-V execution trace. This is possible because precompiles can directly read/write memory. 
Finally, both the precompile trace and the main trace are bundled together into a unified STARK proof.

We can now guarantee (assuming the precompile is written correctly) that executing our SP1 program produces the expected results **with orders of magnitude fewer cycles spent building tables and constraints**.

SP1 gives you total freedom over what to precompile and what to run dynamically.


To conclude, SP1's precompiles deliver the immediate efficiency of custom circuits, while being wrapped up in the flexibility of zkVMs. We can expect popular programs to turn into precompiles themselves. Therefore, development time will keep trending down thanks to precompiles.  
### Historical Note

We take it for granted these days that TCP/IP is the default networking protocol used for global communication. This was far from a given, with serious competition by the OSI Model, a more “elegant” technical solution supported by “stronger” standards. Yet as history shows, the high complexity of OSI lost out to TCP/IP's simple and practical implementation. We see some parallels here between the two “sides” of this current debate. The truth here isn't so simple *because* of precompiles. What’s most extensible is often most standardizable, and will ultimately “win” out on adoption.

