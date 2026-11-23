/* SPDX-License-Identifier: MIT
 * Characterize each toolboxrc passed on the command line, including reloads.
 * Print bytes in hex to keep non-UTF-8 titles and commands comparable. */
#include "tbparse.h"
#include <stdio.h>

static void dump_string(const char *s)
{
    if (!s) {
        putchar('-');
        return;
    }
    while (*s) printf("%02x", (unsigned char)*s++);
}

int main(int argc, char **argv)
{
    struct tb_entry *root = NULL;
    for (int i = 1; i < argc; i++) {
        int status = tb_parse_config(argv[i], &root);
        const char *error = tb_parser_error_string();
        printf("%d %s\n", status, error ? error : "");
        for (const struct tb_entry *entry = root; entry; entry = entry->next) {
            printf("%d %d %02x ", entry->type, entry->level, (unsigned char)entry->mnemonic);
            dump_string(entry->title);
            putchar(' ');
            dump_string(entry->command);
            putchar('\n');
        }
    }
    return 0;
}
