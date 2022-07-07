package org.tealeaf;

import org.tealeaf.cube.Move;
import org.tealeaf.cube.Point;
import org.tealeaf.cube.RubiksCube;
import org.tealeaf.solver.Solver;

import java.util.Arrays;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public class Main {

    public static void main(String[] args) {
        RubiksCube rubiksCube = new RubiksCube();
        System.out.println(rubiksCube.print2d());
        System.out.println("\n");
        rubiksCube.scramble(100);
        System.out.println(rubiksCube.print2d());
        Solver solver = new Solver(rubiksCube);
        solver.solve();
        System.out.println("\n");
        System.out.println(rubiksCube.print2d());


    }


}