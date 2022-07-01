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
        makeRedCross();
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

    private void makeRedCross() {
        //so apparently this makes STUFF SCREW UP AND NOW IT'S DOING BAD STUFF LIKE SETTING A BUNCH OF THINGS TO WHITE RED!?
        move(this::makeRedCross, switch (cube.getPiece(Point.WR)) {
            case WB -> Move.F;
            case WO -> Move.F2;
            case WG -> Move.FP;
            case BR -> Move.DP;
            case GR -> Move.D;
            case YR -> Move.D2;
            case OG -> Move.L;
            case YG -> Move.L2;
            case RG -> Move.LP;
            case OB -> Move.RP;
            case YB -> Move.R2;
            case RB -> Move.R;
            case RW -> Move.D;

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
