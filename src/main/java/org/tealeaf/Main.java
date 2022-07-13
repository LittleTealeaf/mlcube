package org.tealeaf;

import org.tealeaf.cube.Move;
import org.tealeaf.cube.RubiksCube;
import org.tensorflow.ConcreteFunction;
import org.tensorflow.Signature;
import org.tensorflow.Tensor;
import org.tensorflow.ndarray.NdArray;
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
        RubiksCube rubiksCube = new RubiksCube();
        System.out.println(rubiksCube.print());
        rubiksCube.move(Move.yP);
        System.out.println(rubiksCube.print());

        try (ConcreteFunction dbl = ConcreteFunction.create(Main::dbl);
             TInt32 x = TInt32.tensorOf(NdArrays.vectorOf(1,2,3,4,5,6,7,8,9,10));
             Tensor dblx = dbl.call(x);
        ){
          System.out.println(x.getInt() + " doubled is " + ((TInt32) dblx).getInt());
//          System.out.println(x.asRawTensor().data().asInts().);
        }


    }

    private static Signature dbl(Ops tf) {
        Placeholder<TInt32> x = tf.placeholder(TInt32.class);
        Add<TInt32> dblX = tf.math.add(x, x);
        return Signature.builder().input("x",x).output("dbl",dblX).build();
    }
}
