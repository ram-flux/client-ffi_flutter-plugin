#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

const char *add(uintptr_t left, uintptr_t right);

const char *connect_to_node(const char *req,
                            void (*on_connected_callback)(const char *node_ptr, const char *error_message),
                            void (*on_disconnected_callback)(const char *node_ptr, const char *error_message),
                            const char *path);

const char *disconnect(uint16_t port);

const char *log(void (*log_callback)(const char *msg));

const char *test(const char *str);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
