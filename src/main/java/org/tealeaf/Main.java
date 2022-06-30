package org.tealeaf;

import org.tealeaf.cube.Move;
import org.tealeaf.cube.Point;

import java.util.Arrays;
import java.util.stream.Collectors;

public class Main {

    public static void main(String[] args) {
        System.out.println(Move.R2.getPermutations().stream().map(Arrays::deepToString).collect(Collectors.joining(" ")));
//        System.out.println(Move.R.apply(Point.WB));
    }
}