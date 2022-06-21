(module
  (import "env" "atan2" (func $atan2 (param f32 f32) (result f32)))
  (import "env" "time" (func $time (result f32)))
  (import "env" "memory" (memory 4))
  (func (export "upd")
    (local $y i32)
    (local $i i32) 
    (local $x i32)

    (loop $pixels
      i32.const 1
      local.get $i

      local.get $i

      i32.const 36928
      f32.convert_i32_s
      local.get $i
      i32.const 320
      i32.rem_s
      i32.const 160
      i32.sub
      local.tee $x
      local.get $x
      i32.mul
      local.get $i
      i32.const 320
      i32.div_s
      i32.const 120
      i32.sub
      local.tee $y
      local.get $y
      i32.mul
      i32.add
      f32.convert_i32_s
      f32.sqrt
      f32.div
      i32.const 163
      f32.convert_i32_s
      call $time
      f32.mul
      f32.add
      i32.trunc_sat_f32_s

      i32.const 163
      f32.convert_i32_s
      local.get $x
      f32.convert_i32_s
      local.get $y
      f32.convert_i32_s
      call $atan2
      f32.mul
      i32.const 64
      f32.convert_i32_s
      call $time
      f32.mul
      f32.add
      i32.trunc_f32_s

      i32.xor
      i32.const 4
      i32.shr_s
      i32.const 15
      i32.and
      i32.store8 offset=120

      i32.add
      local.tee $i
      i32.const 76800
      i32.rem_s
      br_if $pixels
  )
 )
)
