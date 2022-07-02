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

        Stream.of(Move.values()).forEach(move -> {
            System.out.println(move + " " + move.getPermutations().stream().map(Arrays::toString).collect(Collectors.joining(", ")));
        });

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