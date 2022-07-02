package org.tealeaf.solver;

import org.tealeaf.cube.Move;
import org.tealeaf.cube.RubiksCube;

import java.util.LinkedList;
import java.util.List;
import java.util.stream.Stream;

public class Solver {

    private final RubiksCube cube;
    private final List<Move> steps = new LinkedList<>();

    public Solver(RubiksCube rubiksCube) {
        this.cube = rubiksCube;
    }

    public void solve() {

    }



//    private void move(Runnable runnable, Move move) {
//        move(move);
//        if (move != Move.NULL) {
//            runnable.run();
//        }
//    }
//
//
//
//    public List<Move> getSteps() {
//        return steps;
//    }
//
//    private void move(Move... moves) {
//        Stream.of(moves).forEach(move -> {
//            if (move != Move.NULL) {
//                cube.move(move);
//                steps.add(move);
//            }
//        });
////        cube.move(moves);
////        steps.addAll(List.of(moves));
//    }
}
