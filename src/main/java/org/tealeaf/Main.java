package org.tealeaf;

import org.tealeaf.cube.Move;
import org.tealeaf.cube.Piece;
import org.tealeaf.cube.Point;
import org.tealeaf.cube.RubiksCube;
import org.tealeaf.solver.Solver;

import java.io.Writer;
import java.util.Arrays;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class Main {

    public static void main(String[] args) {
        RubiksCube cube = new RubiksCube();
        cube.scramble(100);
        System.out.println(cube);
        Solver s = new Solver(cube);
        s.solve();
        System.out.println(s.getSteps());
        System.out.println(cube);
//        for (Move move : Move.values()) {
//            RubiksCube rubiksCube = new RubiksCube();
//            rubiksCube.move(move);
//            Solver solver = new Solver(rubiksCube);
//            solver.solve();
//            if(Stream.of(Point.W, Point.R, Point.Y, Point.G, Point.B, Point.O).anyMatch(i -> rubiksCube.getPiece(i) != i)) {
//                System.out.println(move);
//                System.out.println(rubiksCube);
//            }
//        }
    }


}