package org.tealeaf;

import org.tealeaf.cube.Move;
import org.tealeaf.cube.RubiksCube;
import org.tealeaf.solver.Solver;

import java.util.Arrays;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public class Main {

    public static void main(String[] args) {
        long time = System.currentTimeMillis();
        IntStream.range(0,1000000).parallel().forEach(i -> {
            RubiksCube rubiksCube = new RubiksCube();
            rubiksCube.scramble(300);
        });
        long elapsed = System.currentTimeMillis() - time;
        System.out.println(elapsed);
    }


}