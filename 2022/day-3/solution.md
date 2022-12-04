### Rules

1. Each rucksack has 2 compartments
2. Each item is represented as a letter a-z and A-Z
3. Each type of item must only exist in 1 of the two compartments

eg
This is correct:
```
    Compartment 1: AaaaA
    Compartment 2: BbbbB
```
This is incorrect:
```
    Compartment 1: AaabB
    Compartment 2: BbbaA
```
This is also correct:
```
    Compartment 1: abcdefghijklmnopqrstuvwxyz
    Compartment 2: ABCDEFGHIJKLMNOPQRSTUVWXYZ
    // There are no repeated values between 1 and 2
```

### Data

- The data is provided to us as a list of strings.
- Each string represents the current state of each rucksack.
- The first half of the string, is "Compartment 1", the second half, is "Compartment 2"

### Output

- We need to work out what are the items in each rucksack are repeated
- We then need to map those items to a value (a-z is 1-26, A-Z is 27-52)
- Then we sum the values for the result

### Solution

1. Load data
2. Split into compartment 1 and 2
3. Run matching algorithm to find duplicates
4. Map to values
5. Sum and return