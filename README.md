# Advent of Code 2023
##### In Rust

---

Wow, what a year it was. With so many changes in life, I thought it would be best to stick to the same language as last year, to have some sense of continuity. Last year I also did not make it to the last day, so here's to second chances. I have abstained from using Rust for the last 11 months, so I am quite Rusty. Let's see where it goes. 

---

### Day 1
#### Part 1
For each line in a file, we are to take the first and last numerical characters from it, combine those into a 2-digit number, and sum all of those up. I originally took the first numerical character times 10, and added the first numerical character of the reversed string. It worked, but I re-wrote large parts of my code when part 2 didn't go as easy as planned.
#### Part 2
For this part we are to also parse written-out digits and take them into the mix. My approach is to first use Regex to replace all the written-out digits with their numerical counterparts. Then apply the same procedure as above. It fails, but I think I have an idea why.

Okay it turns out, as I could have remembered from my formal reasoning courses at uni, that regex does not match overlapping patterns, i.e. if you have the word `abcd`, and the pattern `abc|bcd`, it will only match `abc`. I originaly wrote code without regex, which just replaced strings one-by-one, but that didn't work either. It took me a while to understand that `eightwo`, for example, had to be converted to `82`, and not to `8wo`. Replacing matches one-by-one with the digit, padded by the leading and trailing characters of the written-out digit solved it for me. 