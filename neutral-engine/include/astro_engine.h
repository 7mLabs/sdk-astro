#ifndef ASTRO_ENGINE_H
#define ASTRO_ENGINE_H
#include <stdint.h>
#ifdef __cplusplus
extern "C" {
#endif
uint32_t astro_abi_version(void);
/* UTF-8 JSON input, NUL-terminated. Output is owned; free exactly once below. */
char *astro_calculate_json(const char *input);
/* Separate JSON->JSON context preparation. Input: {"payload": envelope, "options": {...}}. */
char *astro_compress_json(const char *input);
/* Decode an astro-context/1 envelope to its retained engine envelope. */
char *astro_expand_context_json(const char *input);
/* Optional services; same JSON envelope and ownership rules. */
char *astro_resolve_birth_time_json(const char *input);
char *astro_calculate_from_local_json(const char *input);
char *astro_render_chart_svg_json(const char *input);
char *astro_capabilities_json(const char *input);
void astro_free_string(char *output);
#ifdef __cplusplus
}
#endif
#endif
