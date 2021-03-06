#define IMPORT(MODULE, NAME) __attribute__((import_module(MODULE), import_name(NAME)))

IMPORT("env", "atan2") extern float atan2(float, float);
IMPORT("env", "time") extern float time();

float sqrt(float v) {
    return __builtin_sqrt(v);
}

#define FRAMEBUFFER ((unsigned char*)120)

void upd() {
    int i = 0;

    for( ;; ) {
        float t = time() * 63.0f;
        float x = (float)(i % 320 - 160);
        float y = (float)(i / 320 - 120);
        float d = 40000.0f / sqrt(x * x + y * y);
        float u = atan2(x, y) * 512.0f / 3.141f;
        unsigned char c = (unsigned char)((int)(d + t * 2.0f) ^ (int)(u + t)) >> 4;
        FRAMEBUFFER[i] = c;

        i += 1;
        if(i >= 320*240) break;
    }
}