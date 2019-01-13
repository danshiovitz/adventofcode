#!/usr/bin/perl
use Data::Dumper;
use File::Slurp;
use List::Util qw/sum min max reduce/;
use POSIX qw/ceil floor/;

sub parse_input {
  my $f = shift;
  my @lines = split /\n/, read_file($f);
  my @ret;
  foreach (@lines) {
    die "Bad line $_\n" unless /pos=<(-?[0-9]+),(-?[0-9]+),(-?[0-9]+)>, r=([0-9]+)/;
    push @ret, {"pos" => [int($1), int($2), int($3)], "range" => int($4)};
  }
  return @ret;
}

sub dist_between {
  my ($p1, $p2) = @_;
  return sum(map { abs($p1->[$_] - $p2->[$_]) } (0..2));
}

sub can_reach_exact {
  my ($bot, $point) = @_;
  return dist_between($bot->{"pos"}, $point) <= $bot->{"range"};
}

sub can_reach_approx {
  my ($bot, $point, $scale) = @_;
  for my $axis (0..2) {
    my $p_min = $point->[$axis];
    my $p_max = $point->[$axis] + $scale - 1;
    my $b_min = $bot->{"pos"}->[$axis] - $bot->{"range"};
    my $b_max = $bot->{"pos"}->[$axis] + $bot->{"range"};
    if ($p_min <= $b_min) {
      return 0 if $b_min > $p_max;
    } else {
      return 0 if $p_min > $b_max;
    }
  }
  return 1;
}

sub str_bot {
  my $bot = shift;
  return sprintf("{(%d,%d,%d) - %d}", $bot->{"pos"}->[0], $bot->{"pos"}->[1],
    $bot->{"pos"}->[2], $bot->{"range"});
}

sub main {
  my ($f, $point) = @_;
  my @nanobots = parse_input($f);
  for my $bot (@nanobots) {
    if (can_reach_exact($bot, $point)) {
      print "Exact\n";
    } elsif (can_reach_approx($bot, $point, 10)) {
      my @diffs = map { $bot->{"pos"}->[$_] - $point->[$_] } (0..2);
      my $sum = sum(map { abs($_) } @diffs);
      print "Approx: " . str_bot($bot) . " // @diffs = $sum\n";
    } else {
      print "Nope\n";
    }
  }
}

main($ARGV[0], [int($ARGV[1]), int($ARGV[2]), int($ARGV[3])]);
