# timestamp_compression_experiments
Expressing posix timestamps in less than 64 bits


# Array Timestamp

### four byte array nearly 30 year timestamp v1

## posix time scale notes
```
(u1 to 1; u2 to 2; u4 to 8)
1  1 			= 1 sec
2  10			= 10 sec
(u8 to 256)
3  100		= 1.67 min
(u16 to 65,536; 256^2)
4  1000		= 16.7 minutes
5  10000		= 2.7 hours
(u32 to 16,777,216; 256^3)
6  100000		= 1.157 days / 0.165 weeks
7  1000000 	= 0.381 months / 1.65 Weeks
8  10000000	= 3.81 months / .317 years
(u64 to 4,294,967,296; 245^4)
9  100000000	= 3.17 years
10 1000000000	= 31.7 years
11 10000000000	= 317 years
12 100000000000	= 3171 years
```

## Compressed nonce-like timestamp freshness proxy


Use a four u8 byte array to get a nearly 31 year nonce timestamp

You need 8 digits: (skip the seconds digit)
```
10 (10sec) ->  100000000 (3.17 years)
+
some information about the 10th digit
```

byte 1:
- digit 2 		(in the ones place)
- digit 3 		(in the tens place)
- fragment-1	(in the hundreds' place), not mod !%2

byte 2:
- digit 4 		(in the ones place)
- digit 5 		(in the tens place)
- fragment-2	(in the hundreds' place), not mod !%3

byte 3:
- digit 6 		(in the ones place)
- digit 7 		(in the tens place)
- fragment-3	(in the hundreds' place), not 0 or 4

byte 4:
- digit 8 		(in the ones place)
- digit 9 		(in the tens place)
- fragment-4	(in the hundreds' place), is prime

10th digit fragments:
1. not mod !%2
2. not mod !%3
3. not 0 or 4
4. is prime

## One Collision
Only 5,7 values collide, but it is something.

### Without Bit Manipulation
This works without bitwise operations (fun though those are).
There are four u8 (unsigned 8-bit) values,
each of which can hold (in decimal terms)
up to 0-255
including 199

The hundres's place can safely be 1 or 0 (though it can be 2 also if we know the whole value is less than 255).

from python:

```python
def is_prime(n):
   prime_numbers = [2, 3, 5, 7]
   if n in prime_numbers:
       return True
   else:
       return False


def not_0_4(n):
   prime_numbers = [0,4]
   if n not in prime_numbers:
       return True
   else:
       return False


for i in range(10):
   # print(f"{i} 2:{i%2} 3:{i%3} 4:{i%4}")
   print(f"{i} {not i%2}{not i%3}{not_0_4(i)}{is_prime(i)}")
```

Output:
```
0 TrueTrueFalseFalse
1 FalseFalseTrueFalse
2 TrueFalseTrueTrue
3 FalseTrueTrueTrue
4 TrueFalseFalseFalse
*5 FalseFalseTrueTrue
6 TrueTrueTrueFalse
*7 FalseFalseTrueTrue
8 TrueFalseTrueFalse
9 FalseTrueTrueFalse
```

