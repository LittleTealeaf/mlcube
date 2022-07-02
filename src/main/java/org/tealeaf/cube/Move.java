package org.tealeaf.cube;

import java.util.HashMap;
import java.util.Map;
import java.util.stream.Stream;

public enum Move {
    U(build().normal(Perm.U)),
    D(build().normal(Perm.D)),
    R(build().normal(Perm.R)),
    L(build().normal(Perm.L)),
    F(build().normal(Perm.F)),
    B(build().normal(Perm.B)),
    M(build().normal(Perm.M)),
    E(build().normal(Perm.E)),
    S(build().normal(Perm.S)),
    x(build().normal(Perm.R).prime(Perm.M).prime(Perm.L)),
    y(build().normal(Perm.U).prime(Perm.E).prime(Perm.D)),
    z(build().normal(Perm.F).prime(Perm.B).normal(Perm.S)),
    u(build().normal(Perm.U).prime(Perm.E)),
    d(build().normal(Perm.D).normal(Perm.E)),
    r(build().normal(Perm.R).prime(Perm.M)),
    l(build().normal(Perm.L).normal(Perm.M)),
    f(build().normal(Perm.F).normal(Perm.S)),
    b(build().normal(Perm.B).prime(Perm.S)),
    ;

    private final Map<Point,Point> permutations;

    Move() {
        permutations = new HashMap<>();
    }

    Move(Builder builder) {
        permutations = builder.map;
    }

    public void apply(Piece piece) {
        piece.setPosition(permutations.getOrDefault(piece.getPosition(),piece.getPosition()));
    }

    private enum Perm {
        U,D,R,L,F,B,M,E,S;

        final Point[][] permutations;

        Perm(Point[]... permutations) {
            this.permutations = permutations;
        }
    }


    private static class Builder {

        Map<Point,Point> map = new HashMap<>();

        public Builder() {

        }

        Builder normal(Perm perm) {
            Stream.of(perm.permutations).forEach(p -> {
               for(int i = 0; i < p.length; i++) {
                   map.put(p[i],p[(i+1)%p.length]);
               }
            });
            return this;
        }

        Builder prime(Perm perm) {
            Stream.of(perm.permutations).forEach(p -> {
                map.put(p[0],p[p.length - 1]);
                for(int i = 1; i < p.length; i++) {
                    map.put(p[i],p[i-1]);
                }
            });
            return this;
        }

        Builder two(Perm perm) {
            Stream.of(perm.permutations).forEach(p -> {
                for(int i = 0; i < p.length; i++) {
                    map.put(p[i],p[(i+2)%p.length]);
                }
            });
            return this;
        }




    }

    private static Builder build() {
        return new Builder();
    }
}
