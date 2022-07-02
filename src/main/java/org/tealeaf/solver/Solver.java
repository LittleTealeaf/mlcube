package org.tealeaf.solver;

import org.tealeaf.cube.Move;
import org.tealeaf.cube.Point;
import org.tealeaf.cube.RubiksCube;

import java.util.LinkedList;
import java.util.List;

public class Solver {

    private final RubiksCube cube;
    private final List<Move> steps = new LinkedList<>();

    public Solver(RubiksCube rubiksCube) {
        this.cube = rubiksCube;
    }

    public void solve() {
        orient();
        CrossWR();
    }

    private void orient() {
        step(this::orient, switch (cube.getPiece(Point.W)) {
            case R -> Move.xP;
            case O -> Move.x;
            case B -> Move.zP;
            case G -> Move.z;
            case Y -> Move.x2;
            default -> Move.NULL;
        });
        step(this::orient, switch (cube.getPiece(Point.R)) {
            case G -> Move.y;
            case B -> Move.yP;
            case O -> Move.y2;
            default -> Move.NULL;
        });
    }

    private void CrossWR() {
        step(this::CrossWR, switch (cube.getPiece(Point.WR)) {
            case WB, RW -> Move.D;
            case WO -> Move.D2;
            case WG -> Move.DP;
            case BW, OB -> Move.LP;
            case OW, OY -> Move.B;
            case GW, OG -> Move.R;
            case YR -> Move.F2;
            case YB -> Move.UP;
            case YO -> Move.U2;
            case YG -> Move.U;
            case RY, GR -> Move.F;
            case BY, RB -> Move.L;
            case GY, RG -> Move.RP;
            case GO -> Move.R2;
            case BR -> Move.FP;
            case BO -> Move.L2;

            default -> Move.NULL;
        });
    }

    public Move algorithm(Move... moves) {
        for (Move move : moves) {
            step(move);
        }
        return Move.NULL;
    }

    public void step(Move move) {
        if (move != Move.NULL) {
            cube.move(move);
            steps.add(move);
        }
    }

    public void step(Runnable runnable, Move move) {
        step(move);
        if (move != Move.NULL) {
            runnable.run();
        }
    }

    public List<Move> getSteps() {
        return steps;
    }
}
