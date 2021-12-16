(module
  (import "env" "memory" (memory 4))
  (func (export "upd")
    (local $i i32) ;; local variables are zero initialized

    (loop $pixels
      local.get $i ;; pixel index to write to

      (i32.rem_u (local.get $i) (i32.const 320)) ;; x
      (i32.div_u (i32.load (i32.const 64)) (i32.const 10)) ;; time / 10
      i32.add

      (i32.div_u (local.get $i) (i32.const 320)) ;; y

      i32.xor ;; (x + time / 10) ^ y
      (i32.shr_u (i32.const 3)) ;; .. >> 3
      (i32.and (i32.const 127)) ;; .. & 127

      i32.store8 offset=120 ;; store at pixel index + 120

      (i32.add (local.get $i) (i32.const 1)) ;; i + 1
      local.tee $i ;; write it back but keep it on the stack
      (br_if $pixels (i32.lt_s (i32.const 76800))) ;; branch to start of loop if i < 320*240
    )
  )
)
