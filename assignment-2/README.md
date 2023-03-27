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

The assignment focuses on searching and decoding, specifically in binary trees and the Caesar cipher. The complete assignment can be found [here](./documents/Algorithms-Assignment-2.pdf).

## Tasks

Some code has been implemented, but I ended up not finishing all of it, because of other assignments that I wanted to spend more time on.

### Binary Tree

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

### Two Three Tree

A portion bit of the two three tree has been implemented, and can be found [here](./src/two_three_tree.rs). Insertion is not done, however the general structure of the algorithm is, and a search is implemented.

A two three works to try and make the binary tree stay balanced. A node in a two three tree can be one of three things:

- A leaf node which contains one data element but has no children.
- A two node which holds one data element, and two possible children.
- A three node which holds two data elements, and three possible children.

Insertion into a two three tree works as follows:

Imagine that we want to insert the numbers 30, 10, 50, 70 and 20 in the tree

- 30: 30 becomes the root node as a leaf node.
- 10: the root node becomes a three node where 10 on the left and 30 is on the right.
- 50: 30 becomes the new root node as a two node, where its right node is a leaf node containing 50 and its left is a leaf node containing 10.
- 70: the leaf node containing 50 becomes a three node where the left data is 50 and the right is 70.
- 20: the leaf node containing 10 becomes a three node where the left data is 10 and the right is 20.

You then end up with a tree looking like this:

```
          30
  (10, 20) (50, 70)
```

### Red Black Tree

Like the two three tree a red black tree is a type of balanced binary search tree.

The rules of the red black tree are:

- A node is either black or red
- The root and leaves are black
- If a node is red then its children are black
- All paths from a node to its leaves has the same number of black nodes
- New insertions are always red
- Any null node are considered black

Node names:

- Parent = the parent of the current node
- Grandparent = the parent of the parent of our current node
- Uncle = grandparents child that is not the parent

Rebalancing rules are:

- Black uncle = rotate
  After a rotation, the three nodes, are black, red, and red where black is the parent.
- Red uncle = colour flip
  After a colour flip, the three nodes are red, black, and black where red is the parent.

Example of inserting nodes `5, 10, 7, 8`:

- 5: 5 is inserted as a red node, and then recoloured to black because it is the root node
- 10: 10 is inserted on the right of 5 as red
- 7:

  - 7 is inserted on the left of 10 as red.
  - Because we now have two red nodes we are violating the rules of the tree.
  - The uncle of our recently inserted node is null, so we consider it black meaning we have to rotate.
  - The current imbalance is a right to left imbalance, because 5 goes right then 10 goes left. So we have to do a right-left rotate.
  - 7 becomes the grandparent, 10 goes to the right of 7 and 5 goes to the left of 7.
  - We then have to recolour because 7 and 10 are still two reds in a row.
  - 7 becomes black, and both children become red (as the rebalancing rules also state)

- 8:
  - 8 is inserted to the left of 10 as red.
  - Because we now have two red nodes we are violating the rules of the tree.
  - The uncle of our recently inserted node is red, meaning we have to colour flip.
  - Colour flipping the grandparent, so that 7 becomes red, 5 black, 10 black, and 8 red.
  - We now have a violation in that the root node is red. So we change it to black.

And the cycle continues... HOWEVER, there will be times when the problem tree is incorrect, but not at the place of insertion. (Like the example above with our root colour).
This means that after each insertion, we have to recursively check the parents for inconsistencies with the rules of our tree.
If an inconsistency is found we use the same rebalancing rules used when inserting the node. So either rotate or colour flip.

### Caesar cipher

The code for decoding the provided files using the assumptions given by the assignment can be found [here](src/ceasar_decoder.rs). The code uses the ASCII table to handle dealing with the characters. It's time complexity is O(n) where n is the number of chars in the provided text.

The Caesar cipher is an encoding used which was used back in the times of Caesar. It works like this:
When you encode a piece of text, you decide on a key which is used when encoding.

If the key was 2, and a letter is a, then the encoded version of that letter is c.

Decoding works the same. We have to figure our which key was used to encode the encoded text, and then decode it by taking c and subtracting that with 2 which gives us a.

To figure our the key that was used to encode the text, there are many ways. If you know that the decoded version was written in a specific language you try assuming that the most used letters in the alphabet are also the most used letters in the encoded text. Therefore, if we assume that e is the most common and we and we have a text where d is the most common in the encoded version, that means that it was encoded using a key of -1.

Alternatively you can also brute force it, by simply trying different keys. Eventually you would get it right, assuming that it was written in a language using ASCII characters.
