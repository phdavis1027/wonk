# The Point

Recently, someone forked my graduated project, which was a half-finished iRODS client
in Rust. That made me reminesce, and also remember this cool idea I had to write a 
fully-featured procedural macro that would allow remotely executable rules to be written
in pure Rust.

As I thought about it, that seemed kind of silly. People already have their rules 
written in iRODS rule language. What if they could just generate Rust types for their
existing rulebases? The right way to do that is to write a library and call it from
a build script. That's what `wonkify` does. 

Then I started reading the current Rule Engine code to figure out how on earth to parse
the rule language, and I got to thinking it might be fun to just make a new rule engine.
Theoretically iRODS should allow this because all it requires for a plugin is 
a shared library with functions that match a certain interface. There's not
really a point to this, but it sounds fun.

But also, maybe if the Rust rule engine knows about the same types as the client 
sending them the commands, certain new optimizations would be possible. Who knows!
