package org.tealeaf.cube;

import java.util.Objects;
import java.util.Random;
import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public enum Move {
    R(new Point[][]{

            {
                    Point.WB, Point.OB, Point.YB, Point.RB
            }, {
                    Point.WOB, Point.OYB, Point.YRB, Point.RWB
            }, {
                    Point.WRB, Point.OWB, Point.YOB, Point.RYB
            }, {
                    Point.BW, Point.BO, Point.BY, Point.BR
            }, {
                    Point.BWO, Point.BYO, Point.BYR, Point.BWR
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
                    Point.WRG, Point.BWR, Point.YRB, Point.GYR
            }, {
                    Point.GWR, Point.WRB, Point.BYR, Point.YRG
            }, {
                    Point.RWB, Point.RYB, Point.RYG, Point.RWG
            }
    }),
    D2(D, Transform.TWO),
    DP(D, Transform.PRIME),
    B(new Point[][]{
            {
                    Point.YR, Point.YB, Point.YO, Point.YG
            }, {
                    Point.YRB, Point.YOB, Point.YOG, Point.YRG
            }, {
                    Point.RY, Point.BY, Point.OY, Point.GY
            }, {
                    Point.RYB, Point.BYO, Point.OYG, Point.GYR
            }, {
                    Point.GYO, Point.RYG, Point.BYR, Point.OYB
            }
    }),
    B2(B, Transform.TWO),
    BP(B, Transform.PRIME),
    F(new Point[][]{
            {
                    Point.RW, Point.GW, Point.OW, Point.BW
            }, {
                    Point.WR, Point.WG, Point.WO, Point.WB
            }, {
                    Point.WRB, Point.WRG, Point.WOG, Point.WOB
            }, {
                    Point.RWG, Point.GWO, Point.OWB, Point.BWR
            }, {
                    Point.BWO, Point.RWB, Point.GWR, Point.OWG
            }
    }),
    F2(F, Transform.TWO),
    FP(F, Transform.PRIME),
    MP(new Point[][]{
            {
                    Point.W, Point.O, Point.Y, Point.R
            }, {
                    Point.WO, Point.OY, Point.YR, Point.RW
            }, {
                    Point.WR, Point.OW, Point.YO, Point.RY
            }
    }),
    M(MP, Transform.PRIME),
    M2(M, Transform.TWO),
    E(new Point[][]{
            {
                    Point.W, Point.B, Point.Y, Point.G
            }, {
                    Point.WB, Point.BY, Point.YG, Point.GW
            }, {
                    Point.WG, Point.BW, Point.YB, Point.GY
            }
    }),
    E2(E, Transform.TWO),
    EP(E, Transform.PRIME),
    S(new Point[][]{
            {
                    Point.O, Point.B, Point.R, Point.G
            }, {
                    Point.OB, Point.BR, Point.RG, Point.GO
            }, {
                    Point.OG, Point.BO, Point.RB, Point.GR
            }
    }),
    SP(S, Transform.PRIME),
    S2(S, Transform.TWO),
    X(MP, LP, R),
    Y(EP, U, DP),
    Z(S, F, BP),
    XP(X, Transform.PRIME),
    X2(X, Transform.TWO),
    YP(Y, Transform.PRIME),
    Y2(Y, Transform.TWO),
    ZP(Z, Transform.PRIME),
    Z2(Z, Transform.TWO),
    r(R, MP),
    l(L, M),
    f(F, S),
    b(B, SP),
    d(D, E),
    u(U, EP),
    rP(r, Transform.PRIME),
    lP(l, Transform.PRIME),
    fP(f, Transform.PRIME),
    bP(b, Transform.PRIME),
    dP(d, Transform.PRIME),
    uP(u, Transform.PRIME),
    r2(r, Transform.TWO),
    l2(l, Transform.TWO),
    f2(f, Transform.TWO),
    b2(b, Transform.TWO),
    d2(d, Transform.TWO),
    u2(u, Transform.TWO),
    NONE;

    private final Set<Point[]> permutations;

    Move() {
        permutations = Set.of();
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

    public Point apply(Point point) {
        return permutations.parallelStream().map(perm -> {
            for (int i = 0; i < perm.length; i++) {
                if (point == perm[i]) {
                    return perm[(i + 1) % perm.length];
                }
            }
            return null;
        }).filter(Objects::nonNull).findAny().orElse(point);
    }

    public void apply(Piece piece) {
        piece.setPosition(apply(piece.getPosition()));
    }

    public static Move random() {
        return values()[new Random().nextInt(values().length)];
    }

    private enum Transform {
        PRIME,
        TWO;
    }
}
