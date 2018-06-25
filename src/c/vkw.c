#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <vulkan/vulkan.h>

void c_display_strings(size_t len, const char** strings) {
    for (size_t i = 0; i < len; ++i) {
        printf("%s\n", strings[i]);
    }
}

void c_display_string(const char* str) {
    printf("From C: %s\n", str);
}