#!/usr/bin/perl
use Data::Dumper;
use File::Slurp;
use List::Util qw/sum min max reduce/;
use POSIX qw/ceil floor/;

sub parse_input {
  my $f = shift;
  my @lines = split /\n/, read_file($f);
  my @ret;
  my $team = "";

  sub parse_traits {
    my $txt = shift;
    my %traits = map { $_ => [] } qw/weak immune/;
    if ($txt) {
      die "Bad traits: $txt\n" unless $txt =~ / \((.*?)\)/;
      my @segments = split /;\s*/, $1;
      foreach my $segment (@segments) {
        if ($segment =~ /(.*) to (.*)/ && exists $traits{$1}) {
          push @{$traits{$1}}, split(/,\s*/, $2);
        } else {
          die "Bad trait segment: $segment\n";
        }
      }
    }
    return %traits;
  }

  my $tc = -1;
  foreach (@lines) {
    next if $_ eq "";
    if (/^([A-Za-z ]+):/) {
      $team = $1;
      $tc = 1;
      next;
    }
    die "Bad line: $_\n" unless
      /([0-9]+) units each with ([0-9]+) hit points( \(.*\))? with an attack that does ([0-9]+) ([a-z]+) damage at initiative ([0-9]+)/;
    push @ret, {"id" => @ret + 1, "name" => $tc, "team" => $team, "units" => int($1),
      "hp" => int($2), parse_traits($3), "damage" => int($4), "flavor" => $5,
      "initiative" => int($6)};
    $tc++;
  }
  return @ret;
}

sub str_group {
  my $group = shift;
  my @traits;
  for my $tt (qw/immune weak/) {
    push @traits, "$tt to " . join(", ", @{$group->{$tt}}) if @{$group->{$tt}};
  }
  my $traits = @traits ? " (" . join("; ", @traits) . ")" : "";
  return sprintf("%s group %d: %d units each with %d hit points%s with an attack that does %d %s damage at initiative %d",
    $group->{"team"}, $group->{"name"}, $group->{"units"}, $group->{"hp"}, $traits,
    $group->{"damage"}, $group->{"flavor"}, $group->{"initiative"});
}

sub print_counts {
  my $groups = shift;
  my %teams;
  foreach my $group (@$groups) {
    my $s = sprintf("Group %d contains %d units", $group->{"name"}, $group->{"units"});
    $teams{$group->{"team"}} = [] unless exists $teams{$group->{"team"}};
    push @{ $teams{$group->{"team"}} }, $s;
  }

  foreach my $t (sort keys %teams) {
    print "$t:\n";
    foreach my $g (@{$teams{$t}}) {
      print $g . "\n";
    }
    print "\n";
  }
}

sub pick_target_order {
  my ($x, $y) = @_;
  return $y->{"units"} * $y->{"damage"} <=> $x->{"units"} * $x->{"damage"} ||
    $y->{"initiative"} <=> $x->{"initiative"};
}

sub calc_damage {
  my ($attacker, $defender) = @_;
  return 0 if grep { $_ eq $attacker->{"flavor"} } @{ $defender->{"immune"} };
  my $mult = (grep { $_ eq $attacker->{"flavor"} } @{ $defender->{"weak"} }) ? 2 : 1;
  return $attacker->{"units"} * $attacker->{"damage"} * $mult;
}

sub pick_target {
  my ($group, $possibles) = @_;
  my @pd = map { [$_, calc_damage($group, $_)] } @$possibles;
  @pd = grep { $_->[1] > 0 } @pd;
  return undef unless @pd;
  @pd = sort { $b->[1] <=> $a->[1] || pick_target_order($a->[0], $b->[0]) } @pd;
  return $pd[0]->[0]->{"id"};
}

sub run_fight {
  my ($groups, $verbose) = @_;

  print_counts($groups) if $verbose;

  my %targets;
  my %targeted;
  for my $group (sort { pick_target_order($a, $b) } @$groups) {
    my @possibles = grep {
      $_->{"team"} ne $group->{"team"} && !exists $targeted{$_->{"id"}}
    } @$groups;

    my $target = pick_target($group, \@possibles);
    if ($target) {
      $targets{$group->{"id"}} = $target;
      $targeted{$target} = 1;
    }
  }

  for my $group (sort { $b->{"initiative"} <=> $a->{"initiative"} } @$groups) {
    next unless $group->{"units"} > 0; # since we don't remove untl post-fight
    my $target_id = $targets{$group->{"id"}};
    unless (defined $target_id) {
      printf("%s group %d has no viable target, does nothing\n",
        $group->{"team"}, $group->{"name"}) if $verbose;
      next;
    }
    my $target = (grep { $_->{"id"} == $target_id } @$groups)[0];
    my $damage = calc_damage($group, $target);
    my $uk = min($target->{"units"}, floor($damage / $target->{"hp"}));
    $target->{"units"} -= $uk;
    printf("%s group %d attacks defending group %d, killing %d units\n",
      $group->{"team"}, $group->{"name"}, $target->{"name"}, $uk) if $verbose;
  }

  for (my $i = 0; $i < @$groups; $i++) {
    if ($groups->[$i]{"units"} <= 0) {
      printf("%s group %d has perished!\n",
        $groups->[$i]{"team"}, $groups->[$i]{"name"}) if $verbose;
      splice(@$groups, $i, 1);
      $i--;
    }
  }
  print "\n" if $verbose;
}

sub run_combat {
  my ($groups, $boost, $verbose) = @_;
  my @copy = map { { %$_ } } @$groups;
  for my $g (@copy) {
    $g->{"damage"} += $boost if $g->{"team"} eq "Immune System";
  }

  my %prevc;
  while (1) {
    my %c;
    foreach my $g (@copy) {
      $c{$g->{"team"}} = 0 unless exists $c{$g->{"team"}};
      $c{$g->{"team"}} += $g->{"units"};
    }
    return %c if keys(%c) == 1;
    if (Dumper(\%prevc) eq Dumper(\%c)) {
      run_fight(\@copy, 1);
      return ("Stalemate" => 0);
    }
    %prevc = %c;
    run_fight(\@copy, $verbose);
  }
}

sub main {
  my ($f, $verbose) = @_;
  my @groups = parse_input($f);
  for my $group (@groups) {
    print str_group($group) . "\n";
  }
  print "\n";
  my ($team, $remaining) = run_combat(\@groups, 0, $verbose);
  print "Remaining units on $team: $remaining\n";
  print "\n";
  for (my $boost = 1; ; $boost++) {
    my ($bteam, $bremaining) = run_combat(\@groups, $boost, $verbose);
    print "With boost $boost, remaining units are on $bteam: $bremaining\n";
    last if $bteam eq "Immune System";
  }
}

main($ARGV[0], @ARGV > 1);
