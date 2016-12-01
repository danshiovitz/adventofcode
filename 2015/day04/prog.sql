-- this is postgres
select i
from generate_series(1,100000000) i
where left(md5(concat('iwrupvqb', i)), 6) = '000000'
limit 1;
