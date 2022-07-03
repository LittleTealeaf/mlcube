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
        whiteCross();
    }

    public void orient() {
        step(() -> switch (cube.getPiece(Point.W)) {
            case R -> Move.xP;
            case O -> Move.x;
            case B -> Move.zP;
            case G -> Move.z;
            case Y -> Move.x2;
            default -> Move.NULL;
        });
        step(() -> switch (cube.getPiece(Point.R)) {
            case G -> Move.y;
            case B -> Move.yP;
            case O -> Move.y2;
            default -> Move.NULL;
        });
    }

    public void whiteCross() {
        //WR
        step(() -> switch (cube.getPiece(Point.WR)) {
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
        step(() -> switch (cube.getPiece(Point.WB)) {
            case WO -> switch (cube.getPiece(Point.WR)) {
                case WR -> Move.B;
                case WB -> Move.D;
                default -> Move.NULL;
            };
            case WG -> switch (cube.getPiece(Point.WR)) {
                case WR -> Move.R;
                case WO -> Move.D2;
                default -> Move.NULL;
            };
            case BW, BY, RB -> Move.L;
            case OB -> Move.LP;
            case OW -> Move.BP;
            case OY -> Move.B;
            case GW -> Move.RP;
            case OG -> switch (cube.getPiece(Point.WR)) {
                case WR -> Move.D2;
                case WO -> Move.R;
                default -> Move.NULL;
            };
            case YR -> Move.U;
            case YB -> Move.L2;
            case YO -> Move.UP;
            case YG -> Move.U2;
            case RY, GR, BR -> Move.F;
            case GY -> Move.R;
            case RG -> switch (cube.getPiece(Point.WR)) {
                case WR -> Move.D2;
                case WO -> Move.RP;
                default -> Move.NULL;
            };
            case GO -> switch (cube.getPiece(Point.WR)) {
                case WR -> Move.DP;
                case WB -> Move.BP;
                default -> Move.NULL;
            };
            case WR -> Move.DP;
            case BO -> Move.B2;
            case WB -> switch (cube.getPiece(Point.WR)) {
                case GR -> Move.FP;
                case BR -> Move.F;
                default -> Move.NULL;
            };
            default -> Move.NULL;
        });
        step(() -> switch(cube.getPiece(Point.WO)) {
            case WR -> Move.D2;
            case WO -> switch(cube.getPiece(Point.WB)) {
                case RB -> Move.L;
                default -> Move.NULL;
            };
            case WG -> switch(cube.getPiece(Point.WR)) {
                case WR -> Move.R;
                case WB -> Move.D;
                default -> Move.NULL;
            };
            case WB -> Move.DP;
            case YO -> Move.B2;
            case YB -> Move.U;
            case YG -> Move.UP;
            case YR -> Move.U2;
            case RY -> Move.UP;
            case RB -> switch(cube.getPiece(Point.WR)) {
                case WR -> Move.D;
                case WG -> Move.L;
                default -> Move.NULL;
            };
            case RG -> switch(cube.getPiece(Point.WR)) {
                case WR -> Move.DP;
                case WB -> Move.RP;
                default -> Move.NULL;
            };
            case RW -> Move.NULL;
            case BO -> Move.B;
            case BR -> switch(cube.getPiece(Point.WR)) {
                case WR -> Move.D2;
                case WO -> Move.FP;
                default -> Move.NULL;
            };
            case BW -> Move.NULL;
            case BY -> Move.LP;
            case OW -> Move.B;
            case OB -> Move.B2;
            case OG -> switch (cube.getPiece(Point.WR)) {
                case WR -> Move.DP;
                case WB -> Move.R;
                default -> Move.NULL;
            };
            case OY -> Move.BP;
            case GO -> Move.BP;
            case GW -> Move.RP;
            case GY -> Move.R;
            case GR -> Move.R2;
            default -> Move.NULL;
        });
        Move move = switch(cube.getPiece(Point.WR)) {
            case WR -> Move.NULL;
            case WO -> Move.NULL;
            case WG -> Move.NULL;
            case WB -> Move.NULL;
            case YO -> Move.NULL;
            case YB -> Move.NULL;
            case YG -> Move.NULL;
            case YR -> Move.NULL;
            case RY -> Move.NULL;
            case RB -> Move.NULL;
            case RG -> Move.NULL;
            case RW -> Move.NULL;
            case BO -> Move.NULL;
            case BR -> Move.NULL;
            case BW -> Move.NULL;
            case BY -> Move.NULL;
            case OW -> Move.NULL;
            case OB -> Move.NULL;
            case OG -> Move.NULL;
            case OY -> Move.NULL;
            case GO -> Move.NULL;
            case GW -> Move.NULL;
            case GY -> Move.NULL;
            case GR -> Move.NULL;
            default -> Move.NULL;
        };
    }

    public void step(Step step) {
        Move move;
        while ((move = step.getStep()) != Move.NULL) {
            cube.move(move);
            steps.add(move);
        }
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
