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
    die "Bad line $_\n" unless /(-?[0-9]+),(-?[0-9]+),(-?[0-9]+),(-?[0-9]+)/;
    push @ret, [int($1), int($2), int($3), int($4)];
  }
  return @ret;
}

sub count_constellations {
  my $stars = shift;
  my %pairs;
  for (my $i = 0; $i < @$stars; $i++) {
    for (my $j = $i + 1; $j < @$stars; $j++) {
      my $md = sum(map { abs($stars->[$i]->[$_] - $stars->[$j]->[$_]) } (0..$#{$stars->[$i]}));
      next unless $md <= 3;
      $pairs{$i,$j} = 1;
    }
  }

  for (my $i = 0; $i < @$stars; $i++) {
    for (my $j = $i + 1; $j < @$stars; $j++) {
      print "pairs $i $j\n" if exists $pairs{$i,$j};
    }
  }

  my $next_group = 1;
  my %groups;
  for (my $i = 0; $i < @$stars; $i++) {
    my @existing;
    for my $group (keys %groups) {
      for my $o (@{$groups{$group}}) {
        if (exists $pairs{$o, $i}) {
          push @existing, $group;
          last;
        }
      }
    }
    print "existing for $i: " . join(",", @existing) ."\n";
    if (@existing == 0) {
      $groups{$next_group++} = [$i];
    } else {
      push @{$groups{$existing[0]}}, $i;
      for my $og (@existing[1..$#existing]) {
        push @{$groups{$existing[0]}}, @{$groups{$og}};
        delete $groups{$og};
      }
    }
  }

  return scalar(keys %groups);
}

sub main {
  my ($f) = @_;
  my @stars = parse_input($f);
  my $cc = count_constellations(\@stars);
  print "Total constellations: $cc\n";
}

main($ARGV[0]);
