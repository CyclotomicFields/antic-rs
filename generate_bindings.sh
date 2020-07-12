#!/bin/bash

mkdir include
cp -rv /usr/local/include/{flint,antic} include/
find include -type f -exec sed -i 's/^FMPQ_INLINE//g' {} \;
find include -type f -exec sed -i 's/^NF_ELEM_INLINE//g' {} \;
/usr/bin/bindgen include/antic/nf_elem.h -- -Iinclude > src/bindings.rs
perl -ne 'print if ($. < 20000) or ($_ !~ /FP_(NAN|INFINITE|ZERO|SUBNORMAL|NORMAL)/)' src/bindings.rs | sponge src/bindings.rs
