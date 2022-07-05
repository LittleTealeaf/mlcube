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
        orient();
        whiteCross();
    }

    public void orient() {
        step(switch(cube.getPiece(Point.W)) {
            case R -> alg(Move.xP);
            case O -> alg(Move.x);
            case B -> alg(Move.zP);
            case G -> alg(Move.z);
            case Y -> alg(Move.x2);
            default -> alg();
        });
        step(switch (cube.getPiece(Point.R)) {
            case G ->  alg(Move.y);
            case B ->  alg(Move.yP);
            case O ->  alg(Move.y2);
            default -> alg();
        });
    }

    public void whiteCross() {
        //WR
        step(switch(cube.getPiece(Point.WR)) {
            case WR -> alg();
            case WO -> alg(Move.D2);
            case WG -> alg(Move.DP);
            case WB -> alg(Move.D);
            case YO -> alg(Move.B2,Move.D2);
            case YB -> alg(Move.L2,Move.D);
            case YG -> alg(Move.R2,Move.DP);
            case YR -> alg(Move.F2);
            case RY -> alg(Move.F,Move.RP,Move.DP);
            case RB -> alg(Move.L,Move.D);
            case RG -> alg(Move.RP,Move.DP);
            case RW -> alg(Move.F,Move.L,Move.D);
            case BO -> alg(Move.B,Move.D2);
            case BR -> alg(Move.FP);
            case BW -> alg(Move.LP,Move.FP);
            case BY -> alg(Move.L,Move.FP);
            case OW -> alg(Move.B,Move.R,Move.DP);
            case OB -> alg(Move.LP,Move.D);
            case OG -> alg(Move.R,Move.DP);
            case OY -> alg(Move.B,Move.LP,Move.D);
            case GO -> alg(Move.R2,Move.F);
            case GW -> alg(Move.R,Move.F);
            case GY -> alg(Move.RP,Move.F);
            case GR -> alg(Move.F);
            default -> alg();
        });
        step(switch(cube.getPiece(Point.WB)) {
            case WR -> alg();
            case WO -> alg(Move.BP,Move.DP,Move.B,Move.D);
            case WG -> alg(Move.D2,Move.L,Move.D2,Move.LP);
            case WB -> alg();
            case YO -> alg(Move.UP,Move.L2);
            case YB -> alg(Move.L2);
            case YG -> alg(Move.U2,Move.L2);
            case YR -> alg(Move.U,Move.L2);
            case RY -> alg(Move.FP,Move.L,Move.F);
            case RB -> alg(Move.L);
            case RG -> alg(Move.F2,Move.L,Move.F2);
            case RW -> alg();
            case BO -> alg(Move.DP,Move.B,Move.D);
            case BR -> alg(Move.D,Move.FP,Move.DP);
            case BW -> alg(Move.LP,Move.D,Move.FP,Move.DP);
            case BY -> alg(Move.U,Move.B,Move.LP);
            case OW -> alg(Move.BP,Move.LP);
            case OB -> alg(Move.LP);
            case OG -> alg(Move.B2,Move.LP);
            case OY -> alg(Move.B,Move.LP);
            case GO -> alg(Move.DP,Move.BP,Move.D);
            case GW -> alg(Move.D,Move.BP,Move.DP,Move.LP);
            case GY -> alg(Move.UP,Move.B,Move.LP);
            case GR -> alg(Move.D,Move.F,Move.DP);
            default -> alg();
        });

        Move[] moves = switch(cube.getPiece(Point.WR)) {
            case WR -> alg();
            case WO -> alg();
            case WG -> alg();
            case WB -> alg();
            case YO -> alg();
            case YB -> alg();
            case YG -> alg();
            case YR -> alg();
            case RY -> alg();
            case RB -> alg();
            case RG -> alg();
            case RW -> alg();
            case BO -> alg();
            case BR -> alg();
            case BW -> alg();
            case BY -> alg();
            case OW -> alg();
            case OB -> alg();
            case OG -> alg();
            case OY -> alg();
            case GO -> alg();
            case GW -> alg();
            case GY -> alg();
            case GR -> alg();
            default -> alg();
        };
    }

    public void step(Move[] moves) {
        Stream.of(moves).forEach(this::step);
    }

    public Move[] alg(Move... moves) {
        return moves;
    }

    public void step(Move move) {
        if (move != Move.NULL) {
            cube.move(move);
            steps.add(move);
        }
    }

    public List<Move> getSteps() {
        return steps;
    }

    private interface Step {

        Move getStep();
    }
}
