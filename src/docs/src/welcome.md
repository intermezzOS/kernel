# Welcome to Nucleus

Hi there! Welcome to the Nucleus project. I’m building an operating sytem
in [Rust], and this is its primary documentation.

[Rust]: https://www.rust-lang.org/

It’s a bit more than just documentation though. Nucleus is guided by three
principles, currently:

1. Learning new things is awesome.
2. Being a beginner at something isn’t bad.
3. Documenting a thing is more important than the thing itself.

## Learning new things is awesome

Here’s the source of the first principle:

>  A language that doesn’t affect the way you think about programming, is not
>  worth knowing. - [“Epigrams in Programming”][epi] #19

[epi]: http://www.cs.yale.edu/homes/perlis-alan/quotes.html

I have mostly been doing web development in my professional life, but started
off doing much more low-level programming. And in college, I did some kernel
hacking with my group of friends. But once I found Ruby and Rails, I dove
straight into web-based stuff. No need to fight with manual memory management
and all the frustrations of really low-level code.

But as I’ve worked on Rust over the past three years, I’ve come back around to
the lower-level stuff. It’s nice to learn things over again. It’s also nice to
get out of your comfort zone, generally.

So, when Philipp Oppermann started writing [a tutorial on OS development in
Rust][phil], I decided to give it a shot one night. I followed along, and was
hooked. There’s something really magic about printing `OK` to a screen by
poking video memory. I was hooked.

[phil]: http://os.phil-opp.com/multiboot-kernel.html

## Being a beginner at something isn’t bad

So I’ve been thinking a lot about this. Why was I so much more enthused this
time around?

For that, we have to look at one of the best resources out there for learning
how to do hobby OS development: [the OSDev Wiki](http://wiki.osdev.org/).

Here’s [one of the first pages](http://wiki.osdev.org/Required_Knowledge)

> Writing an OS is not a beginner’s task. In fact, writing an OS is usually
> considered the most difficult programming task. You will need above-average
> programming skills before even considering a project like this. Failure to
> comply will make you look silly. 

This anti-beginner stance is common throughout.
[Another page](http://wiki.osdev.org/Beginner_Mistakes):

> Because this place can not and does not cater for beginner developers, the
> question for some other place that does provide a tutorial, good explanations
> or easy to understand reading is often requested. However, they do not exist.

Of course, the OSDev wiki cannot cater to everyone. They’re free to not cater
to beginners. But that’s why it’s so hard to get involved. Everyone who does
this kind of development “just knows” stuff, and it’s tough for a beginner to
know where to get started.

By contrast, Phil’s tutorial is really great: it walks you through the steps,
it doesn’t treat you as being stupid just because you’re a beginner, and it’s
fairly easy to follow.

This was the difference. I like things friendly, and especially beginner
friendly. “Suffer in order to learn” is a poor, outdated, religious attitude to
take.

Anyway, using Phil’s tutorial and some other resources, I managed to get my
little OS up and running, to the point where it can get keyboard input. That by
itself took me a really long time. But it works! So cool.

## Documenting a thing is more important than the thing itself.

So I got it working... but do I really understand this, or am I just repeating
a tutorial? 

Here’s a quote I’ve always enjoyed.

> “Writing is nature’s way of telling us how lousy our thinking is.” - Leslie
> Lamport

Three years ago, I started learning Rust, and wrote down my learning experience,
which became “Rust for Rubyists”, the first community tutorial for Rust.

So why not do the same for OS Dev?

This project is going to take a lot longer than winter break, though. I don’t
know where it will start and stop. I’m going to try to do a work log, and then
once I get things done, write up tutorials explaining things. So you can read
either way; the finished tutorial, or the in-progress struggles. Both should be
interesting for different reasons.

This is an area where I know very little, the details really matter, and I’m
learning a lot. So I may make mistakes. Please [open an issue] if there’s
something that I’ve overlooked or misrepresented. This is not “Steve knows all
and is handing down The Tutorial from on high”, it’s “Steve is writing stuff
down and sharing it as he learns in the hopes that it will encourage you to
learn too.”

And let me know if you start a little OS of your own!

[open an issue]: https://github.com/steveklabnik/nucleus/issues/new
