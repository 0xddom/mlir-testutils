module {
     func.func @add(%x: index, %y: index) -> index {
         %r = arith.addi %x, %y: index
         func.return %r : index
     }
 }
