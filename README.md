# Chromic

Chromic is a processing engine for video and lighting data. It is designed as a backend, for which various front-ends can be implemented.

It is intended to unify Lighting and Video processing, as a single backend can be used, with a lighting frontend, and a video frontend both interacting with the same graph on the backend.
This will allow very simple integration between the video and lighting systems.

It is designed so that data can flow through the graph from sources to outputs, but metadata can flow backwards.
