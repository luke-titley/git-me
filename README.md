# git-me

This is an alternative git workflow. It's written to work with gitlab only for the moment.

It's a bit like git flow, only develop is rebased on master after each patch release.
It makes a very neat and tidy git history, and also manages your changelogs for you.



```
                                                 develop
                                                    ^
                                                    |
                                                    |
                                                    |
                                                    |
                                                    |
                                                    +<------------+
                                                    |             |
                                                    |             |
                                                    |             |
                                                    |             |
                                                    |             | feature
                                                    |             |
                                                    |             |
                                 master             |             |
                                                    |             |
                                   ^                +-------------+
                                   |                |
                                   |                |
                                   |  ff only merge |
                                   +<----------------<------------+release 0.<minor>.0
                                   |                |
                  +---------------->                |
                  |  no ff merge   |                |
                  |                |                |
          hotfix  |                |                +<------------+
                  |                |                | no ff merge |
                  +----------------+                |             |
                                   |     rebase     |             |
    release 0.0.<patch> +--------->---------------->+             |
                                   |                |             |
                                   |                |             |feature
                                   |                |             |
                                   |                |             |
                                   |                |             |
                                   |                |             |
                                   |                +-------------+
                                   |                |
                                   |                |
                                   +                +
