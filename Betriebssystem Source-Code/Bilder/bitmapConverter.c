#include <stdio.h>  // Fuer fopen, fscanf, fclose, printf
#include <stdlib.h> // Fuer exit
#include <unistd.h>
#include <ctype.h>
#include <string.h>

#ifndef INPUT_FILE
#define INPUT_FILE "default_pic.c"
#endif
#include "bitmapConverter.h"

int main()
{
   printf("%s", INPUT_FILE);
   char buff[256];
   char output_fname[256];
   sprintf(buff, "%s", INPUT_FILE);

   char *fname = strtok(buff, ".");
   sprintf(output_fname, "%s.rs", fname);

   printf("Converting GIMP c source image '%s' to Rust array file '%s' ...\n", INPUT_FILE, output_fname);

   FILE *f = fopen(output_fname, "w+");
   if (f == NULL)
   {
      printf("error: could not create output file\n");
      exit(EXIT_FAILURE);
   }

   fprintf(f, "pub const WIDTH:u32  = %d;\n", gimp_image.width);
   fprintf(f, "pub const HEIGHT:u32 = %d;\n", gimp_image.height);
   fprintf(f, "pub const BPP:u32    = %d;\n", gimp_image.bytes_per_pixel);
   fprintf(f, "\n");
   fprintf(f, "pub const DATA: &[u8;%ld] = b\"", sizeof(gimp_image.pixel_data));

   for (int i = 0; i < sizeof(gimp_image.pixel_data); i++)
   {
      // printf("Pixeldata %d, {%d}\n", i, picture.pixel_data[i]);
      fprintf(f, "\\x%02x", gimp_image.pixel_data[i]);
   }

   fprintf(f, "\";\n");

   printf("Successfully converted");

   fclose(f);
   exit(EXIT_SUCCESS);
}
