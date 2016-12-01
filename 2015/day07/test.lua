function blah(x)
  f = function(y) return y*x end
  x = 22
  return f
end

print("hello")
abc = blah(3)
print(abc(7))

