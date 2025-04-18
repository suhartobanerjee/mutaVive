# Mutavive

A cellular automaton featuring structural variants in cells.

## Development path

- [x] create the grid of cells.
- [x] Colour every mutation with a random colour.
- [x] Mutation probability sample from a distribution.
- [x] Model cell data as n bases rather than genomic regions.
    - [x] Mutation on a 'x' number of bases chosen at random.
- [x] Impl Distribution for SvState in order to randomly sample a Sv State for mutation.
- [x] Colors as a function of the total state of the genome.
    - [x] Model 3 broad type of mutations as the 3 channels of colour { gain: red, loss: blue, inv: green}
    - [x] With every mutation, slowly decrease the corresponding channel by the difference over NATURAL_SELECTION and prop of genome affected till it reaches black
- [ ] Pause functionality.
- [x] Local natural selection.
- [ ] Model germline mutations as border colour --> can be used to track lineages.
- [ ] Model somatic mutations as the fill colour --> to check heterogeneity.
- [ ] Model the entire grid as a tissue.
- [ ] Find nearby cells (6 nearby cells).
- [ ] Differing strength of natural selection as a function of selection strength of nearby cells.
- [ ] Show generation as a label somewhere.
