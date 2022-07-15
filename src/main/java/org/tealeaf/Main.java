package org.tealeaf;

import org.tealeaf.cube.RubiksCube;
import org.tealeaf.environment.Move;
import org.tensorflow.ConcreteFunction;
import org.tensorflow.Signature;
import org.tensorflow.Tensor;
import org.tensorflow.ndarray.NdArrays;
import org.tensorflow.op.Ops;
import org.tensorflow.op.core.Placeholder;
import org.tensorflow.op.math.Add;
import org.tensorflow.types.TInt32;

public class Main {

//    public static void main(String[] args) {

//
//
//
//    }

    public static void main(String[] args) {
        System.out.println(Move.R.getPermutations());
        System.out.println(Move.RP.getPermutations());
        System.out.println(Move.R2.getPermutations());

//        try (ConcreteFunction dbl = ConcreteFunction.create(Main::dbl);
//             TInt32 x = TInt32.tensorOf(NdArrays.vectorOf(1,2,3,4,5,6,7,8,9,10));
//             Tensor dblx = dbl.call(x);
//        ){
//          System.out.println(x.getInt() + " doubled is " + ((TInt32) dblx).getInt());
//        }


    }

    private static Signature dbl(Ops tf) {
        Placeholder<TInt32> x = tf.placeholder(TInt32.class);
        Add<TInt32> dblX = tf.math.add(x, x);
        return Signature.builder().input("x",x).output("dbl",dblX).build();
    }
}
