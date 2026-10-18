/* 7mlabs: a pinned, read-only Swiss asteroid file; no ambient filesystem lookup. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "astro_ephemeris.h"
static int enabled = 0; /* Only changed while the Rust provider mutex is held. */
void astro_enable_builtin_asteroids(int value) { enabled = value; }
#ifdef __APPLE__
struct reader { size_t offset; };
static int read_bytes(void *cookie, char *buffer, int count) {
  struct reader *r = cookie;
  size_t n = (size_t)count;
  if (n > sizeof(astro_ephemeris) - r->offset) n = sizeof(astro_ephemeris) - r->offset;
  memcpy(buffer, astro_ephemeris + r->offset, n); r->offset += n; return (int)n;
}
static fpos_t seek_bytes(void *cookie, fpos_t offset, int origin) {
  struct reader *r = cookie;
  fpos_t base = origin == SEEK_SET ? 0 : origin == SEEK_CUR ? (fpos_t)r->offset : origin == SEEK_END ? (fpos_t)sizeof(astro_ephemeris) : -1;
  if (base < 0 || offset < -base || offset > (fpos_t)sizeof(astro_ephemeris) - base) return -1;
  r->offset = (size_t)(base + offset); return (fpos_t)r->offset;
}
static int close_bytes(void *cookie) { free(cookie); return 0; }
#endif
FILE *astro_builtin_ephemeris_open(const char *name) {
  if (!enabled || strcmp(name, "seas_18.se1") != 0) return NULL;
#ifdef __APPLE__
  struct reader *r = calloc(1, sizeof(*r));
  if (!r) return NULL;
  FILE *f = funopen(r, read_bytes, NULL, seek_bytes, close_bytes);
  if (!f) free(r);
  return f;
#elif defined(__linux__)
  return fmemopen((void *)astro_ephemeris, sizeof(astro_ephemeris), "rb");
#else
  /* Portable fallback contains embedded bytes only; never reads user files. */
  FILE *f = tmpfile();
  if (!f) return NULL;
  if (fwrite(astro_ephemeris, 1, sizeof(astro_ephemeris), f) != sizeof(astro_ephemeris)) { fclose(f); return NULL; }
  rewind(f); return f;
#endif
}
