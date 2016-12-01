#!/usr/bin/env perl6

sub run(Str $filename) {
  my @registers = qw/a b/;
  my @instructions = parse_instructions($filename, @registers);
  my %state = compute(@instructions, @registers);
  say "Final state:";
  say %state;
}

sub parse_instructions(Str $filename, @registers) {
  return map {
    my $line = $_;
    unless ($line ~~ /(<[a..z]>+)(.*)/) {
      die "Bad line: $line\n";
    }
    my ($opcode, $args) = ($/.list[0].Str, $/.list[1].Str);
    my @args = ($args || "").trim.split(/","\s+/);

    my $parse_func = "&parse_" ~ $opcode;
    if (OUTER::{$parse_func}:exists) {
      # dumb, but now we're one level deeper, meaning OUTER doesn't point to
      # the same thing:
      OUTER::OUTER::{$parse_func}.(@args, $line);
    } else {
      die "Bad line (unknown opcode): $line\n";
    }
  }, $filename.IO.lines;

  sub parse_hlf(@args, $line) {
    validate_len(1, @args.elems, $line);
    my $reg = validate_register(@args[0], $line);
    return sub (%state) { %state{$reg} = (%state{$reg} / 2).floor; return 1; }
  }

  sub parse_tpl(@args, $line) {
    validate_len(1, @args.elems, $line);
    my $reg = validate_register(@args[0], $line);
    return sub (%state) { %state{$reg} = %state{$reg} * 3; return 1; }
  }

  sub parse_inc(@args, $line) {
    validate_len(1, @args.elems, $line);
    my $reg = validate_register(@args[0], $line);
    return sub (%state) { %state{$reg} = %state{$reg} + 1; return 1; }
  }

  sub parse_jmp(@args, $line) {
    validate_len(1, @args.elems, $line);
    my $value = validate_number(@args[0], $line);
    return sub (%state) { %state{"pc"} += $value; return 0; }
  }

  sub parse_jie(@args, $line) {
    validate_len(2, @args.elems, $line);
    my $reg = validate_register(@args[0], $line);
    my $value = validate_number(@args[1], $line);
    return sub (%state) { if (%state{$reg} % 2 == 0) { %state{"pc"} += $value; return 0; } else { return 1; } }
  }

  sub parse_jio(@args, $line) {
    validate_len(2, @args.elems, $line);
    my $reg = validate_register(@args[0], $line);
    my $value = validate_number(@args[1], $line);
    return sub (%state) { if (%state{$reg} == 1) { %state{"pc"} += $value; return 0; } else { return 1; } }
  }

  sub validate_len(Int $expected_size, Int $actual_size, Str $line) {
    die "Bad line (wrong number of args, expected $expected_size): $line"
        if $expected_size != $actual_size;
  }

  sub validate_register(Str $reg, Str $line) {
    return $reg if $reg eq any(@registers);
    die "Bad line (bad register $reg): $line\n";
  }

  sub validate_number(Str $num, Str $line) {
    return +$num if $num ~~ /^("-"|"+")?\d+$/;
    die "Bad line (bad value $num): $line\n";
  }
}

sub compute(@instructions, @registers) {
  my %state = map { $_ => 0 }, @registers;
  %state{"pc"} = 0;
  %state{"a"} = 1;

  while (%state{"pc"} < @instructions) {
    my &cur_inst = @instructions[%state{"pc"}];
    my $advance_pc = &cur_inst(%state);
    %state{"pc"}++ if $advance_pc;
  }

  return %state;
}

sub MAIN($filename) {
  run($filename);
}

sub USAGE() {
  say "Usage: prog.pl <filename>";
}
