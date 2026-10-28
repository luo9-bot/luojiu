#include "luo9_extension.h"
#include <stdio.h>

static const luo9_host_api *host_api;

int32_t luo9_extension_init(const luo9_host_api *host) {
    if (!host || host->abi_version != LUO9_EXTENSION_ABI_VERSION) return 1;
    host_api = host;
    if (host_api->log) host_api->log("example extension initialized");
    return 0;
}

void luo9_extension_shutdown(void) {
    if (host_api && host_api->log) host_api->log("example extension stopped");
    host_api = NULL;
}

int32_t luo9_extension_on_event(const char *json_event) {
    (void)json_event;
    return 0;
}
