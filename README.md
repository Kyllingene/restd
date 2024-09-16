# restd

`restd` is a re-implementation of various std features. It started out with
`std::fmt` (still my favorite part), then I had to remake the core of `std::io`
as well. In the future, I plan on implementing things like threads, file and
std{in,out,err}, and whatever strikes my fancy.

## Should I use this?

Well, sure, go ahead...! The eventual hope is that I'll be able to use it
myself. While in 0.1.x there are zero guarantees, but once I publish on
crates.io I'll begin (mostly) adhering to semver.

However, do be warned: this is *not* made to be particularly usable. I did my
best, but it's not always good enough. Some features still rely on `std` and may
always, and others may only be available on Unix (my native platform). PR's are
always welcome though!
