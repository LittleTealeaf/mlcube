package org.tealeaf.cube;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Stream;

/**
 * <img src="https://jperm.net/images/notation.png"/>
 */
public enum MoveDeprecated {
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

    private final Map<PointDeprecated, PointDeprecated> permutations;

    MoveDeprecated() {
        permutations = new HashMap<>();
    }

    MoveDeprecated(Builder builder) {
        permutations = builder.map;
    }

    public void apply(PieceDeprecated pieceDeprecated) {
        pieceDeprecated.setPosition(permutations.getOrDefault(pieceDeprecated.getPosition(), pieceDeprecated.getPosition()));
    }

    @Override
    public String toString() {
        return permutations.toString();
    }

    private enum Perm {
        L(new PointDeprecated[][]{
                {
                        PointDeprecated.OY, PointDeprecated.OB, PointDeprecated.OW, PointDeprecated.OG
                }, {
                        PointDeprecated.WO, PointDeprecated.GO, PointDeprecated.YO, PointDeprecated.BO
                }, {
                        PointDeprecated.OWG, PointDeprecated.OYG, PointDeprecated.OYB, PointDeprecated.OWB
                }, {
                        PointDeprecated.WOB, PointDeprecated.GWO, PointDeprecated.YOG, PointDeprecated.BYO
                }, {
                        PointDeprecated.BWO, PointDeprecated.WOG, PointDeprecated.GYO, PointDeprecated.YOB
                }
        }),
        U(new PointDeprecated[][]{
                {
                        PointDeprecated.WR, PointDeprecated.WG, PointDeprecated.WO, PointDeprecated.WB
                }, {
                        PointDeprecated.RW, PointDeprecated.GW, PointDeprecated.OW, PointDeprecated.BW
                }, {
                        PointDeprecated.WRB, PointDeprecated.WRG, PointDeprecated.WOG, PointDeprecated.WOB
                }, {
                        PointDeprecated.RWB, PointDeprecated.GWR, PointDeprecated.OWG, PointDeprecated.BWO
                }, {
                        PointDeprecated.BWR, PointDeprecated.RWG, PointDeprecated.GWO, PointDeprecated.OWB
                }
        }),
        E(new PointDeprecated[][]{
                {
                        PointDeprecated.B, PointDeprecated.O, PointDeprecated.G, PointDeprecated.R
                }, {
                        PointDeprecated.BR, PointDeprecated.OB, PointDeprecated.GO, PointDeprecated.RG
                }, {
                        PointDeprecated.OG, PointDeprecated.BO, PointDeprecated.RB, PointDeprecated.GR
                }
        }),
        R(new PointDeprecated[][]{
                {
                        PointDeprecated.RW, PointDeprecated.RB, PointDeprecated.RY, PointDeprecated.RG
                }, {
                        PointDeprecated.WR, PointDeprecated.BR, PointDeprecated.YR, PointDeprecated.GR
                }, {
                        PointDeprecated.RWG, PointDeprecated.RWB, PointDeprecated.RYB, PointDeprecated.RYG
                }, {
                        PointDeprecated.WRG, PointDeprecated.BWR, PointDeprecated.YRB, PointDeprecated.GYR
                }, {
                        PointDeprecated.GWR, PointDeprecated.WRB, PointDeprecated.BYR, PointDeprecated.YRG
                }
        }),
        B(new PointDeprecated[][]{
                {
                        PointDeprecated.BR, PointDeprecated.BW, PointDeprecated.BO, PointDeprecated.BY
                }, {
                        PointDeprecated.RB, PointDeprecated.WB, PointDeprecated.OB, PointDeprecated.YB
                }, {
                        PointDeprecated.BYR, PointDeprecated.BWR, PointDeprecated.BWO, PointDeprecated.BYO
                }, {
                        PointDeprecated.RWB, PointDeprecated.WOB, PointDeprecated.OYB, PointDeprecated.YRB
                }, {
                        PointDeprecated.YOB, PointDeprecated.RYB, PointDeprecated.WRB, PointDeprecated.OWB
                }
        }),
        S(new PointDeprecated[][]{
                {
                        PointDeprecated.O, PointDeprecated.W, PointDeprecated.R, PointDeprecated.Y
                }, {
                        PointDeprecated.OY, PointDeprecated.WO, PointDeprecated.RW, PointDeprecated.YR
                }, {
                        PointDeprecated.YO, PointDeprecated.OW, PointDeprecated.WR, PointDeprecated.RY
                }
        }),
        F(new PointDeprecated[][]{
                {
                        PointDeprecated.GY, PointDeprecated.GO, PointDeprecated.GW, PointDeprecated.GR
                }, {
                        PointDeprecated.YG, PointDeprecated.OG, PointDeprecated.WG, PointDeprecated.RG
                }, {
                        PointDeprecated.GYR, PointDeprecated.GYO, PointDeprecated.GWO, PointDeprecated.GWR
                }, {
                        PointDeprecated.YOG, PointDeprecated.OWG, PointDeprecated.WRG, PointDeprecated.RYG
                }, {
                        PointDeprecated.RWG, PointDeprecated.YRG, PointDeprecated.OYG, PointDeprecated.WOG
                }
        }),
        M(new PointDeprecated[][]{
                {
                        PointDeprecated.B, PointDeprecated.W, PointDeprecated.G, PointDeprecated.Y
                }, {
                        PointDeprecated.GW, PointDeprecated.YG, PointDeprecated.BY, PointDeprecated.WB
                }, {
                        PointDeprecated.WG, PointDeprecated.GY, PointDeprecated.YB, PointDeprecated.BW
                }
        }),
        D(new PointDeprecated[][]{
                {
                        PointDeprecated.YR, PointDeprecated.YB, PointDeprecated.YO, PointDeprecated.YG
                }, {
                        PointDeprecated.RY, PointDeprecated.BY, PointDeprecated.OY, PointDeprecated.GY
                }, {
                        PointDeprecated.YRG, PointDeprecated.YRB, PointDeprecated.YOB, PointDeprecated.YOG
                }, {
                        PointDeprecated.RYB, PointDeprecated.BYO, PointDeprecated.OYG, PointDeprecated.GYR
                }, {
                        PointDeprecated.GYO, PointDeprecated.RYG, PointDeprecated.BYR, PointDeprecated.OYB
                }
        });

        final PointDeprecated[][] permutations;

        Perm(PointDeprecated[]... permutations) {
            this.permutations = permutations;
        }
    }

    private static class Builder {

        Map<PointDeprecated, PointDeprecated> map = new HashMap<>();

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

        Builder makePrime() {
            Map<PointDeprecated, PointDeprecated> primeMap = new HashMap<>();
            map.forEach((key, value) -> primeMap.put(value, key));
            this.map = primeMap;
            return this;
        }

        Builder makeTwo() {
            Map<PointDeprecated, PointDeprecated> twoMap = new HashMap<>();
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

    public static List<MoveDeprecated> interpret(String set) {
        boolean prime = false, two = false;

        List<MoveDeprecated> moveDeprecateds = new ArrayList<>();

        for (int i = set.length() - 1; i >= 0; i--) {
            switch (set.charAt(i)) {
                case '\'' -> {
                    prime = true;
                }
                case '2' -> two = true;
                case ' ' -> {}
                default -> {
                    String string = set.charAt(i) + "";
                    if (two) {
                        string += "2";
                    } else if (prime) {
                        string += "P";
                    }
                    prime = two = false;
                    moveDeprecateds.add(0, valueOf(string));
                }
            }
        }

        return moveDeprecateds;
    }
}
