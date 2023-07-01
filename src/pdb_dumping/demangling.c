#pragma comment(lib, "DbgHelp.lib")

#include "Windows.h"
#include "DbgHelp.h"

const char* demangle(const char* symbol)
{
    char* buffer = malloc(1024 * sizeof(char));
    memset(buffer, 0, 1024);
    UnDecorateSymbolName(symbol, buffer, 1024, 0);
    return buffer;
}

void free_demangled_name(char* name)
{
    free(name);
}
