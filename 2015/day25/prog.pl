#!/usr/bin/perl -w
use strict;
use IO::File;

sub run {
  my $file = shift;
  my ($row, $col) = load_input($file);
  my $n = calc_n($row, $col);
  
  print "n=$n\n";
  my $value = 20151125;
  for (my $i = 1; $i <= $n; $i++) {
    $value *= 252533;
    $value %= 33554393;
    print "Done ${i}...\n" if $i % 1000 == 0;
  }

  print "Value at ${n}th slot is ${value}.\n";
}

sub load_input {
  my $fh = IO::File->new($_[0]);
  while (<$fh>) {
    chomp;
    return ($1, $2) if /Enter the code at row (\d+), column (\d+)/;
    die "Didn't get expected input? Was $_\n"
  }
}

sub calc_n {
  my ($row, $col) = @_;
  my $diag_num = $row + $col - 1;
  # if we're on diagonal D+1, then diagonals 1..D are fully complete,
  # meaning N is the sum of 1..D, plus our offset into this row
  # (and then minus 1 because we're programmers)
  # and our offset is just our column number
  return int(($diag_num - 1) * $diag_num * 0.5) + $col - 1;
}

run($ARGV[0]);
