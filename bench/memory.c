#define ITERATIONS 1000

int main(void) {
    int array[1000];

    for (int i = 0; i < 1000; i++) {
        array[i] = i;    
    }

    for (int i = 0; i < ITERATIONS; i++) {
        for (int i = 0; i < 1000; i++) {
            array[i] += 1;
        }
    }
    __asm__("mov     $0,%rbx");
    __asm__("mov     $1,%rax");
    __asm__("int     $0x80");
}
