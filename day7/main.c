#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

long long concat(long long lhs, long long rhs) {
  long long rest = rhs;
  while (rest) {
    lhs *= 10;
    rest /= 10;
  }
  long long result = lhs + rhs;
  if (result < 0) {
    return LLONG_MAX;
  }
  return result;
}

int possible(long long target, long long value, int start,
             long long ops[100],  int count, int day2) {
  if (value > target)
    return 0;
  if (start == count && target == value)
    return 1;
  if (start == count)
    return 0;

  return possible(target, value + ops[start], start + 1, ops, count, day2) ||
    possible(target, value * ops[start], start + 1, ops, count, day2) ||
    (day2 && possible(target, concat(value, ops[start]), start + 1, ops, count, day2));
}

int main() {
  FILE *fp = fopen("input.txt", "r");
  if (!fp) {
    return -1;
  }
  long long total_day1 = 0;
  long long total_day2 = 0;
  int read = 0;
  char line[255];
  while(fgets(line, sizeof(line), fp)) {
    char *next = line;
    long long result = strtoll(next, &next, 10);
    long long operands[100];
    int opcount = 0;
    next ++;
    while ((operands[opcount] = strtoll(next, &next, 10))) {
      opcount ++;
    }
    if (possible(result, operands[0], 1, operands, opcount, 0)) {
      total_day1 += result;
    }
    if (possible(result, operands[0], 1, operands, opcount, 1)) {
      total_day2 += result;
    }
  }
  printf("Day 7.1 %lld\n", total_day1);
  printf("Day 7.2 %lld\n", total_day2);
}


