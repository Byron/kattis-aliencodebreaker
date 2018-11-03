https://open.kattis.com/problems/aliencodebreaking

### Results

 * **Submission 1 - time limit exceeded**
   * The profiling data shows that most time is spent making the table, with
     about half of the time spent in the bignum radix conversion. If one could convert more diretly,
     maybe one could win this one. Maybe it's possible to more quickly build the padding table, too. 
