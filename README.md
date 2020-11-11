# git-me

This is an alternative git workflow. It's written to work with gitlab only for the moment.

It's a bit like git flow, only develop is rebased on master after each patch release.
It makes a very neat and tidy git history, and also manages your changelogs for you.

# General rules:
* hotfix is based off master and merges back to master with no ff. So you always get a merge commit.
** hotfix must rebase off master before merging back.
* feature is based off develop and merges back to develop with no ff. So you always get a merge commit.
** feature must rebase off develop before merging back.
* to release features we merge develop ff only into master, then tag with minor bump.
* to release hotfix we tag master with patch bump and rebase develop on top of master.

# Benefits
* develop and master are in lock step. They have simple linear history, that doesn't diverge.
* hotfix and features are all on a single linear history.

# Ultimate goal
Your git history is neat and tidy. Can be a large development team but it looks like it's done by one developer.



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
