# Heme-meat

Trashy code thrown together to provide a WebUI to allow person/persons to note their attendance for an event.

No auth, no attendence modification, single event.

If this sounds useful, it's probably best to consider using
[Google Forms](https://www.google.com.au/forms/about/) instead


## Running

Export `DATABASE_URL` as a path to sqlite.

Then `diesel migrate`

and finally `cargo run`


The project is staticly set to sqlite for all the pros and cons that are there...
