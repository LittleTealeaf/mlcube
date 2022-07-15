package org.tealeaf;

import org.tealeaf.environment.Cube;
import org.tealeaf.environment.Move;
import org.tensorflow.Signature;
import org.tensorflow.op.Ops;
import org.tensorflow.op.core.Placeholder;
import org.tensorflow.op.math.Add;
import org.tensorflow.types.TInt32;

import java.text.RuleBasedCollator;
import java.util.Random;

public class Main {

//    public static void main(String[] args) {

//
//
//
//    }

    public static void main(String[] args) {
        Cube cube = new Cube();
        long start = System.currentTimeMillis();
        for(int i = 0; i < 1_000_000; i++) {
            cube.move(Move.random());
        }
        long end = System.currentTimeMillis();
        System.out.println(cube);
        System.out.println(end - start);


    }

    private static Signature dbl(Ops tf) {
        Placeholder<TInt32> x = tf.placeholder(TInt32.class);
        Add<TInt32> dblX = tf.math.add(x, x);
        return Signature.builder().input("x",x).output("dbl",dblX).build();
    }
}
