

static int a;
static int b;
static int c;


int main(void) {
    a = 5;
    b = 10;
    c = 15;

    if (a != 5) {
        __asm__("int3");
    }
    if (b != 10) {
        __asm__("int3");
    }
    if (c != 15) {
        __asm__("int3");
    }

    a += 1;
    b += 2;
    c += 3;

    if (a != 6) {
        __asm__("int3");
    }
    if (b != 12) {
        __asm__("int3");
    }
    if (c != 18) {
        __asm__("int3");
    }
    __asm__("mov     $0,%rbx");
    __asm__("mov     $1,%rax");
    __asm__("int     $0x80");
    return a;
}