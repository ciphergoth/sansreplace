# Third party software

* SIMDxorshift
  * cloned from https://github.com/lemire/SIMDxorshift.git
  * revision 35ad835e2160e1460cc4d3a6e385f2fd55255716
  * with one local change: `avx512_xorshift128plus_init` initializes all eight lanes, as
    upstream now does. The revision above left lanes 4--7 uninitialized, so the AVX-512 build's
    random stream was neither deterministic nor reliably independent across lanes.
