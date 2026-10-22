#include <stdio.h>

int *get_ref()
{
    int num = 42;
    return &num;
}

void nested(int *ref_u32)
{
    printf("Вложеный указатель: %d\n", *ref_u32);
}

void use_ref(int *ref_u32)
{
    printf("Число по указателю: %d\n", *ref_u32);
    nested(ref_u32);
}

int main(int argc, char *argv[])
{
    int num = 69;
    use_ref(&num);

    int *num_ref = get_ref();
    printf("Число из указателя: %d\n", *num_ref);
}
