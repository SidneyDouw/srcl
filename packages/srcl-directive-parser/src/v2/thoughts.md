-   Parser should do an initial pass where it scans for all the `interface` (and eventually `type`) keywords and checks its contents for `directive syntax`.
    We can just ignore the entire thing if we don't find any directives.

-   Parser should only parse properties when it finds a directive for it

-   Skip the `directives per interface` functionality, this can easily be done from the users' side by wrapping the target component.

-   Can we make the lexer a little bit more efficient? At the moment I feel like it's doing too many lookups
    It is also creating a new iterator over all characters **for each lookup**, not good...

-   Parser should have more / better abstraction layers for the different combinations of "expressions" that exist, might make things easier...

-   Current lexer is already analysing combinations of tokens, that should be the parsers job
