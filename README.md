https://open.kattis.com/problems/aliencodebreaking

### Results

 * **Submission 1 - time limit exceeded**
   * The profiling data shows that most time is spent making the table, with
     about half of the time spent in the bignum radix conversion. If one could convert more diretly,
     maybe one could win this one. Maybe it's possible to more quickly build the padding table, too. 
 * **Submission 2 - time limit exceeded**
   * It looks like the 'constant' optimization is very powerful, as it sped up the 'to radix 27'
     conversion considerably. We seems to be loosing too much time during pad generation though.
 * **Submission 3 - time limit exceeded**
   * By now it runs in ~8.2s using a 6/12 intel CPU (MBPro 15" 2018), but it's still too slow
     on the 4/? core xeon they are using for judging. The slowest part now is the transformation
     into a base 27 vector, which is single threaded still.
 * **Submission 4 - runtime error**
   * non-zero exit code, probably some malformed input?
 * **Submission 5 - runtime error**
   * non-zero exit code, probably some malformed input? **It would have been nice...**
