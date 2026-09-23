#!/usr/bin/env bash
# Execute the guide's actual single-quoted `avon run` examples, not copied code.
set -euo pipefail
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
export AVON="${AVON:-$PROJECT_ROOT/target/release/avon}"
TEST_DIR="$(mktemp -d)"
trap 'rm -rf "$TEST_DIR"' EXIT
cd "$TEST_DIR"
perl - "$PROJECT_ROOT/tutorial/GETTING_STARTED.md" <<'PERL'
use strict;
use warnings;
local $/;
open my $doc, '<', $ARGV[0] or die $!;
my $text = <$doc>;
# Extract the actual local-import library example into the isolated fixture dir.
my ($utils) = $text =~ /```avon\n(# utils\.av[^`]+)```/;
defined $utils or die "Missing utils fixture in guide\n";
open my $fixture, '>', 'utils.av' or die $!;
print {$fixture} $utils;
close $fixture;
my ($count, $failed) = (0, 0);
while ($text =~ /^avon run '([^']*)'\n# Output: ?([^\n]*)(\n(?:# [^\n]*\n?)*)?/mg) {
    my ($code, $expected, $rest) = ($1, $2, $3 // '');
    if ($expected eq '') {
        $rest =~ s/^\n//;
        $rest =~ s/^# ?//mg;
        $expected = $rest;
        $expected =~ s/\n+$//;
    }
    # These two examples deliberately depend on the environment.
    local $ENV{HOME} = '/home/yourusername';
    local $ENV{NONEXISTENT};
    delete $ENV{NONEXISTENT};
    $expected =~ s/  \(or your actual home path\)$//;
    open my $process, '-|', $ENV{AVON}, 'run', $code or die $!;
    my $actual = <$process> // '';
    my $ok = close $process;
    $actual =~ s/\n$//;
    # Simple dictionary examples have unspecified display order.
    my $normalize = sub {
        my ($s) = @_;
        return join(', ', sort split /, /, $1) if $s =~ /^\{([^{}]*)\}$/;
        return $s;
    };
    ++$count;
    unless ($ok && $normalize->($actual) eq $normalize->($expected)) {
        warn "FAIL: $code\nExpected: $expected\nActual: $actual\n";
        ++$failed;
    }
}
die "Too few guide examples checked: $count\n" if $count < 90;
print "Getting Started: $count actual documented commands checked, $failed failures\n";
exit($failed ? 1 : 0);
PERL