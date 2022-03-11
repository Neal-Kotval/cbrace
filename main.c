#include "common.h"
#include "chunk.h"
#include "debug.h"

int main(int argc, const char* argv[]) {
    Chunk chunk;
    initChunk(&chunk);

    disassembleChunk(&chunk, "test chunk");

    writeChunk(&chunk, OP_RETURN);
    freeChunk(&chunk);

    return 0;
}