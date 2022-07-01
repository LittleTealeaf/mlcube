package org.tealeaf.solver;

import org.tealeaf.cube.Move;
import org.tealeaf.cube.Point;
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
        orientCorrectly();
        makeWhiteCross();
    }

    private void orientCorrectly() {
        move(switch (cube.getPiece(Point.W)) {
            case O -> Move.XP;
            case R -> Move.X;
            case Y -> Move.Y2;
            case G -> Move.YP;
            case B -> Move.Y;
            default -> Move.NONE;
        });
        move(switch (cube.getPiece(Point.R)) {
            case B -> Move.Z;
            case O -> Move.Z2;
            case G -> Move.ZP;
            default -> Move.NONE;
        });
    }

    private void move(Runnable runnable, Move move) {
        move(move);
        if (move != Move.NONE) {
            runnable.run();
        }
    }

    private void makeWhiteCross() {
        move(this::move, switch (cube.getPiece(Point.WR)) {
            default -> Move.NONE;
        });
    }

    public List<Move> getSteps() {
        return steps;
    }

    private void move(Move... moves) {
        Stream.of(moves).forEach(move -> {
            if (move != Move.NONE) {
                cube.move(move);
                steps.add(move);
            }
        });
//        cube.move(moves);
//        steps.addAll(List.of(moves));
    }
}
