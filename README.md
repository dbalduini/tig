# tig

A lightweight git clone.

### Commands:
- init
- cat-file
- hash-object
- update-ref

# Get Started

`tig` doesn't provide high level apis such add and commit. 
But it is possible to achieve using a few git commands.

## Initialize repo

```
tig init .
```

## Adding file

Since the index is so hard to implement, I will manually use update-index

git add will do:

```

tig hash-object -w blob README.md

git update-index --add --cacheinfo 100644 <blob-hash> README.md

```

Git status should show the README.md file as added now.

```
git status
```


## Commiting

After updating the index, this is, saving the state of the stagin area, we can create a tree object of this state.

Commiting a tree will create a commit object

```
git write-tree

git cat-file -p <tree-hash>

git commit-tree -m "Initial Commit" <tree-hash>

git cat-file -p <commit-hash>

git log --stat <commit-hash>
```

Pointing to this commit hash will

```
tig update-ref refs/heads/master <commit-hash>
```

Git log will now show the commit message.

```
git log
```
