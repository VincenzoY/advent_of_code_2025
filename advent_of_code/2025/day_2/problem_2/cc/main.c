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
                    printf("%ld\n", cur);

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

    for(int digits = 1; digits <= length / 2; digits++){
        if (length % digits != 0) continue;

        long tmp = n;
        long matchValue = 0;

        for (int x = 0; x < digits; x += 1) {
            matchValue = matchValue + pow(10, x) * (tmp % 10);
            tmp = tmp / 10;
        }


        for (int y = 0; y < (length / digits) - 1; y++) {
            long nextVal = 0;
            for (int x = 0; x < digits; x += 1) {
                nextVal = nextVal + pow(10, x) * (tmp % 10);
                tmp = tmp / 10;
            }

            if (nextVal != matchValue) break;
            if (tmp == 0) return true;
        }
    }

    return false;
}

size_t numDigits(long int n) {
    if (n < 10) return 1;
    return 1 + numDigits(n / 10);
}