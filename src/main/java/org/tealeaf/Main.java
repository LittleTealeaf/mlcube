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
        RubiksCube rubiksCube = new RubiksCube();
        rubiksCube.scramble(100);
        Solver solver = new Solver(rubiksCube);
        System.out.println(rubiksCube);
        solver.solve();
        System.out.println(solver.getSteps());
        System.out.println(rubiksCube);
    }


}