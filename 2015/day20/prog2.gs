50:v;{:h,{1+}%{.h\%!\v*h<!and},{+}*11*}:f;{:t,{1+}%{f t<!}?}:g;~g

# Breakdown (with spoiler space)
# .
# .
# .
# .
# .
# .
# .
# .
# .
# .
# .
# .
# .
# .
# .
# .
# .
# .
# .
# .
# .
# .
# .
# .
# .
# .
# .
# .
# This is very similar to part 1, of course.
# 50:v;
# Define a variable v holding the value 50
# {:h,{1+}%{.h\%!\v*h<!and},{+}*11*}:f;
# Create a block (ie, function), bind it to f, then pop the block off
# the stack so it's only called on-demand. The function calculates the
# number of presents received at house h, which it does as follows:
#   Create an array going from 0..h-1
#   Add one to every element so it instead goes 1..h
#   Filter it to only those elements where h % element == 0 and
#     where element * 50 is greater than or equal to h
#   Sum those
#   Multiply the result by eleven
# {:t,{1+}%{f t<!}?}:g;
# Create a function bound to g as above. The function calculates the first
# house that receives at least t presents, which it does as follows:
#  Create an array going from 0..t-1
#  Add one to every element so it instead goes 1..h
#  Find the first element where f(element) is not less than t
# ~g
# Convert the input from a string to a number, then call g on it
