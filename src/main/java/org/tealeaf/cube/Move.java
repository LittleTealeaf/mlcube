package org.tealeaf.cube;

import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public enum Move {
    R(new Point[][]{
            {
                    Point.OYB, Point.RWB, Point.WOB, Point.YRB
            }, {
                    Point.WB, Point.OB, Point.YB, Point.RB
            }, {
                    Point.BW, Point.BO, Point.BY, Point.BR
            }, {
                    Point.WRB, Point.OWB, Point.YOB, Point.RYB
            }, {
                    Point.BWR, Point.BWO, Point.BYO, Point.BYR
            }
    }),
    RP(R, Transform.PRIME),
    R2(R, Transform.TWO),
    U(new Point[][]{
            {
                    Point.WG, Point.RG, Point.YG, Point.OG
            }, {
                    Point.GW, Point.GR, Point.GY, Point.GO
            }, {
                    Point.WRG, Point.RYG, Point.YOG, Point.OWG
            }, {
                    Point.OYG, Point.WOG, Point.RWG, Point.YRG
            }, {
                    Point.GYR, Point.GYO, Point.GWO, Point.GWR
            }
    }),
    U2(U, Transform.TWO),
    UP(U, Transform.PRIME),
    L(new Point[][]{
            {
                    Point.WG, Point.RG, Point.YG, Point.OG
            }, {
                    Point.GW, Point.GR, Point.GY, Point.GO
            }, {
                    Point.WOG, Point.RWG, Point.YRG, Point.OYG
            }, {
                    Point.OWG, Point.WRG, Point.RYG, Point.YOG
            }, {
                    Point.GWR, Point.GYR, Point.GYO, Point.GWO
            }
    }),
    L2(L, Transform.TWO),
    LP(L, Transform.PRIME),
    D(new Point[][]{
            {
                    Point.WR, Point.BR, Point.YR, Point.GR
            }, {
                    Point.RW, Point.RB, Point.RY, Point.RG
            }, {
            }
    }),
    D2(D, Transform.TWO),
    DP(D, Transform.PRIME),
    B,
    B2(B, Transform.TWO),
    BP(B, Transform.PRIME),
    F,
    F2(F, Transform.TWO),
    FP(F, Transform.PRIME),

    ;

    private final Set<Point[]> permutations;

    Move() {
        this.permutations = Set.of();
    }

    Move(Point[]... permutations) {
        this.permutations = Set.of(permutations);
    }

    Move(Move... moves) {
        this.permutations = Stream.of(moves).flatMap((move) -> move.permutations.stream()).collect(Collectors.toSet());
    }

    Move(Move move, Transform transform) {
        this.permutations = (
                switch (transform) {
                    case PRIME -> move.permutations.stream().map((perm) -> {
                        Point[] a = new Point[perm.length];
                        for (int i = 0; i < a.length; i++) {
                            a[i] = perm[perm.length - 1 - i];
                        }
                        return a;
                    });
                    case TWO -> move.permutations.stream().flatMap((perm) -> {
                        Point[][] a = new Point[2][perm.length / 2];
                        for (int i = 0; i < perm.length; i++) {
                            a[i % 2][i / 2] = perm[i];
                        }
                        return Stream.of(a[0], a[1]);
                    });
                }
        ).collect(Collectors.toSet());
    }

    public Set<Point[]> getPermutations() {
        return permutations;
    }

    private enum Transform {
        PRIME,
        TWO;
    }
}
