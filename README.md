# Raft-leader | leadership consensus implementation in raft

An implementation of leader election in rust.

- A leader is elected at the start and when the previous leader has stopped sending heartbeats.
- Every node has simulated network delay of up to a few hundred milliseconds when receiving each message.
- Every node randomly goes down once every 3 minutes and stays down for up to 10 seconds.
- Tested on 100 nodes with consensus reached within a few seconds at every election.
- 400 lines of rust code. Simulator included.
