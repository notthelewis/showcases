# Orchestrator

The orchestrator's purpose is to divvy up the work in a concurrent manor. It creates N `Source` objects; where N = the
number of CPU cores. Each `Source` object creates a buffered scanner for the underlying file.

Calling the `ReadChunk` method on an `Orchestrator` object reads a line from each file pointer sequentially. Upon 
reading a line, the `parseLine` method of the 
