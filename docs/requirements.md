# The requirements and feature requests

## Add tattva util to be provided as a part of the library itself

## Digesting a config file, that is a sibling of client's Cargo.toml for video resolution etc configs

## Video export by default. 

## A construct to add in scene that will request a scheenshot saving when control flow reaches them. Whenever this construct is executed, a scheenshot at that point in time will be saves.

## Add an example where graph is being shown drawn on a 2d/3d axes.

## Support to have an image or a construct(preferrably AUTH logo) to be optionally permanently visible in a corner, maybe upper right (configurable)

## Agentic loop/ flow chart

A construct representing agentic loops. Some rectangular bixes, with lines connecting them.  ABility to provide shape of any box in the loop. Also the ability to indicate flow, just like SIgnalFlow implementation that we have for the neural network. It should have the ability to connect forward block with the blocks that came earlier.

The signal can start from the start, and follow some path.  The path that is follows will be user provided example, box 0 -> box 1 -> box2 -> bx 1 -> box2 -> box3[end].

When the signal reaches a box, it will do indicate animation (Just like manim indicate).

Usecase: We will use this construct to showcase agentic flow, workflows, deterministic pipelines, and flow chart in general.


This should be configurable to allow vertical as well as horizontal placement of the construct.



## Neural network v2
ABility to deactivate some node in the neural network.
Also ability to pass SignalFLow through all the path in the neural network simultaneously. The deactivated node will not have paths and signalFlow coming out of them.