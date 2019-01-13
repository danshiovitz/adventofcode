#!/usr/bin/perl
use Data::Dumper;
use File::Slurp;
use List::Util qw/sum min max reduce all/;
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

sub can_reach {
  my ($bot, $min, $max) = @_;
  $max = $min unless defined $max;
  # figure out the nearest point to the bot that's within the min/max cube:
  my $target = [map {
    min(
      max($bot->{"pos"}->[$_], $min->[$_]),
      $max->[$_]
    )
  } (0..2)];
  return dist_between($bot->{"pos"}, $target) <= $bot->{"range"};
}

sub str_bot {
  my $bot = shift;
  return sprintf("{(%d,%d,%d) - %d}", $bot->{"pos"}->[0], $bot->{"pos"}->[1],
    $bot->{"pos"}->[2], $bot->{"range"});
}

sub strongest_count {
  my $bots = shift;
  my @sorted = sort { $a->{"range"} <=> $b->{"range"} } @$bots;
  my $strongest = $sorted[$#sorted];
  my @dists = grep { can_reach($strongest, $_->{"pos"}) } @sorted;
  return $strongest, scalar(@dists);
}

sub better_result {
  my ($result, $other) = @_;
  return 1 unless defined $other;
  return 0 unless defined $result;
  return 1 if @{$result->[0]} > @{$other->[0]};
  return 0 if @{$other->[0]} > @{$result->[0]};
  return 1 if dist_between($result->[1], [0,0,0]) < dist_between($other->[1], [0,0,0]);
  return 0;
}

sub recursive_check {
  my ($bots, $bids, $min, $max, $best) = @_;

  my $md = max(map { abs($max->[0] - $min->[0]) } (0..2));
  print "Querying with " . @$bids . " bots from @$min + $md (best=" .
    ($best ? @{$best->[0]} : "none") . ")\n";
  my @matches = ();

  my @scales = map { max(ceil(($max->[$_] - $min->[$_]) / 8), 1) } (0..2);
  for (my $x = $min->[0]; $x <= $max->[0]; $x += $scales[0]) {
    for (my $y = $min->[1]; $y <= $max->[1]; $y += $scales[1]) {
      for (my $z = $min->[2]; $z <= $max->[2]; $z += $scales[2]) {
        my $cmin = [$x,$y,$z];
        my $cmax = [
          min($max->[0], $x+$scales[0]-1),
          min($max->[1], $y+$scales[1]-1),
          min($max->[2], $z+$scales[2]-1),
        ];

        my @cbids;
        for my $bid (@$bids) {
          push @cbids, $bid if can_reach($bots->[$bid], $cmin, $cmax);
        }
        push @matches, [\@cbids, $cmin, $cmax] if @cbids && @cbids > 950;
      }
    }
  }

  print "mm: " . @$bids . " - " . join(",", (map { scalar(@{$_->[0]}) } sort { @{$a->[0]} <=> @{$b->[0]} } @matches)[-5..-1]);
  @matches = sort { @{$b->[0]} <=> @{$a->[0]} } grep { better_result($_, $best) } @matches;
  print " / " . join(",", (map { scalar(@{$_->[0]}) } @matches)[-5..-1]) . "\n";

  my $is_top = (@$bots == @$bids);
  my $mt = @matches;
  my $mc = 0;
  while (@matches) {
    my $m = shift @matches;
    $mc++;
    print "DONE: $mc / $mt\n" if $is_top;
    my $is_minimal = all { $m->[1]->[$_] == $m->[2]->[$_] } (0..2);
    my $m_best = $is_minimal ? $m :
      recursive_check($bots, $m->[0], $m->[1], $m->[2], $best);
    if (defined $m_best && better_result($m_best, $best)) {
      $best = $m_best;
      @matches = grep { better_result($_, $best) } @matches;
    }
  }
  return $best;
}

sub most_common_point {
  my $bots = shift;

  my $min = [
    min(map { $_->{"pos"}[0] - $_->{"range"} } @$bots),
    min(map { $_->{"pos"}[1] - $_->{"range"} } @$bots),
    min(map { $_->{"pos"}[2] - $_->{"range"} } @$bots),
  ];
  my $max = [
    max(map { $_->{"pos"}[0] + $_->{"range"} } @$bots),
    max(map { $_->{"pos"}[1] + $_->{"range"} } @$bots),
    max(map { $_->{"pos"}[2] + $_->{"range"} } @$bots),
  ];

  my $best = recursive_check($bots, [0..$#{$bots}], $min, $max);
  die "No best could be found!\n" unless defined $best;
  return @{$best->[1]}, dist_between($best->[1], [0,0,0]);
}

sub main {
  my $f = shift;
  my @nanobots = parse_input($f);
  my ($bot, $in_range) = strongest_count(\@nanobots);
  my $bd = str_bot($bot);
  print "Number in range of $bd: $in_range\n";
  my ($cx, $cy, $cz, $cdist) = most_common_point(\@nanobots);
  print "Most common point ($cx,$cy,$cz): $cdist\n";
}

main($ARGV[0]);
