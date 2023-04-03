;; MicroW8 APIs, in WAT (Wasm Text) format
(import "env" "memory" (memory 4))

(import "env" "sin" (func $sin (param f32) (result f32)))
(import "env" "cos" (func $cos (param f32) (result f32)))
(import "env" "tan" (func $tan (param f32) (result f32)))
(import "env" "asin" (func $asin (param f32) (result f32)))
(import "env" "acos" (func $acos (param f32) (result f32)))
(import "env" "atan" (func $atan (param f32) (result f32)))
(import "env" "atan2" (func $atan2 (param f32) (param f32) (result f32)))
(import "env" "pow" (func $pow (param f32) (param f32) (result f32)))
(import "env" "log" (func $log (param f32) (result f32)))
(import "env" "fmod" (func $fmod (param f32) (param f32) (result f32)))
(import "env" "random" (func $random (result i32)))
(import "env" "randomf" (func $randomf (result f32)))
(import "env" "randomSeed" (func $randomSeed (param i32)))
(import "env" "cls" (func $cls (param i32)))
(import "env" "setPixel" (func $setPixel (param i32) (param i32) (param i32)))
(import "env" "getPixel" (func $getPixel (param i32) (param i32) (result i32)))
(import "env" "hline" (func $hline (param i32) (param i32) (param i32) (param i32)))
(import "env" "rectangle" (func $rectangle (param f32) (param f32) (param f32) (param f32) (param i32)))
(import "env" "circle" (func $circle (param f32) (param f32) (param f32) (param i32)))
(import "env" "line" (func $line (param f32) (param f32) (param f32) (param f32) (param i32)))
(import "env" "time" (func $time (result f32)))
(import "env" "isButtonPressed" (func $isButtonPressed (param i32) (result i32)))
(import "env" "isButtonTriggered" (func $isButtonTriggered (param i32) (result i32)))
(import "env" "printChar" (func $printChar (param i32)))
(import "env" "printString" (func $printString (param i32)))
(import "env" "printInt" (func $printInt (param i32)))
(import "env" "setTextColor" (func $setTextColor (param i32)))
(import "env" "setBackgroundColor" (func $setBackgroundColor (param i32)))
(import "env" "setCursorPosition" (func $setCursorPosition (param i32) (param i32)))
(import "env" "rectangleOutline" (func $rectangleOutline (param f32) (param f32) (param f32) (param f32) (param i32)))
(import "env" "circleOutline" (func $circleOutline (param f32) (param f32) (param f32) (param i32)))
(import "env" "exp" (func $exp (param f32) (result f32)))
(import "env" "playNote" (func $playNote (param i32) (param i32)))
(import "env" "sndGes" (func $sndGes (param i32) (result f32)))
(import "env" "blitSprite" (func $blitSprite (param i32) (param i32) (param i32) (param i32) (param i32)))
(import "env" "grabSprite" (func $grabSprite (param i32) (param i32) (param i32) (param i32) (param i32)))

;; to use defines, include this file with a preprocessor
;; like gpp (https://logological.org/gpp).
#define TIME_MS 0x40;
#define GAMEPAD 0x44;
#define FRAMEBUFFER 0x78;
#define PALETTE 0x13000;
#define FONT 0x13400;
#define USER_MEM 0x14000;
#define BUTTON_UP 0x0;
#define BUTTON_DOWN 0x1;
#define BUTTON_LEFT 0x2;
#define BUTTON_RIGHT 0x3;
#define BUTTON_A 0x4;
#define BUTTON_B 0x5;
#define BUTTON_X 0x6;
#define BUTTON_Y 0x7;
