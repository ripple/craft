(module
  ;; A function that loops a fixed number of times.
  ;; The result is to ensure the loop body is not optimized away.
  (func (export "finish") (result i32)
    (local $count i32)
    (local.set $count (i32.const 100000000))

    (loop $my_loop
      ;; Do nothing. This is the empty loop body.

      ;; Decrement the counter
      (local.get $count)
      (i32.const 1)
      (i32.sub)
      (local.set $count)

      ;; Repeat the loop if the counter is not zero.
      (local.get $count)
      (i32.const 0)
      (i32.ne)
      (br_if $my_loop)
    )

    (i32.const 0) ;; Return a dummy value
  )
)