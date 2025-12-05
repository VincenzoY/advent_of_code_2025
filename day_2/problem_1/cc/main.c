#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <math.h>

size_t numDigits(long int n);
bool isInvalidId(long int n);

int main() {
    FILE * fptr;

    fptr = fopen("../../input.txt", "r");

    if(fptr != NULL) {
        long int sum = 0;

        while (!feof(fptr)) {
            long int start = 0;
            long int end = 0;

            for(char next_char = fgetc(fptr); next_char != '-'; next_char = fgetc(fptr)) {
                start = start * 10 + (next_char - '0');
            }

            for(char next_char = fgetc(fptr); next_char != ',' && next_char != EOF; next_char = fgetc(fptr)) {
                end = end * 10 + (next_char - '0');
            }

            for(long int cur = start; cur <= end; cur++) {
                if (isInvalidId(cur)) {
                    sum += cur;
                }
            }
        }

        printf("%ld\n", sum);

    } else {
        printf("Not able to open the file.");
    }

    fclose(fptr);
}

bool isInvalidId(long int n) {
    size_t length = numDigits(n);
    long int upper_value = n / pow(10, length / 2);
    return upper_value == n - (upper_value * pow(10, length / 2));
}

size_t numDigits(long int n) {
    if (n < 10) return 1;
    return 1 + numDigits(n / 10);
}