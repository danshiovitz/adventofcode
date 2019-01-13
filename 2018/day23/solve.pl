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

sub str_bot {
  my $bot = shift;
  return sprintf("{(%d,%d,%d) - %d}", $bot->{"pos"}->[0], $bot->{"pos"}->[1],
    $bot->{"pos"}->[2], $bot->{"range"});
}

sub strongest_count {
  my $bots = shift;
  my @sorted = sort { $a->{"range"} <=> $b->{"range"} } @$bots;
  my $strongest = $sorted[$#sorted];
  my @dists = grep { $_ <= $strongest->{"range"} } map {
    dist_between($strongest->{"pos"}, $_->{"pos"});
  } @sorted;
  return $strongest, scalar(@dists);
}

sub bounds {
  my ($point, $range) = @_;
  return [$point->[0] - $range, $point->[1] - $range, $point->[2] - $range],
    [$point->[0] + $range, $point->[1] + $range, $point->[2] + $range];
}

sub overlap_bounds {
  my ($a_low, $a_high, $b_low, $b_high) = @_;
  my $low = [max($a_low->[0], $b_low->[0]),
    max($a_low->[1], $b_low->[1]),
    max($a_low->[2], $b_low->[2])];
  my $high = [min($a_high->[0], $b_high->[0]),
    min($a_high->[1], $b_high->[1]),
    min($a_high->[2], $b_high->[2])];
  if ($low->[0] > $high->[0] || $low->[1] > $high->[1] || $low->[2] > $high->[2]) {
    return undef, undef;
  } else {
    return $low, $high;
  }
}

sub scale_bot {
  my ($bot, $scale) = @_;
  return {"pos" => [map {ceil($_ / $scale) } @{$bot->{"pos"}}],
          # add 3 to range to, basically, make sure we always overestimate
          # the reach of the bot based on rounding in pos axes
          "range" => ceil($bot->{"range"} / $scale) + 3};
}

sub can_reach {
  my ($bot, $point, $scale) = @_;
  # approximate:
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

  if ($scale == 1) {
    return dist_between($bot->{"pos"}, $point) <= $bot->{"range"};
  } else {
    return 1;
  }
}

sub same_bots {
    my ($bots1, $bots2) = @_;
    return 0 if @$bots1 != @$bots2;
    for (my $i = 0; $i < @$bots1; $i++) {
      my $p1 = $bots1->[$i]->{"pos"};
      my $p2 = $bots2->[$i]->{"pos"};
      return 0 if $p1->[0] != $p2->[0];
      return 0 if $p1->[1] != $p2->[1];
      return 0 if $p1->[2] != $p2->[2];
    }
    return 1;
}

sub is_inside {
  my ($bot, $min, $max) = @_;
  for my $axis (0..2) {
    my $p_min = $min->[$axis];
    my $p_max = $max->[$axis];
    return 0 if $bot->{"pos"}->[$axis] < $p_min;
    return 0 if $bot->{"pos"}->[$axis] > $p_max;
  }
  return 1;
}


sub region_search {
  my ($bots, $min, $max, $scale, $best_count) = @_;

  my (@yes, @maybe, @no);
  for my $bot (@$bots) {
    my $reach_min = can_reach($bot, $min, $scale);
    my $reach_max = can_reach($bot, $max, $scale);
    if ($reach_min && $reach_max) {
      push @yes, $bot;
    } else {
      if ($reach_min || $reach_max || is_inside($bot, $min, $max)) {
        push @maybe, $bot;
      } else {
        push @no, $bot;
      }
    }
  }

  print "Yes: " . @yes . " No: " . @no . " Maybe: " . @maybe . "\n" if $scale > 1;

  if (@maybe == 0) {
    if (@yes == 0) {
      print "No bots can reach @$min\n" if $scale > 1;
      return ();
    } else {
      print "All bots can/can't reach @$min: " . @yes . " " . @no . "\n" if $scale > 1;
      return ([$min, \@yes]);
    }
  }

  if (@yes + @maybe < $best_count) {
    print "Can't beat threshhold, giving up\n" if $scale > 1;
    return ();
  }

  my @regions;
  for (my $x = $min->[0]; $x < $max->[0]; $x += $scale) {
    for (my $y = $min->[1]; $y < $max->[1]; $y += $scale) {
      for (my $z = $min->[2]; $z < $max->[2]; $z += $scale) {
        my $point = [$x,$y,$z];
        my @matches = @yes;
        for my $bot (@maybe) {
          push @matches, $bot if can_reach($bot, $point, $scale);
        }
        push @regions, [$point, \@matches] if @matches;
      }
    }
  }
  # # for any particular list of bots, keep only the best region
  # @regions = sort { dist_between([0,0,0], $a->[0]) <=> dist_between([0,0,0], $b->[0]) } @regions;
  # my @keep;
  # while (@regions) {
  #   my $r = shift @regions;
  #   push @keep, $r;
  #   @regions = grep { !same_bots($r->[1], $_->[1]) } @regions;
  # }
  # return @keep;
  return @regions;
}

sub bot_bounds {
  my ($bots) = @_;
  my $min_x = min(map { $_->{"pos"}[0] - $_->{"range"} } @$bots);
  my $min_y = min(map { $_->{"pos"}[1] - $_->{"range"} } @$bots);
  my $min_z = min(map { $_->{"pos"}[2] - $_->{"range"} } @$bots);
  my $max_x = max(map { $_->{"pos"}[0] + $_->{"range"} } @$bots);
  my $max_y = max(map { $_->{"pos"}[1] + $_->{"range"} } @$bots);
  my $max_z = max(map { $_->{"pos"}[2] + $_->{"range"} } @$bots);
  return [$min_x,$min_y,$min_z], [$max_x,$max_y,$max_z];
}

sub better_result {
  my ($result, $other) = @_;
  return 1 unless defined $other;
  return 0 unless defined $result;
  return 1 if @{$result->[1]} > @{$other->[1]};
  return 0 if @{$other->[1]} > @{$result->[1]};
  return 1 if dist_between($result->[0], [0,0,0]) < dist_between($other->[0], [0,0,0]);
  return 0;
}

sub refinement_search {
  my ($bots, $scale, $min, $max, $best_so_far) = @_;
  if (!defined $min) {
    ($min, $max) = bot_bounds($bots);
    $min = [map {floor($_/$scale) * $scale} @$min];
    $max = [map {ceil($_/$scale) * $scale} @$max];
    # assume our best will be at least 50%, so cut out the part where we slowly
    # increase our best to reach that number:
    $best_so_far = [[0,0,0], [(undef) x int(@$bots/2)]];
  }
  return undef unless better_result([$min, $bots], $best_so_far);

  print "Trying @$min - @$max @ $scale (" . @$bots . ")\n" if $scale > 1;

  my @regions = region_search($bots, $min, $max, $scale, scalar(@{$best_so_far->[1]}));
  @regions = sort { @{$b->[1]} <=> @{$a->[1]} }
    grep { better_result($_, $best_so_far) } @regions;

  if ($scale > 1) {
    my @refined;
    while (@regions) {
      my $region = shift @regions;
      my $rmin = $region->[0];
      my $rmax = [map {$_ + $scale - 1} @$rmin];
      my $result = refinement_search(
        $region->[1], ceil($scale/10), $rmin, $rmax, $best_so_far);
      if ($result && better_result($result, $best_so_far)) {
        push @refined, $result;
        $best_so_far = $result;
        print "Best is now " . @{$best_so_far->[1]} . " @ (" . join(",", @{$best_so_far->[0]}) . ") - $scale\n";
        # eliminate anything where the overestimate can't beat our actual
        @regions = grep { better_result($_, $best_so_far) } @regions;
      }
    }
    @regions = @refined;
  }

  return reduce { better_result($a, $b) ? $a : $b } @regions;
}

sub most_common_point {
  my $bots = shift;
  # Theory: find bots that can reach everything other bots can reach, or
  # can't reach anything another bot can reach
  # my @sorted = sort { $b->{"range"} <=> $a->{"range"} } @$bots;
  # for (my $i = 0; $i < @sorted; $i++) {
  #   for (my $j = $i + 1; $j < @sorted; $j++) {
  #     my $dist_to = dist_between($sorted[$i]->{"pos"}, $sorted[$j]->{"pos"});
  #     if ($sorted[$i]->{"range"} - $dist_to >= $sorted[$j]->{"range"}) {
  #       print "$i " . str_bot($sorted[$i]) . " subsumes $j:" . str_bot($sorted[$j]) . "\n";
  #     }
  #     my ($i_low, $i_high) = bounds($sorted[$i]->{"pos"}, $sorted[$i]->{"range"});
  #     my ($j_low, $j_high) = bounds($sorted[$j]->{"pos"}, $sorted[$j]->{"range"});
  #     my ($low, $high) = overlap_bounds($i_low, $i_high, $j_low, $j_high);
  #     if (!defined($low)) {
  #       print "$i " . str_bot($sorted[$i]) . " has no overlap with $j:" . str_bot($sorted[$j]) . "\n";
  #     }
  #   }
  # }

  # # theory: does it help to look at just one axis? say, x
  # my @vals = ([], [], []);
  # for (my $i = 0; $i < @$bots; $i++) {
  #   for (my $axis = 0; $axis < 3; $axis++) {
  #     my $min = $bots->[$i]->{"pos"}->[$axis] - $bots->[$i]->{"range"};
  #     my $max = $bots->[$i]->{"pos"}->[$axis] + $bots->[$i]->{"range"};
  #     push @{$vals[$axis]}, {"type" => "min", "val" => $min};
  #     push @{$vals[$axis]}, {"type" => "max", "val" => $max};
  #   }
  # }
  #
  # my $peak_point = [];
  # for (my $axis = 0; $axis < 3; $axis++) {
  #   $vals[$axis] = [sort { $a->{"val"} <=> $b->{"val"} } @{$vals[$axis]}];
  #
  #   my $active = 0;
  #   my $peak = 0;
  #   my $peak_min = 0;
  #   my $peak_max = 0;
  #   for (my $i = 0; $i < @{$vals[$axis]}; $i++) {
  #     if ($vals[$axis]->[$i]->{"type"} eq "min") {
  #       $active++;
  #       if ($active > $peak) {
  #         $peak = $active;
  #         $peak_min = $vals[$axis]->[$i]->{"val"};
  #       }
  #     } else {
  #       if ($active == $peak) {
  #         $peak_max = $vals[$axis]->[$i]->{"val"};
  #       }
  #       $active--;
  #     }
  #     # print "$i: " . $vals[$i]->{"val"} . " - $active\n";
  #   }
  #   print "Peak for $axis: $peak $peak_min / $peak_max\n";
  #   push @$peak_point, $peak_min;
  # }
  #
  # my @in_range = grep { $_ } map {
  #   dist_between($peak_point, $_->{"pos"}) <= $_->{"range"} ? 1 : 0
  # } @$bots;
  # print "In range of peak: " . scalar(@in_range) . "\n";
  # $peak_point->[0] += 7000000;
  # $peak_point->[1] += 7000000;
  # $peak_point->[2] += 7000000;
  # my @in_range2 = grep { $_ } map {
  #   dist_between($peak_point, $_->{"pos"}) <= $_->{"range"} ? 1 : 0
  # } @$bots;
  # print "In range of peak--: " . scalar(@in_range2) . "\n";

  # Theory: divide by a factor and do an exhaustive search to get
  # an approximation, then refine
  my $scale = 10**8;
  my $result = refinement_search($bots, $scale);
  my ($cx, $cy, $cz) = @{$result->[0]};
  return $cx, $cy, $cz, dist_between([0,0,0], [$cx, $cy, $cz]);
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
