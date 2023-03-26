<div align="center">

# Algorithms assignment 2

By Tobias Zimmermann (cph-tz11@cphbusiness.dk)

</div>

## Table of contents

- [Introduction](#introduction)
- [Tasks](#tasks)
  - [From algorithms](#tasks-from-algorithms)
    - [Binary Tree](#binary-tree)
    - [Two Three Tree](#two-three-tree)
    - [Red Black Tree](#red-black-tree)
  - [Small algorithms project](#small-algorithms-project)

## Introduction

The assignment focuses on searching and decryption, specifically in binary trees and the Caesar cipher. The complete assignment can be found [here](./documents/Algorithms-Assignment-2.pdf).

## Tasks

### Tasks from Algorithms

Some code has been implemented, but I ended up not finishing all of it, because of other assignments that I wanted to spend more time on.

#### Binary Tree

The code for a binary tree has been implemented, and can be found [here](src/binary_tree.rs). I would definitely change the way it is implemented if was to do it again, but it sort of works.

How the binary tree insertion and deletion works is as follows:

Insertion:
1. Search through the tree. If the value we are inserting is more than the value of the current node, we go to the right, otherwise we go left. This means that we are essentially able to skip half of the nodes.
2. If a node is found to be equal then insert a new value at the same place, and put the old equal value as the right of our newly created node.
3. If no node is found to be equal, that means that the value is either the smallest or the biggest value in the tree, so we can insert it at the bottom of our tree.

Deletion:
1. The same searching is done, this time with the value to delete.
2. If a node is found to be equal and:
  - Node has no left. Make the current node to be deleted the right node.
  - Node has no right. Make the current node to be deleted the left node.
  - Node has both a right and a left. Delete the smallest node that can be found from the node to be deleted, and set its value to the current node

#### Two Three Tree

#### Red Black Tree

### Small algorithm project
