# evolve-sbrain

This program uses a simple, highly parallel evolutionary algorithm to evolve computer programs in my language [sbrain](https://github.com/silverwingedseraph/sbrain).

It is quite effective at evolving relatively simple algorithms. It is invoked with `cargo run --release <config file>`, where the config file looks like this:

```
mutations_per_generation = 3
initial_program_length = 8
max_runtime = 128
population_size = 32
inputs = [ [1],       [2],       [999]]
targets = [[1, 2, 3], [2, 3, 4], [999, 1000, 1001]]
```

This is the specification for a program that starts at the input, and prints the next two consecutive numbers.
A sample run from this example is:

```
Generation     1 Cost  5079: .Sa.s-^a
Generation     2 Cost  2013: ..S,.s-^a
Generation    26 Cost  1011: ,..zS,.s-^a
Generation    30 Cost     6: ,..zS+.s-^a
Generation    53 Cost     3: ,..+S+.s-^a
Generation   101 Cost     0: ,.*+.+.s-^a
Program found after 101 tries.
,.*+.+.s-^a
Ran for 100 cycles and did not halt
[1] -> ,.*+.+.s-^a -> [1, 2, 3]
Ran for 100 cycles and did not halt
[2] -> ,.*+.+.s-^a -> [2, 3, 4]
Ran for 100 cycles and did not halt
[999] -> ,.*+.+.s-^a -> [999, 1000, 1001]
```

The optimal program, of course, is `,.+.+.`. The final program this found is, in a sort of pseudo-assembly:

1. Get input
2. Put input
3. Multiply
4. Add one to the current cell
5. Output
6. Add one to the current cell
7. Output
8. Bitshift to the left
9. Subtract one from the current cell
10. Bitwise NOR
11. Add

There are a number of sample configurations in `sample-configs/`. Try them out!