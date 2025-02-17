/*
See: https://en.wikipedia.org/wiki/Levenshtein_distance

Use code like this to compare and check if the rust code returns
comparable results:

```python
!pip install python-Levenshtein
# or
!pip install distance

# Using python-Levenshtein
from Levenshtein import distance as lev_distance1

def test_levenshtein1():
    # Same test cases as in Rust
    test_cases = [
        ("kitten", "sitting"),
        ("Saturday", "Sunday"),
        ("hello", "world"),
        ("rust", "dust"),
        ("", "test"),
        ("test", "")
    ]

    for s, t in test_cases:
        distance = lev_distance1(s, t)
        print(f"Distance between '{s}' and '{t}' is {distance}")

print( test_levenshtein1() )

# Alternative using 'distance' package:
from distance import levenshtein as lev_distance2

def test_levenshtein2():
    # Same test cases as in Rust
    test_cases = [
        ("kitten", "sitting"),
        ("Saturday", "Sunday"),
        ("hello", "world"),
        ("rust", "dust"),
        ("", "test"),
        ("test", "")
    ]

    for s, t in test_cases:
        distance = lev_distance2(s, t)
        print(f"Distance between '{s}' and '{t}' is {distance}")


print( test_levenshtein2() )
```

e.g.
Distance between 'kitten' and 'sitting' is 3
Distance between 'Saturday' and 'Sunday' is 3
Distance between 'hello' and 'world' is 4
Distance between 'rust' and 'dust' is 1
Distance between '' and 'test' is 4
Distance between 'test' and '' is 4
*/


fn levenshtein_distance(s: &str, t: &str) -> usize {
    // Get the lengths of both strings
    let m = s.len();
    let n = t.len();

    // Handle empty string cases
    if m == 0 { return n; }
    if n == 0 { return m; }

    // Create two work vectors
    let mut v0: Vec<usize> = (0..=n).collect();
    let mut v1: Vec<usize> = vec![0; n + 1];

    // Convert strings to vectors of chars for easier indexing
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();

    // Iterate through each character of s
    for i in 0..m {
        // First element of v1 is the deletion cost
        v1[0] = i + 1;

        // Calculate costs for each character of t
        for j in 0..n {
            let deletion_cost = v0[j + 1] + 1;
            let insertion_cost = v1[j] + 1;
            let substitution_cost = v0[j] + if s_chars[i] == t_chars[j] { 0 } else { 1 };

            v1[j + 1] = deletion_cost
                .min(insertion_cost)
                .min(substitution_cost);
        }

        // Swap vectors for next iteration
        v0.clone_from_slice(&v1);
    }

    // Return final distance
    v0[n]
}

fn main() {
    // Test cases
    let test_cases = [
        ("kitten", "sitting"),
        ("Saturday", "Sunday"),
        ("hello", "world"),
        ("rust", "dust"),
        ("", "test"),
        ("test", ""),
    ];

    for (s, t) in test_cases.iter() {
        let distance = levenshtein_distance(s, t);
        println!("Distance between '{}' and '{}' is {}", s, t, distance);
    }
}
