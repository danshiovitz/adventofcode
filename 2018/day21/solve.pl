my $r3 = 0;
my $r4 = 0;
my %r3s;
while (1) {
  $r4 = $r3 | 65536;
  $r3 = 7041048;
  while (1) {
    $r3 += ($r4 & 255);
    $r3 &= 16777215;
    $r3 *= 65899;
    $r3 &= 16777215;
    if ($r4 >= 256) {
      $r4 = int($r4 / 256);
    } else {
      last;
    }
  }
  print "r3 = $r3 (" . scalar(%r3s) . ")\n";
  if (exists $r3s{$r3}) {
    print "Repeat! $r3\n";
  }
  $r3s{$r3} = 1;
}
