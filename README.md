# parker-word-puzzle

Matt Parker, a rather famous [math youtuber](https://www.youtube.com/user/standupmaths), posted a [video](https://www.youtube.com/watch?v=_-AfhLQfb6w) with the descriptive title:

> Can you find: five five-letter words with twenty-five unique letters?

After explaining the problem setup, he explained that [he had solved it with an brute force approach](https://youtu.be/_-AfhLQfb6w?t=382):

> I could have been more clever. [...] I could have kept trying to hone the code, make it more efficient, but at this point I'm like: 'Look, I've just got to set the code running and move on to other projects'. So I did. I just set the code going and it ran for over a month.'

As soon as he had spoken those words, I, and probably a lot of other people, thought: I can do better. Matt himself quickly put the [solution by the viewer Benjamin](https://gitlab.com/bpaassen/five_clique) in the description of his video. Benjamins solution is based on graph theory and only ran for [roughly 22 minutes](https://gitlab.com/bpaassen/five_clique/-/blob/main/README.md?plain=1#L28). That is already an impressive ~2000x improvement over the 31.5 days of the original solution.

Having no real time have a go at the problem myself I left it at that. Until the Youtube algorithm blessed me with the [video of the user Fred Overflow](https://www.youtube.com/watch?v=947Ewgue4DM). Like Matt, he brute forced the solution.

# Eww, a brute force solution?

His solution in 10 seconds and thus is another ~75x improvement over Benjamins clever solution. How did he do it? By transforming the problem into a better suited representation. I'm going to let you in on his technique in a second, but first let's have deeper look at the problem first.

Brute forcing the solution, we need to compare every word of the `n` possible words with each other word. After that, we compare every pair to every word again and keep that process up until we have groups of five words that are either unique in their letters or not. Even with 'aggressive pruning' as Matt called it, i.e. stopping comparisons early if we already violate the unique letter condition, the brute force solution still has a worst case runtime of `O(n^5)`.

That is quite bad especially for `n` being ~10,000. Maybe we have better luck with the actual operation that is performed and it is really fast? We need to perform two operations:

1. Checking whether two words or groups of words have any overlap in the characters
2. Adding a word to a group of words in case there is no overlap

This screams 'set of characters', doesn't it? Using a hashset as datastructure to represent the characters of a word already gives us a nice way to filter all words that don't have 5 unique letters to begin with, by simply checking the length of the hashset. Of course this will also get rid of anagrams, but for solving the problem that is not a requirement. 

Furthermore, the two operations described above can be achieved really simply with sets:

1. Checking the overlap can be done by simply computing whether or not their intersection is the empty set.
2. Adding two words or groups together is just the set union.

So, that is it, right? We found the perfect representation for the problem. Right? 

Wrong. That is [what Matt parker did](https://github.com/standupmaths/fiveletterworda/). And it took 31.5 days. So, how did Fred Overflow do it?

# A change of perspective

Fred Overflow realized something really important. Or better two things:

1. The number of possible elements in each word or group is very restricted. There are only 26 letters in the English alpabet. While a hashset is certainly able to handle this, we could get away with only 26 bits of information for each word or group.
2. While the operations of intersection and union are certainly able to handle the operations we need for the algorithm, they can do a lot more than we need. 
    - We don't care about the actual intersection of the sets, but only if the intersection is the empty set or not.
    - Similarly, we already know that if we perform the union of the sets, that there will be no duplicate elements.

 With that in mind, Fred Overflow encoded each word as an unsigned integer with 32 bits. Each bit encodes whether or not a letter is present. For this, we only need 26 bits and have the remaining 6 bits to spare. For example, the word 'waltz' is encoded as 
 `10010010000000100000000001` with 'a' being the least significant bit. Since binary is quite hard for us humans to decipher, I'm going to switch to the same display format Fred Overflow also uses: a `0` is shown as dash `-` and a `1` is shown as the letter it represents starting with the least significant bit, i.e. `A`, on the left:

```
A----------L-------T--W--Z  waltz
```

At this point you could justifiably ask, how all of that simplifies the problem. So far it just looks more complicated. And the answer to that is: [bitwise operations](https://en.wikipedia.org/wiki/Bitwise_operation).

To check whether two encoded words share no characters, we only need to compute the [bitwise and](https://en.wikipedia.org/wiki/Bitwise_operation#AND) and check if the result is `0`. For example:

- The words 'waltz' and 'vibex' share no letters:

    ```
    A----------L-------T--W--Z  waltz
    BITWISE AND
    -B--E---I------------V-X--  vibex
    =
    --------------------------
    ```

- The words 'waltz' and 'treck' share the letter 't':

    ```
    A----------L-------T--W--Z  waltz
    BITWISE AND
    --C-E-----K------R-T------  treck
    =
    -------------------T------
    ```

After we have checked that there is no overlap, we can perform the [bitwise or](https://en.wikipedia.org/wiki/Bitwise_operation#OR) to combine two words. For example:

- Combining the words 'waltz' and 'vibex':

    ```
    A----------L-------T--W--Z  waltz
    BITWISE OR
    -B--E---I------------V-X--  vibex
    =
    AB--E---I--L-------T-VWX-Z  waltz and vibex
    ```

- Of course this also works to combine a group of words with another word:


    ```
    AB--E---I--L-------T-VWX-Z  waltz and vibex
    BITWISE OR
    --C----H--K--N------U-----  chunk
    =
    ABC-E--HI-KL-N-----TUVWX-Z  waltz, vibex, and chunk
    ```

So why is this a game changer? Because bitwise operations are blazingly fast. They only take a single CPU operation to compute. This is how Fred Overflow could beat Benjamins clever algorithm with a simple brute force solution. By simply changing the representation of the input data.

# So, what is this repository about?

It seems there is nothing really left to do, right?[^1] And you would be correct if it weren't for the fact that Fred Overflow implemented his solution in [Java](https://github.com/fredoverflow/wordle)[^2]. In fact, in a comment that Fred overflow liked, a user wrote:

> "just a single instruction on your CPU so that's very efficient" **programs in Java** :P

And that is where I come into play. I wanted to brush up my [Rust](https://www.rust-lang.org/) skills for quite some time now. Having never gone further than the examples in the [Rust book](https://doc.rust-lang.org/book/), I was looking for an interesting application that needed a fast language. Thus, this repository exists so I can make some random user on Youtube happy while at the same time use Rust for a 'real world application'.

# How do I use it?

Start by `git clone`'ing this repository

```sh
$ git clone
$ cd parker-wordle
```

Run in release mode to enable all optimizations that help speed things up. The possible words have to be passed in as text files where each line holds exactly one word. Words with more or fewer than five letters as well as anagrams will be automatically filtered. 

```
cargo run --release words_alpha.txt  # 321 solutions
cargo run --release wordle-answers-alphabetical.txt wordle-allowed-guesses.txt  # 10 solutions
```

You can download the word lists that Matt used with
```sh
wget https://raw.githubusercontent.com/dwyl/english-words/master/words_alpha.txt
wget https://gist.githubusercontent.com/cfreshman/a03ef2cba789d8cf00c08f767e0fad7b/raw/c915fa3264be6d35990d0edb8bf927df7a015602/wordle-answers-alphabetical.txt
wget https://gist.githubusercontent.com/cfreshman/cdcdf777450c5b5301e439061d29694c/raw/b8375870720504ecf89c1970ea4532454f12de94/wordle-allowed-guesses.txt
```

[^1]: I'm aware that he has several videos now with improvements to speed things up. Hopefully I'll get to implement them here as well evetually.

[^2]: I'm also aware that he also has implemented his solution in [Go](https://go.dev/). Since this repository is also about me getting more into Rust, I'm just going to ignore that.