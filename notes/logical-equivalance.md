# Logical Equivalence

## Domination

```
P OR T = T
P AND F = F
```

Example:

- I am red haired OR life exists = true
- I am red haired AND life does not exist = false

## Idempotent

```
P OR P = P
P AND P = P
```

Example:

- red OR red = red
- red AND red = red

## Double negation

```
NOT NOT P = P
```

Example:

- I don't consider myself not red haired.

## Commutative

```
P OR Q = Q OR P
P AND Q = Q AND P
```

Example:

- I am red haired OR life exists = life exists OR I am red haired
- I am red haired AND life exists = life exists AND I am red haired

## Negation

```
P OR NOT P = T
P AND NOT P = F
```

Example:

- I am red haired OR I am not red haired
- I am red haired AND I am not red haired

## Associative

```
(P OR Q) OR R = P OR (Q OR R)
(P AND Q) AND R = P AND (Q AND R)
```

Example:

Honestly not sure how to write a good example for this?

- (I am red haired OR I am blue haired) OR I am yellow haired
- I am red haired OR (I am blue haired OR yellow haired)

## Distributive

```
P OR (Q AND R) = (P OR Q) AND (P OR R)
P AND (Q OR R) = (P AND Q) OR (P AND R)
```

Example:

## De Morgan's

```
NOT (P AND Q) = NOT P OR NOT Q
NOT (P OR Q) = NOT P AND NOT Q
```

## Absorption

```
P OR (P AND Q) = P
P AND (P OR Q) = P
```

## Identity

```
P AND T = P
P OR F = P
```

## Biconditional statements

```
P <-> Q = (P -> Q) AND (Q -> P)
P <-> Q = P <-> NOT Q
```

## Conditional statements

```
P -> Q = NOT P OR Q
P -> Q = NOT Q -> NOT P
```
