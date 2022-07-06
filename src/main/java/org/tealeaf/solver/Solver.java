package org.tealeaf.solver;

import org.tealeaf.cube.Move;
import org.tealeaf.cube.Point;
import org.tealeaf.cube.RubiksCube;

import java.security.spec.RSAKeyGenParameterSpec;
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
        F2L();
    }

    public void orient() {
        step(switch (cube.getPiece(Point.W)) {
            case R -> alg(Move.xP);
            case O -> alg(Move.x);
            case B -> alg(Move.zP);
            case G -> alg(Move.z);
            case Y -> alg(Move.x2);
            default -> alg();
        });
        step(switch (cube.getPiece(Point.R)) {
            case G -> alg(Move.y);
            case B -> alg(Move.yP);
            case O -> alg(Move.y2);
            default -> alg();
        });
    }

    public void whiteCross() {
        //WR
        step(switch (cube.getPiece(Point.WR)) {
            case WR -> alg();
            case WO -> alg(Move.D2);
            case WG -> alg(Move.DP);
            case WB -> alg(Move.D);
            case YO -> alg(Move.B2, Move.D2);
            case YB -> alg(Move.L2, Move.D);
            case YG -> alg(Move.R2, Move.DP);
            case YR -> alg(Move.F2);
            case RY -> alg(Move.F, Move.RP, Move.DP);
            case RB -> alg(Move.L, Move.D);
            case RG -> alg(Move.RP, Move.DP);
            case RW -> alg(Move.F, Move.L, Move.D);
            case BO -> alg(Move.B, Move.D2);
            case BR -> alg(Move.FP);
            case BW -> alg(Move.LP, Move.FP);
            case BY -> alg(Move.L, Move.FP);
            case OW -> alg(Move.B, Move.R, Move.DP);
            case OB -> alg(Move.LP, Move.D);
            case OG -> alg(Move.R, Move.DP);
            case OY -> alg(Move.B, Move.LP, Move.D);
            case GO -> alg(Move.R2, Move.F);
            case GW -> alg(Move.R, Move.F);
            case GY -> alg(Move.RP, Move.F);
            case GR -> alg(Move.F);
            default -> alg();
        });
        step(switch (cube.getPiece(Point.WB)) {
            case WR -> alg();
            case WO -> alg(Move.BP, Move.DP, Move.B, Move.D);
            case WG -> alg(Move.D2, Move.L, Move.D2, Move.LP);
            case WB -> alg();
            case YO -> alg(Move.UP, Move.L2);
            case YB -> alg(Move.L2);
            case YG -> alg(Move.U2, Move.L2);
            case YR -> alg(Move.U, Move.L2);
            case RY -> alg(Move.FP, Move.L, Move.F);
            case RB -> alg(Move.L);
            case RG -> alg(Move.F2, Move.L, Move.F2);
            case RW -> alg();
            case BO -> alg(Move.DP, Move.B, Move.D);
            case BR -> alg(Move.D, Move.FP, Move.DP);
            case BW -> alg(Move.LP, Move.D, Move.FP, Move.DP);
            case BY -> alg(Move.U, Move.B, Move.LP);
            case OW -> alg(Move.BP, Move.LP);
            case OB -> alg(Move.LP);
            case OG -> alg(Move.B2, Move.LP);
            case OY -> alg(Move.B, Move.LP);
            case GO -> alg(Move.DP, Move.BP, Move.D);
            case GW -> alg(Move.D, Move.BP, Move.DP, Move.LP);
            case GY -> alg(Move.UP, Move.B, Move.LP);
            case GR -> alg(Move.D, Move.F, Move.DP);
            default -> alg();
        });
        step(switch (cube.getPiece(Point.WO)) {
            case WR -> alg();
            case WO -> alg();
            case WG -> alg(Move.RP, Move.DP, Move.R, Move.D);
            case WB -> alg();
            case YO -> alg(Move.B2);
            case YB -> alg(Move.U, Move.B2);
            case YG -> alg(Move.UP, Move.B2);
            case YR -> alg(Move.U2, Move.B2);
            case RY -> alg(Move.UP, Move.R, Move.BP);
            case RB -> alg(Move.D, Move.L, Move.DP);
            case RG -> alg(Move.DP, Move.RP, Move.D);
            case RW -> alg();
            case BO -> alg(Move.B);
            case BR -> alg(Move.L2, Move.B, Move.L2);
            case BW -> alg();
            case BY -> alg(Move.LP, Move.B, Move.L);
            case OW -> alg(Move.D, Move.L, Move.DP, Move.B);
            case OB -> alg(Move.D, Move.LP, Move.DP);
            case OG -> alg(Move.DP, Move.R, Move.D);
            case OY -> alg(Move.BP, Move.DP, Move.R, Move.D);
            case GO -> alg(Move.BP);
            case GW -> alg(Move.RP, Move.BP);
            case GY -> alg(Move.R, Move.BP);
            case GR -> alg(Move.R2, Move.BP);
            default -> alg();
        });

        step(switch (cube.getPiece(Point.WG)) {
            case YO -> alg(Move.U, Move.R2);
            case YB -> alg(Move.U2, Move.R2);
            case YG -> alg(Move.R2);
            case YR -> alg(Move.UP, Move.R2);
            case RY -> alg(Move.F, Move.RP, Move.FP);
            case RB -> alg(Move.F2, Move.RP, Move.F2);
            case RG -> alg(Move.RP);
            case BO -> alg(Move.D, Move.B, Move.DP);
            case BR -> alg(Move.DP, Move.FP, Move.D);
            case BY -> alg(Move.FP, Move.UP, Move.F, Move.RP);
            case OB -> alg(Move.B2, Move.R, Move.B2);
            case OG -> alg(Move.R);
            case OY -> alg(Move.BP, Move.R, Move.B);
            case GO -> alg(Move.D, Move.BP, Move.DP);
            case GW -> alg(Move.RP, Move.D, Move.BP, Move.DP);
            case GY -> alg(Move.FP, Move.U, Move.F, Move.RP);
            case GR -> alg(Move.DP, Move.F, Move.D);
            default -> alg();
        });
    }

    public void F2L() {
        Point[][] pairs = {
                {
                        Point.WRG, Point.RG, Point.WRB, Point.RB
                }, {
                        Point.WOG, Point.OG, Point.WOB, Point.OB
                }
        };
        for (Point[] pair : pairs) {
            //get the right corner in the right spot
            step(switch (cube.getPiece(pair[0])) {
                case WOG, GWO, OWG -> alg(Move.RP, Move.U, Move.R, Move.U);
                case WOB, BWO, OWB -> alg(Move.BP, Move.U2, Move.BP);
                case WRB, RWB, BWR -> alg(Move.LP, Move.U, Move.L);
                default -> alg();
            });
            //get right edge into right spot
            step(switch (cube.getPiece(pair[1])) {
                case OG -> alg(Move.RP, Move.UP, Move.R);
                case GO -> alg(Move.RP, Move.U, Move.R);
                case BO, OB -> alg(Move.BP, Move.U, Move.B, Move.UP);
                case RB, BR -> alg(Move.LP, Move.U, Move.L, Move.UP);
                default -> alg();
            });
            //use algorithms
            step(switch (cube.getPiece(pair[0])) {
                case RYG -> switch(cube.getPiece(pair[1])) {
                    case YG -> alg(Move.U,Move.R,Move.UP,Move.RP);
                    case YO -> alg(Move.UP,Move.R,Move.U,Move.RP,Move.U2,Move.R,Move.UP,Move.RP);
                    case YB -> alg(Move.UP,Move.R,Move.U2,Move.RP,Move.U2,Move.R,Move.UP,Move.RP);
                    case YR -> alg(Move.FP,Move.U,Move.F,Move.U2,Move.R,Move.U,Move.RP);
                    case BY -> alg(Move.FP,Move.U,Move.F);
                   default -> alg();
                };
                default -> alg();
            });

            step(Move.y2);
        }
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
