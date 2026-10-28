#ifndef LUO9_EXTENSION_H
#define LUO9_EXTENSION_H

#include <stdint.h>

#ifdef _WIN32
#define LUO9_EXPORT __declspec(dllexport)
#else
#define LUO9_EXPORT __attribute__((visibility("default")))
#endif

#define LUO9_EXTENSION_ABI_VERSION 1u

typedef void (*luo9_log_fn)(const char *message);
typedef char *(*luo9_get_config_fn)(const char *extension_id);
typedef void (*luo9_free_string_fn)(char *value);

typedef struct luo9_host_api {
    uint32_t abi_version;
    luo9_log_fn log;
    luo9_get_config_fn get_config;
    luo9_free_string_fn free_string;
} luo9_host_api;

/* Return 0 to accept the host. */
LUO9_EXPORT int32_t luo9_extension_init(const luo9_host_api *host);

/* Called before the library is unloaded. */
LUO9_EXPORT void luo9_extension_shutdown(void);

/* Optional. Return 0 after accepting the JSON event. */
LUO9_EXPORT int32_t luo9_extension_on_event(const char *json_event);

#endif
