package org.tealeaf.cube;

import java.util.Collection;
import java.util.Random;
import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.Stream;

/**
 * White on bottom looking at red side
 */

//    //    B(perm(Point.OY, Point.OB, Point.OW, Point.OG), perm(Point.YO, Point.BO, Point.WO, Point.GO), perm(Point.OYB, Point.OWB, Point
// .OWG, Point.OYG),
////      perm(Point.YOB, Point.BWO, Point.WOG, Point.GYO), perm(Point.YOG, Point.BYO, Point.WOB, Point.GWO)
////    ),
//    B,
//    D,
//    E,
//    F,
//    L,
//    M,
//    NULL,
//    R,
//    S,
//    //    U(perm(Point.YG, Point.YR, Point.YB, Point.YO), perm(Point.GY, Point.RY, Point.BY, Point.OY), perm(Point.YRG, Point.YRB, Point.YOB, Point.YOG),
////      perm(Point.GYR, Point.RYB, Point.BYO, Point.OYG), perm(Point.GYO, Point.RYG, Point.BYR, Point.OYB)
////    ),
//    U,
//    x,
//    y,
//    z;

public enum Move {
    B(Set.of(Perms.B.normal())),
    B2(Set.of(Perms.B.two())),
    BP(Set.of(Perms.B.prime())),
    D(Set.of(Perms.D.normal())),
    D2(Set.of(Perms.D.two())),
    DP(Set.of(Perms.D.prime())),
    E(Set.of(Perms.E.normal())),
    E2(Set.of(Perms.E.two())),
    EP(Set.of(Perms.E.prime())),
    F(Set.of(Perms.F.normal())),
    F2(Set.of(Perms.F.two())),
    FP(Set.of(Perms.F.prime())),
    L(Set.of(Perms.L.normal())),
    L2(Set.of(Perms.L.two())),
    LP(Set.of(Perms.L.prime())),
    M(Set.of(Perms.M.normal())),
    M2(Set.of(Perms.M.two())),
    MP(Set.of(Perms.M.prime())),
    R(Set.of(Perms.R.normal())),
    R2(Set.of(Perms.R.two())),
    RP(Set.of(Perms.R.prime())),
    S(Set.of(Perms.S.normal())),
    S2(Set.of(Perms.S.two())),
    SP(Set.of(Perms.S.prime())),
    U(Set.of(Perms.U.normal())),
    U2(Set.of(Perms.U.two())),
    UP(Set.of(Perms.U.prime())),
    u(Set.of(Perms.U.two(),Perms.E.prime())),
    u2(Set.of(Perms.U.two(),Perms.E.two())),
    uP(Set.of(Perms.U.prime(),Perms.E.normal())),
    ;

    private final Set<Point[]> permutations;

    Move() {
        permutations = Set.of();
    }

    Move(Set<Set<Point[]>> perms) {
        this(perms.stream());
    }

    Move(Stream<Set<Point[]>> perms) {
        this.permutations = perms.flatMap(Collection::stream).collect(Collectors.toSet());
    }

    public Set<Point[]> getPermutations() {
        return permutations;
    }

//    public Point apply(Point point) {
//        return permutations.parallelStream().map(perm -> {
//            for (int i = 0; i < perm.length; i++) {
//                if (point == perm[i]) {
//                    return perm[(i + 1) % perm.length];
//                }
//            }
//            return null;
//        }).filter(Objects::nonNull).findAny().orElse(point);
//    }
//
//    public void apply(Piece piece) {
//        piece.setPosition(apply(piece.getPosition()));
//    }

    public static Move random() {
        return values()[new Random().nextInt(values().length)];
    }

    private enum Perms {
        B(new Point[][]{
                {
                        Point.OY, Point.OB, Point.OW, Point.OG
                }, {
                        Point.YO, Point.BO, Point.WO, Point.GO
                }, {
                        Point.OYB, Point.OWB, Point.OWG, Point.OYG
                }, {
                        Point.YOB, Point.BWO, Point.WOG, Point.GYO
                }, {
                        Point.YOG, Point.BYO, Point.WOB, Point.GWO
                }
        }),
        D(new Point[][]{
                {
                        Point.RW, Point.GW, Point.OW, Point.BW
                }, {
                        Point.WR, Point.WG, Point.WO, Point.WB
                }, {
                        Point.WRG, Point.WOG, Point.WOB, Point.WRB
                }, {
                        Point.RWG, Point.GWO, Point.OWB, Point.BWR
                }, {
                        Point.BWO, Point.RWB, Point.GWR, Point.OWG
                }
        }),
        E,
        F,
        L,
        M,
        R,
        S,
        U;

        final Set<Point[]> permutations;

        Perms(Point[]... permutations) {
            this.permutations = Set.of(permutations);
        }

        Set<Point[]> normal() {
            return permutations;
        }

        Set<Point[]> prime() {
            return permutations.stream().map(perm -> {
                Point[] r = new Point[perm.length];
                for (int i = 0; i < r.length; i++) {
                    r[i] = perm[perm.length - i - 1];
                }
                return r;
            }).collect(Collectors.toSet());
        }

        Set<Point[]> two() {
            return permutations.stream().flatMap(perm -> {
                Point[][] r = new Point[2][perm.length / 2];
                for (int i = 0; i < perm.length; i++) {
                    r[i % 2][i / 2] = perm[i];
                }
                return Stream.of(r[0], r[1]);
            }).collect(Collectors.toSet());
        }
    }
}
