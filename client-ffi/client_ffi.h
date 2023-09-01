#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

jstring Java_com_techecho_rfapp_FFIUtil_connectToNode(JNIEnv env,
                                                      JClass _class,
                                                      JString req,
                                                      JObject connect_status_callback,
                                                      JString path,
                                                      jint fd);

jstring Java_com_techecho_rfapp_FFIUtil_disconnect(JNIEnv env, JClass _class, jint port);

void Java_com_techecho_rfapp_FFIUtil_syncCallback(JNIEnv env, JClass _class, JObject callback);

jstring Java_com_techecho_rfapp_FFIUtil_test(JNIEnv env, JClass _class, JString str);

const char *add(uintptr_t left, uintptr_t right);

const char *connect_to_node(const char *req,
                            void (*on_connected_callback)(const char *node_ptr, const char *error_message),
                            void (*on_disconnected_callback)(const char *node_ptr, const char *error_message),
                            int fd);

const char *disconnect(uint16_t port);

const char *init_log(void (*log_callback)(const char *msg));

const char *reset_transport(uint16_t port, const char *ip, const char *protocol);

const char *test(const char *str);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
