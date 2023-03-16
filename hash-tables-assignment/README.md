# Hash Tables Exercises – coding can be in a language of your choice

## 1) Describe with examples 2 different collision handling methods for inserting new entries into a hash table

Collision handling is necessary when a key is run through the hash function, but the hash is the same as another already present using a different key.

To fix this, we have to place our key in another spot than previously thought.

There are many algorithms to fix this problem, but two of them are called `open addressing` and `closed addressing`

### Open Addressing

If a collision happens, we simply look for another index that is open for usage. This can be done in different ways, but a popular one is through linear probing.

Linear probing simply traverses table linearly for an index that is free. This is done both when inserting and retrieving.

### Closed Addressing

Instead of using other indexes of the table, we use the same index for every key that returns the same hash. This is done by having the value of each of our tables be a linked list, and each key for a specific hash is placed somewhere in the list.

Retrieval of keys are then done by doing a normal traversal through a linked list.

A way of making collisions happen less is to increase the table size. This way you are less likely to get the same index.

## 2) Explain the MD5 hash – what is it, what is it used for and what are the strengths/weaknesses of it. Use examples

MD5 is a cryptographic hashing function, which was originally made to be used for all forms of hashing, such as password hashing. As technology improved, collisions where discovered in the algorithm, making it not suitable for places where collisions are not acceptable.

Instead, today it is used for places where no collision is less important, or close to impossible (files).

The MD5 hashing function is used for checking if something is what they say they are. An example of this is uploads and downloads of files on the internet. When a file is uploaded, a hash is generated for that file, and when downloaded, the hash is checked to make sure that the file has not been tampered with.

Other examples of MD5 is for ssh fingerprinting.

## 3) What is a Bloom filter and using examples show how and why it is useful. What are the weaknesses of Bloom filters

A bloom filter used to figure out if something has been seen before. However, A bloom filter is what's known as an approximate data structure, since it will not always give you the correct result.

A bloom filter works like this:

1. Given a key
2. Run the key through __multiple__ hash functions, which all give different results.
3. Check if all the keys are stored yet. If one of them are not, we have not seen that key yet.

A bloom filter is called approximate, because it may produce false positives.
If all the produced indexes from our hash functions collide with either an index created by itself or one of the other hash functions, the function will return true, even if the key is not the one we have already seen.

## 4) See the questions here – they come with solutions – add code to demonstrate the

solutions and additional comments
https://www.geeksforgeeks.org/practice-problems-on-hashing/
