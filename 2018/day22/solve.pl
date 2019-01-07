#!/usr/bin/perl
use File::Slurp;
use List::Util qw/sum/;

use constant MOVE_COST => 1;
use constant SWITCH_COST => 7;
# Be better to dynamically calculate the margin as they move into it, but whatever
use constant MARGIN => int(SWITCH_COST/MOVE_COST) * 5;

sub parse_input {
  my $f = shift;
  my @lines = split /\n/, read_file($f);
  foreach (@lines) {
    $depth = $1 if /depth: ([0-9]+)/;
    $target = [$1,$2] if /target: ([0-9]+),([0-9]+)/;
  }
  return $depth, $target;
}

sub gen_grid {
  my ($depth, $target) = @_;
  my ($tx, $ty) = @$target;
  my $max_x = $tx + MARGIN;
  my $max_y = $ty + MARGIN;
  my @grid;
  for my $y (0..$max_y) {
    my @line;
    for my $x (0..$max_x) {
      my $geo_idx = 0;
      if (($x == 0 && $y == 0) || ($x == $tx && $y == $ty)) {
        $geo_idx = 0;
      } elsif ($y == 0) {
        $geo_idx = $x * 16807;
      } elsif ($x == 0) {
        $geo_idx = $y * 48271;
      } else {
        my $ero1 = $line[$x - 1];
        my $ero2 = $grid[$y - 1][$x];
        $geo_idx = $ero1 * $ero2;
      }
      push @line, (($geo_idx + $depth) % 20183);
    }
    push @grid, \@line;
  }

  return \@grid;
}

sub print_grid {
  my $grid = shift;
  my $annotations = shift || {};

  my $map_one = sub {
    my $ero = shift;
    if ($ero % 3 == 0) { return "."; }
    if ($ero % 3 == 1) { return "="; }
    if ($ero % 3 == 2) { return "|"; }
    die "Oops $ero\n";
  };

  foreach my $y (0..$#{$grid}) {
    my $row = $grid->[$y];
    my @chars;
    foreach my $x (0..$#{$row}) {
      if (exists $annotations->{$x,$y}) {
        push @chars, $annotations->{$x,$y};
      } else {
        push @chars, $map_one->($grid->[$y][$x]);
      }
    }
    print join("", @chars) . "\n";
  }
}

sub calc_risk {
  my $grid = shift;

  my $map_one = sub {
    my $ero = shift;
    return $ero % 3;
  };

  my $risk = 0;
  foreach my $line (@{$grid}[0..($#{$grid} - MARGIN)]) {
    my @sl = @{$line}[0..($#{$line} - MARGIN)];
    $risk += sum(map { $map_one->($_) } @sl);
  }
  return $risk;
}

sub fastest_route {
  # I think we can calculate the fastest point to eac spot and bubble out
  # sometimes it might be cheaper to go down and then up, in which case we
  # recalculate a square
  # we probably need to expand by 7 in each dir (nb no negative x/y)
  # (it seems like it's never cost-effective to divert by 7 or more)
  my ($grid, $target) = @_;
  my ($tx, $ty) = @$target;
  my $max_y = @$grid - 1;
  my $max_x = @{$grid->[0]} - 1;

  my $MAX_COST = $max_x * $max_y * SWITCH_COST;

  # not really working to have constants as hash lookup, so just switch to
  # vars
  my $TORCH = 'torch';
  my $CLIMBING = 'climbing gear';
  my $NEITHER = 'neither';

  my @cost = map {
    [map { {$TORCH => $MAX_COST, $CLIMBING => $MAX_COST, $NEITHER => $MAX_COST} } @$_]
  } @$grid;

  $cost[0][0]{$TORCH} = 0;

  my %prev;

  sub fix_tools {
    my ($x, $y, $good_tools, $bad_tool, $cost, $MAX_COST, $prev) = @_;
    if ($cost->[$y][$x]{$bad_tool} != $MAX_COST) {
      die "Shouldn't have $bad_tool at ($x,$y)!\n";
    }
    my $c0 = $cost->[$y][$x]{$good_tools->[0]};
    my $c1 = $cost->[$y][$x]{$good_tools->[1]};
    if ($c0 + SWITCH_COST < $c1) {
      $cost->[$y][$x]{$good_tools->[1]} = $c0 + SWITCH_COST;
      $prev->{($x, $y, $good_tools->[1])} = [$x, $y, $good_tools->[0]];
    } elsif ($c1 + SWITCH_COST < $c0) {
      $cost->[$y][$x]{$good_tools->[0]} = $c1 + SWITCH_COST;
      $prev->{($x, $y, $good_tools->[0])} = [$x, $y, $good_tools->[1]];
    }
  }

  sub neighbors {
    my ($x, $y, $max_x, $max_y) = @_;
    my @ret;
    if ($y > 0) {
      push @ret, [$x, $y-1];
    }
    if ($y < $max_y) {
      push @ret, [$x, $y+1];
    }
    if ($x > 0) {
      push @ret, [$x-1, $y];
    }
    if ($x < $max_x) {
      push @ret, [$x+1, $y];
    }
    return \@ret;
  }

  my @type_tools = (
    [[$TORCH, $CLIMBING], $NEITHER],
    [[$CLIMBING, $NEITHER], $TORCH],
    [[$NEITHER, $TORCH], $CLIMBING],
  );

  my @working = ([0,0]);
  while (@working) {
    my ($wx, $wy) = @{shift @working};
    # print "Recosting ($wx, $wy)\n";
    my ($good_tools, $bad_tool) = @{$type_tools[$grid->[$wy][$wx] % 3]};
    fix_tools($wx, $wy, $good_tools, $bad_tool, \@cost, $MAX_COST, \%prev);
    for my $n (@{neighbors($wx, $wy, $max_x, $max_y)}) {
      my ($nx, $ny) = @$n;
      my ($n_tools, $nbad) = @{$type_tools[$grid->[$ny][$nx] % 3]};
      # print "For ($nx, $ny): our: @$good_tools, theirs: @$n_tools\n";
      my $updated = 0;
      foreach my $tool (@$good_tools) {
        next unless grep {$_ eq $tool} @$n_tools;
        my $cw = $cost[$wy][$wx]{$tool};
        my $cn = $cost[$ny][$nx]{$tool};
        if ($cw + MOVE_COST < $cn) {
          $cost[$ny][$nx]{$tool} = $cw + MOVE_COST;
          $prev{($nx, $ny, $tool)} = [$wx, $wy, $tool];
          $updated = 1;
        }
      }
      if ($updated) {
        push @working, $n;
      }
    }
  }

  my %annotations;
  my @cur = ($tx, $ty, $TORCH);
  while ($cur[0] != 0 || $cur[1] != 0 || $cur[2] ne $TORCH) {
    my @pp = @{$prev{$cur[0], $cur[1], $cur[2]}};
    if ($pp[0] != $cur[0] || $pp[1] != $cur[1]) {
      my $val;
      $val = "T" if $pp[2] eq $TORCH;
      $val = "C" if $pp[2] eq $CLIMBING;
      $val = "N" if $pp[2] eq $NEITHER;
      $annotations{$pp[0], $pp[1]} = $val;
    }
    @cur = @pp;
  }
  $annotations{$tx,$ty} = "*";
  $annotations{0,0} = "*";
  print_grid($grid, \%annotations);
  return $cost[$ty][$tx]{$TORCH};
}

sub main {
  my $f = shift;
  my ($depth, $target) = parse_input($f);
  my $grid = gen_grid($depth, $target);
  my $annot = {};
  $annot->{0,0} = "M";
  $annot->{$target->[0], $target->[1]} = "T";
  print_grid($grid, $annot);
  my $risk = calc_risk($grid);
  print "Calculated risk of $risk\n";
  my $min_cost = fastest_route($grid, $target);
  print "Minimum cost to target: $min_cost\n";
}

main($ARGV[0]);
