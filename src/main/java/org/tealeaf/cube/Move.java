package org.tealeaf.cube;

import java.util.HashMap;
import java.util.Map;
import java.util.stream.Stream;

public enum Move {
    B(build().normal(Perm.B)),
    B2(build().normal(Perm.B).makeTwo()),
    BP(build().normal(Perm.B).makePrime()),
    D(build().normal(Perm.D)),
    D2(build().normal(Perm.D).makeTwo()),
    DP(build().normal(Perm.D).makePrime()),
    E(build().normal(Perm.E)),
    E2(build().normal(Perm.E).makeTwo()),
    EP(build().normal(Perm.E).makePrime()),
    F(build().normal(Perm.F)),
    F2(build().normal(Perm.F).makeTwo()),
    FP(build().normal(Perm.F).makePrime()),
    L(build().normal(Perm.L)),
    L2(build().normal(Perm.L).makeTwo()),
    LP(build().normal(Perm.L).makePrime()),
    M(build().normal(Perm.M)),
    M2(build().normal(Perm.M).makeTwo()),
    MP(build().normal(Perm.M).makePrime()),
    NULL,
    R(build().normal(Perm.R)),
    R2(build().normal(Perm.R).makeTwo()),
    RP(build().normal(Perm.R).makePrime()),
    S(build().normal(Perm.S)),
    S2(build().normal(Perm.S).makeTwo()),
    SP(build().normal(Perm.S).makePrime()),
    U(build().normal(Perm.U)),
    U2(build().normal(Perm.U).makeTwo()),
    UP(build().normal(Perm.U).makePrime()),
    b(build().normal(Perm.B).prime(Perm.S)),
    b2(build().normal(Perm.B).prime(Perm.S).makeTwo()),
    bP(build().normal(Perm.B).prime(Perm.S).makePrime()),
    d(build().normal(Perm.D).normal(Perm.E)),
    d2(build().normal(Perm.D).normal(Perm.E).makeTwo()),
    dP(build().normal(Perm.D).normal(Perm.E).makePrime()),
    f(build().normal(Perm.F).normal(Perm.S)),
    f2(build().normal(Perm.F).normal(Perm.S).makeTwo()),
    fP(build().normal(Perm.F).normal(Perm.S).makePrime()),
    l(build().normal(Perm.L).normal(Perm.M)),
    l2(build().normal(Perm.L).normal(Perm.M).makeTwo()),
    lP(build().normal(Perm.L).normal(Perm.M).makePrime()),
    r(build().normal(Perm.R).prime(Perm.M)),
    r2(build().normal(Perm.R).prime(Perm.M).makeTwo()),
    rP(build().normal(Perm.R).prime(Perm.M).makePrime()),
    u(build().normal(Perm.U).prime(Perm.E)),
    u2(build().normal(Perm.U).prime(Perm.E).makeTwo()),
    uP(build().normal(Perm.U).prime(Perm.E).makePrime()),
    x(build().normal(Perm.R).prime(Perm.M).prime(Perm.L)),
    x2(build().normal(Perm.R).prime(Perm.M).prime(Perm.L).makeTwo()),
    xP(build().normal(Perm.R).prime(Perm.M).prime(Perm.L).makePrime()),
    y(build().normal(Perm.U).prime(Perm.E).prime(Perm.D)),
    y2(build().normal(Perm.U).prime(Perm.E).prime(Perm.D).makeTwo()),
    yP(build().normal(Perm.U).prime(Perm.E).prime(Perm.D).makePrime()),
    z(build().normal(Perm.F).prime(Perm.B).normal(Perm.S)),
    z2(build().normal(Perm.F).prime(Perm.B).normal(Perm.S).makeTwo()),
    zP(build().normal(Perm.F).prime(Perm.B).normal(Perm.S).makePrime());

    private final Map<Point, Point> permutations;

    Move() {
        permutations = new HashMap<>();
    }

    Move(Builder builder) {
        permutations = builder.map;
    }

    public void apply(Piece piece) {
        piece.setPosition(permutations.getOrDefault(piece.getPosition(), piece.getPosition()));
    }

    private enum Perm {
        B(new Point[][]{
                {
                        Point.OY, Point.OB, Point.OW, Point.OG
                }, {
                        Point.WO, Point.GO, Point.YO, Point.BO
                }, {
                        Point.OWG, Point.OYG, Point.OYB, Point.OWB
                }, {
                        Point.WOB, Point.GWO, Point.YOG, Point.BYO
                }, {
                        Point.BWO, Point.WOG, Point.GYO, Point.YOB
                }
        }),
        D(new Point[][]{
                {
                        Point.WR, Point.WG, Point.WO, Point.WB
                }, {
                        Point.RW, Point.GW, Point.OW, Point.BW
                }, {
                        Point.WRB, Point.WRG, Point.WOG, Point.WOB
                }, {
                        Point.RWB, Point.GWR, Point.OWG, Point.BWO
                }, {
                        Point.BWR, Point.RWG, Point.GWO, Point.OWB
                }
        }),
        E(new Point[][]{
                {
                        Point.R, Point.G, Point.O, Point.B
                }, {
                        Point.RG, Point.GO, Point.OB, Point.BR
                }, {
                        Point.BO, Point.RB, Point.GR, Point.OG
                }
        }),
        F(new Point[][]{
                {
                        Point.RW, Point.RB, Point.RY, Point.RG
                }, {
                        Point.WR, Point.BR, Point.YR, Point.GR
                }, {
                        Point.RWG, Point.RWB, Point.RYB, Point.RYG
                }, {
                        Point.WRG, Point.BWR, Point.YRB, Point.GYR
                }, {
                        Point.GWR, Point.WRB, Point.BYR, Point.YRG
                }
        }),
        L(new Point[][]{
                {
                        Point.BR, Point.BW, Point.BO, Point.BY
                }, {
                        Point.RB, Point.WB, Point.OB, Point.YB
                }, {
                        Point.BYR, Point.BWR, Point.BWO, Point.BYO
                }, {
                        Point.RWB, Point.WOB, Point.OYB, Point.YRB
                }, {
                        Point.YOB, Point.RYB, Point.WRB, Point.OWB
                }
        }),
        M(new Point[][]{
                {
                        Point.Y, Point.R, Point.W, Point.O
                }, {
                        Point.YR, Point.RW, Point.WO, Point.OY
                }, {
                        Point.RY, Point.WR, Point.OW, Point.YO
                }
        }),
        R(new Point[][]{
                {
                        Point.GY, Point.GO, Point.GW, Point.GR
                }, {
                        Point.YG, Point.OG, Point.WG, Point.RG
                }, {
                        Point.GYR, Point.GYO, Point.GWO, Point.GWR
                }, {
                        Point.YOG, Point.OWG, Point.WRG, Point.RYG
                }, {
                        Point.RWG, Point.YRG, Point.OYG, Point.WOG
                }
        }),
        S(new Point[][]{
                {
                        Point.Y, Point.G, Point.W, Point.B
                }, {
                        Point.WB, Point.BY, Point.YG, Point.GW
                }, {
                        Point.BW, Point.YB, Point.GY, Point.WG
                }
        }),
        U(new Point[][]{
                {
                        Point.YR, Point.YB, Point.YO, Point.YG
                }, {
                        Point.YRG, Point.YRB, Point.YOB, Point.YOG
                }, {
                        Point.RYB, Point.BYO, Point.OYG, Point.GYR
                }, {
                        Point.GYO, Point.RYG, Point.BYR, Point.OYB
                }
        });

        final Point[][] permutations;

        Perm(Point[]... permutations) {
            this.permutations = permutations;
        }
    }

    private static class Builder {

        Map<Point, Point> map = new HashMap<>();

        public Builder() {

        }

        Builder normal(Perm perm) {
            Stream.of(perm.permutations).forEach(p -> {
                for (int i = 0; i < p.length; i++) {
                    map.put(p[i], p[(i + 1) % p.length]);
                }
            });
            return this;
        }

        Builder prime(Perm perm) {
            Stream.of(perm.permutations).forEach(p -> {
                map.put(p[0], p[p.length - 1]);
                for (int i = 1; i < p.length; i++) {
                    map.put(p[i], p[i - 1]);
                }
            });
            return this;
        }

        Builder two(Perm perm) {
            Stream.of(perm.permutations).forEach(p -> {
                for (int i = 0; i < p.length; i++) {
                    map.put(p[i], p[(i + 2) % p.length]);
                }
            });
            return this;
        }

        Builder makePrime() {
            Map<Point, Point> primeMap = new HashMap<>();
            map.forEach((key, value) -> primeMap.put(value, key));
            this.map = primeMap;
            return this;
        }

        Builder makeTwo() {
            Map<Point, Point> twoMap = new HashMap<>();
            map.forEach((key, value) -> {
                twoMap.put(key, map.getOrDefault(value, key));
            });
            this.map = twoMap;
            return this;
        }
    }

    private static Builder build() {
        return new Builder();
    }
}
