# Chapter 1 

## Exercises

### 1.1 - Calculate the maximum number of class A, B & C network IDs 

```
-----------------------------------------
                 |  7 bits |     24 bits
A        0       |  netid  |     hostid   
-----------------------------------------

-----------------------------------------
                 | 14 bits |     16 bits
B        10      |  netid  |     hostid
-----------------------------------------


-----------------------------------------
C        110     | 21 bits |     8 bits
                 |  netid  |     hostid
-----------------------------------------

-----------------------------------------
D        1110    |      28 bits 
                 |  multicast group id
-----------------------------------------

-----------------------------------------
E        1111    |      28 bits 
                 |  reserved for future use
-----------------------------------------
```

There are 7 bits used for a class A network ID, so there can be a maximum of 128 (2^7) class A network ID. One of these
is used for the loopback address and a network ID of all 1s indicates a broadcast address. This means there are 126 
addresses.

There are 14 bits used for a class B network ID, so there can be a maxium of 16384 (2^14) class B network IDs.

There are 21 bits used for a class C network ID, so there can be a maximum of 2,097,152 (2^21) class C network IDs.

### 1.2 - Fetch file from nic.merit.edu

Unfortunately, this subdomain no longer points to anything useful. 

### 1.3 - Obtain a copy of the Host Requirements RFC and lookup the robustness principle. What is the reference for this principle?

[The robustness principle](https://datatracker.ietf.org/doc/html/rfc1122#section-3)

[reference](https://datatracker.ietf.org/doc/html/rfc791#section-3.2)

>>The implementation of a protocol must be robust.  Each implementation
must expect to interoperate with others created by different
individuals.  While the goal of this specification is to be explicit
about the protocol there is the possibility of differing
interpretations.  In general, an implementation must be conservative
in its sending behavior, and liberal in its receiving behavior.  That
is, it must be careful to send well-formed datagrams, but must accept
any datagram that it can interpret (e.g., not object to technical
errors where the meaning is still clear).

### 1.4 - Obtain a copy of the Assigned Numbers RFC and find the port for the quote of the day protocol

Port 17

### 1.5 - Join ISOC

Joined... they actually have some sweet courses for free.


