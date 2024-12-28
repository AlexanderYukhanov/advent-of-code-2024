#include <stdlib.h>
#include <stdio.h>

int sort_ints(const void *lhs, const void *rhs) { return *(int *)lhs - *(int *)rhs; }

int main() {
  FILE *fp = fopen("input.txt", "r");
  if (!fp) {
    return EXIT_FAILURE;
  }
  int cnt = 0;
  int cap = 16;
  int *lft = (int*) malloc(cap * sizeof(int));
  int *rht = (int*) malloc(cap * sizeof(int));
  while(2 == fscanf(fp, "%d %d", lft+cnt, rht+cnt)) {
    cnt ++;
    if (cnt == cap) {
      cap *= 2;
      lft = realloc(lft, cap * sizeof(int));
      rht = realloc(rht, cap * sizeof(int));
      if (!lft || !rht) {
        fclose(fp);
        return EXIT_FAILURE;
      }
    }
  }
  fclose(fp);

  qsort(lft, cnt, sizeof(int), sort_ints);
  qsort(rht, cnt, sizeof(int), sort_ints);

  long long dist = 0;
  for (int i = 0; i != cnt; i++) {
    dist += abs(lft[i] - rht[i]);
  }
  printf("Day 1.1: %lld\n", dist);

  long long score = 0;
  for (int *l = lft, *r = rht; l != lft + cnt; l++) {
    while (*r < *l && r != rht + cnt) r++;
    if (r == rht + cnt)
      break;
    int match = 0;
    for (;r + match != rht + cnt && *(r + match) == *l; match ++)
      ;
    r += match;
    score += (long long)match * *l;
  }
  printf("Day 1.2: %lld\n", score);

  free(lft);
  free(rht);
  return EXIT_SUCCESS;
}
