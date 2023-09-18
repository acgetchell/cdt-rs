# cdt-rs

[![CI](https://github.com/acgetchell/cdt/actions/workflows/ci.yml/badge.svg)](https://github.com/acgetchell/cdt/actions/workflows/ci.yml)
[![Codecov](https://codecov.io/gh/acgetchell/cdt-rs/graph/badge.svg?token=CsbOJBypGC)](https://codecov.io/gh/acgetchell/cdt-rs)

Causal Dynamical Triangulations using Constrained Delaunay Triangulations in Rust

## Introduction

For an introduction to Causal Dynamical Triangulations, see [this paper](https://arxiv.org/abs/hep-th/0105267).

## Roadmap

- [x] Use [Kani] to verify correctness
- [ ] Use an existing Rust Delaunay triangulation library, or write one that can be verified by [Kani]?
- [x] 2D Delaunay triangulation
- [ ] 1+1 foliation
- [ ] 2D ergodic moves
- [ ] 2D Metropolis-Hastings
- [ ] Diffusion-accelerated MCMC?
- [ ] Visual output
- [ ] 3D Delaunay triangulation
- [ ] 2+1 foliation
- [ ] 3D ergodic moves
- [ ] 3D Metropolis-Hastings
- [ ] Visual output
- [ ] 4D Delaunay triangulation
- [ ] 3+1 foliation
- [ ] 4D ergodic moves
- [ ] 4D Metropolis-Hastings
- [ ] Initialize masses in 3D with Constrained Delaunay Triangulation
- [ ] Initialize masses in 4D with Constrained Delaunay Triangulation
- [ ] Shortest path
- [ ] Geodesic distance
- [ ] Einstein tensor
- [ ] Quantize spacetime!

## Issues

[Kani] and [Spade] [don't get along](https://github.com/model-checking/kani/issues/2776).

[Kani]: https://crates.io/crates/kani-verifier
[Spade]: https://crates.io/crates/spade
