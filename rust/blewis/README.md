# blewis (working title)

A redis alternative with some niceties.

# The goals 

1) Replacing RESP
    The first goal is to replace the Redis SErialization Protocol (RESP)[https://redis.io/docs/reference/protocol-spec/] 
    with a _true_ binary protocol. The main reason for this is because I find protocols with a delimiter between fields 
    innefficient when compared with protocols where the encoding is properly specified. My plan is to build a  protocol 
    from the ground up to meet my usecase. I considered using ASN.1 and ProtoBuf, though I want to try and keep external
    dependencies down to almost zero. I want to be accountable for as much of this code as possible. Full specification
    can be found at: PROTO.md

2) Using lock-free data structures for speedy-quick, safe concurrency.
    I want to use lock-free data structures for concurrency, as opposed to mutexes. Redis' codebase is littered with 
    mutexes **which is fine** but I want to try and better understand lock-free data structures, as this is something 
    that has been on my list of things to learn for a while. I *believe* that I may be able to get *some* performance
    speed ups in *some* cases. Even if not, I just want to _truly_ understand lock-free programming. 

3) Support TCP, UDP, unix sockets and HTTP and web sockets out of the box. 
    RESP could _theoretically_ support UDP but it doesn't. It could also _theoretically_ support HTTP but it doesn't.
    The main reasons I want to adopt all of the above are: 1) I want to further develop my skills writing UDP 
    applications 2) It's not a massive stretch to go from TCP to unix sockets. 3) HTTP is super easy. 4) Websockets could 
    open the doors for some cool web stuff.

    Also the more protocols I support the more likely `blewis` is to be actually used by a real human... Providing that
    it's not total dog 💩.

4) Have different mediums of storage **Undecided**
    Redis does support persistence in the form of dumping the dataset to disk periodically or by appending each cmd to a
    disk-based log. These are excellent features and wondering whether to expand on them a little more, adding other 
    storage mediums as options, as opposed to just persisting in-memory storage. 

5) Use the tracing crate to provide good logging and metrics
    I want to learn the tracing crate anyway and this seems like the perfect project to do so. I have chosen to take a 
    dependancy here and not in encoding, as this is less "mission critical".

# The things I'm not looking to do (at least not right away)

- Full blown query language
- Scripting / functions
- Pub / sub
- build a proper CLI 
- Any kind of GUI
