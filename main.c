#include <main.h>

static const char help[] = 
  "weird sht compiler "VERSION" - Copyright (C) 2023 sofa3376\n\n"
  "Usage:\n"
  "    ./wsc [options...]"
  "General options:\n"
  "    -help or -h     show this\n"
  "    -v or -version  show version of compiler\n"
;

int main(int argc, char* argv[]) {
  if (argc > 1) {
    for (int i = 1; i < argc; i++) {
      if (strcmp(argv[i], "-help") || strcmp(argv[i], "-h"))
        printf(help);
      else if (strcmp(argv[i], "-v") || strcmp(argv[i], "-version"))
        printf(VERSION);
    }
  } else {
    printf("no options to run");
  }
  return 0;
}
