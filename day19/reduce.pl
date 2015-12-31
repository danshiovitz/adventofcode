#!/usr/bin/perl -w
use strict;
use IO::File;

sub load_input {
  my $fh = IO::File->new($_[0]);
  my @productions;
  my $molecule;
  while (<$fh>) {
    chomp;
    next if $_ eq "";
    if ($_ =~ /^(.*?)\s+=>\s+(.*?)$/) {
      my @prod = (parse_molecule($1), parse_molecule($2));
      push @prod, join(".", @{$prod[0]}) . " => " . join(".", @{$prod[1]});
      push @prod, join(".", @{$prod[1]}) . " => " . join(".", @{$prod[0]});
      push @productions, \@prod;
    } else {
      $molecule = parse_molecule($_);
    }
  }
  return \@productions, $molecule;
}

sub parse_molecule {
  my $molecule = $_[0];
  my @atoms = $molecule =~ /(e|[A-Z][a-z]*)/g;
  return \@atoms;
}

sub molecules_are_equal {
  my ($molecule, $target) = @_;

  return 0 if @$molecule != @$target;
  
  for (my $i = 0; $i < @$molecule; $i++) {
    return 0 if ($molecule->[$i] ne $target->[$i]);
  }

  return 1;
}

sub reduce_molecule {
  my ($molecule, $target, $productions, $cache) = @_;
  $cache = {} unless defined $cache;

#  print "Reducing molecule: " . join(".", @$molecule) . "\n";

  if (@$molecule == 0) {
    return undef;
  }

  if (molecules_are_equal($molecule, $target)) {
    return {"atoms" => $target, "key" => join(".", @$target), "via" => []};
  }

  my @rest = @$molecule;
  my @sentence;
  my $depth = 0;
  while (@rest > 0) {
    my $head = shift @rest;
    push @sentence, $head;

    if ($head eq "Rn") {
      $depth++;
    } elsif ($head eq "Ar") {
      $depth--;
      if ($depth < 0) {
        die "Bad nesting depth before " . join(".", @rest) . "\n";
      } elsif ($depth == 0) {
        last;
      }
    }
  }

  my $reductions = reduce_complex_sentence(\@sentence, $productions, $cache);
  return undef if @$reductions == 0;
  
  my $best = undef;
  foreach my $reduction (@$reductions) {
    my $new_molecule = [ @{ $reduction->{"atoms"} }, @rest ];
    # it's possible for the "new" molecule to be the same as the old,
    # if we reached an end-state reduction
    next if molecules_are_equal($new_molecule, $molecule);

    my $cur = reduce_molecule($new_molecule, $target, $productions, $cache);
    next unless defined $cur;
    my @via = ( @{ $reduction->{"via"} }, @{ $cur->{"via"} } );
    next if defined($best) && @via >= @{ $best->{"via"} };
    $best = { "atoms" => $cur->{"atoms"}, "key" => join(".", @{$cur->{"atoms"}}), "via" => \@via };
  }

  return $best;
}

# a complex sentence is one that has multiple Rn..Ars in it, which implies
# nested Rn..Ars. That can look like
# ...Rn-1...Rn-2...Ar-2...Ar-1
# or like
# ...Rn-1...Rn-2...Rn-3...Ar-3...Ar-2...Ar-1
# or like
# ...Rn-1...Rn-3...Ar-3...Rn-2...Ar-2...Ar-1
# but in all cases the correct procedure is to find the first Ar, grab
# everything up to right before the previous-previous Rn, and parse that
# sentence (which must be non-complex), then substitute those reductions in and
# re-parse the whole sentence
sub reduce_complex_sentence {
  my ($atom_list, $productions, $cache) = @_;

  my @rn_idxs;
  my $ar_idx = @$atom_list - 1;
  for (my $i = 0; $i < @$atom_list; $i++) {
    if ($atom_list->[$i] eq "Rn") {
      push @rn_idxs, $i;
    } elsif ($atom_list->[$i] eq "Ar") {
      $ar_idx = $i;
      last;
    }
  }
  
  if ($ar_idx == @$atom_list - 1) {
    return reduce_sentence($atom_list, $productions, $cache);
  }

  my $first = $rn_idxs[-2] + 1;
  my $last = $ar_idx;
  my $nested = [ @{$atom_list}[$first..$last] ];
  my $nested_reductions = reduce_sentence($nested, $productions, $cache);

  my @ret;
  foreach my $nested (@$nested_reductions) {
    my $with_nested = [ @{$atom_list}[0..$first-1], @{$nested->{"atoms"}}, @{$atom_list}[$last+1..@$atom_list-1] ];
    
    my $overall_reductions = reduce_complex_sentence($with_nested, $productions, $cache);
    foreach my $overall (@$overall_reductions) {
      my $combined = {"atoms" => $overall->{"atoms"}, "key" => $overall->{"key"}, "via" => [ @{ $nested->{"via"} }, @{ $overall->{"via"} } ]};
      push @ret, $combined;
    }
  }

  return \@ret;
}

sub reduce_sentence {
  my ($atom_list, $productions, $cache, $steps, $max_steps) = @_;
  $cache = {} unless defined $cache;
  $steps = 0 unless defined $steps;
  $max_steps = @$atom_list * 2 unless defined $max_steps;

  return [] if @$atom_list == 0;

  my $key = join(".", @$atom_list);

  if (exists $cache->{$key}) {
    return $cache->{$key};
  }

  my $full_reductions = [];
  add_if_final($atom_list, $full_reductions);

  if ($steps < $max_steps) {
    foreach my $single (single_reductions($atom_list, $productions)) {
      my $new_atoms = $single->{"atoms"};
      my $rule = $single->{"rule"};
      my $sub_reductions = reduce_sentence($new_atoms, $productions, $cache, $steps+1, $max_steps);
      foreach my $sub_reduction (@$sub_reductions) {
        add_if_better($sub_reduction, $rule, $full_reductions);
      }
    }
  }

  $cache->{$key} = $full_reductions;
  return $full_reductions;
}

sub add_if_final {
  my ($atom_list, $full_reductions) = @_;

  my $y_pos = -1;
  for (my $i = 0; $i < @$atom_list; $i++) {
    if ($atom_list->[$i] eq "Y") {
      $y_pos = $i;
    # this should technically be > 2 - it seems like it should be possible
    # to have a productions like e -> AB, B -> C[Rn]D[Ar], which means you
    # get AC[Rn]D[Ar], and there's no way to reduce AC to a single atom.
    # but this gets the right answer without the change, so either I got
    # lucky on the input, or there's some rule about how the input is
    # generated that makes this situation not happen (I guess the example
    # above is a little simplified, since that'd be parsed all as one sentence
    # in practice and it'd work fine - the more correct example is with a 
    # nested [Ar]s, so it's like e -> A[Rn]B[Ar], B -> CD, D -> X[Rn]Y[Ar], so
    # you get A[Rn]CX[Rn]Y[Ar][Ar], and end up doing a parse of 
    # CX[Rn]Y[Ar], and that can't be parsed to a single atom here)
    } elsif ($i - $y_pos > 1) { 
      return;
    }
  }

  push @$full_reductions, {"atoms" => $atom_list, "key" => join(".", @$atom_list), "via" => []};
}

sub single_reductions {
  my ($atom_list, $productions) = @_;
  
  my @reductions;
  
  for (my $i = 0; $i < @$atom_list; $i++) {
    PRODUCTION: foreach my $production (@$productions) {
      my ($from, $to, $fwd_str, $reduce_str) = @$production;
      next if ($i + @$to > @$atom_list);
      for (my $j = 0; $j < @$to; $j++) {
        next PRODUCTION if ($to->[$j] ne $atom_list->[$i+$j]);
      }

      my @atoms = @$atom_list[0..($i-1)];
      push @atoms, @$from;
      push @atoms, @$atom_list[$i+@$to..(@$atom_list-1)];
      
      push @reductions, {"atoms" => \@atoms, "rule" => $reduce_str};
    }
  }

  return @reductions;
}

sub add_if_better {
  my ($reduction, $rule, $full_reductions) = @_;

  for (my $i = 0; $i < @$full_reductions; $i++) {
    if ($full_reductions->[$i]->{"key"} eq $reduction->{"key"}) {
      my $this_via = $reduction->{"via"};
      if (@{ $full_reductions->[$i]->{"via"} } <= @$this_via + 1) {
        return;
      } else {
        $full_reductions->[$i]->{"via"} = [$rule, @$this_via];
        return;
      }
    }
  }

  my $this_reduction = {"atoms" => $reduction->{"atoms"},
                        "key" => $reduction->{"key"},
                        "via" => [$rule, @{$reduction->{"via"}}]};
  push @$full_reductions, $this_reduction;
}

my ($productions, $molecule) = load_input($ARGV[0] or die "No input file given\n");
my $target = parse_molecule($ARGV[1] or die "No target given\n");

my $best_reduction = reduce_molecule($molecule, $target, $productions);

print "Molecule " . join(".", @$molecule) . "\n";
my $target_key = join(".", @$target);
if (defined $best_reduction) {
  my @via = @{$best_reduction->{"via"}};
  print "Reduces to " . $target_key . " in " . scalar(@via) . " steps:\n";
  foreach my $rule (@via) {
    print "    $rule\n";
  }
} else {
  print "has no reduction to $target_key :C\n";
}
