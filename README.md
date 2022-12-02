<p align="center">
  <img src="https://raw.githubusercontent.com/xrctius/finchly/master/finchly_logo.png" width="500px" />
</p>

Finchly simulates self-replication and genetic programming. It will provide full customisability in experiment topology, genetic instruction sets, organism incentives, and mutation rates. With the planned inclusion of measurement tools such as phenotype and ancestry tracking/logging, it will be able to model various ecological interactions (e.g. competition, parasitism, predation, mutualism).


Finchly is inspired by projects such as **Tierra** and **Avida**. In particular, Avida is used in biological research to model ecological interactions and is under active development.

# Developer Documentation

Key documentation for this project can be found in this README. 

Each Finchly simulation is encapsulated in an `Environment` struct, containing the key variables about the simulation and tells the simulation how to behave. This struct has information about the simulation such as population cap behavior and world topology. 

Each simulation has a number of `Finch` structs, which are the simulated organisms in Finchly. For each cycle, the simulation will allow for mutation of each finch as well as reward each finch with extra clock cycles if it completes certain tasks. These tasks are often logical tasks and are administered by an IO manager.

More details about this project can be found here:

1. [Instruction Sets](#instruction-sets)
2. [World Topology](#world-topology)
3. ...

 # Instruction Sets

The current default instruction set is similar to the instructions available in Avida, with the same key behavior.

Finchly and Avida both have No-Operation instructions, and have multiple variants which are used as modifiers for preceding instructions and as labels.

# World Topology

The current default topology is a bounded $\mathbb{R}^2$ with Moore neighbourhood. A finch in this topology may only interact with neighbouring finches.

There will also be support for a 2D toriodal grid with Moore neighbourhood, and a fully connected topology. In the latter, each finch is a neighbour to every other finch.

 





