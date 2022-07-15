package org.tealeaf.environment;

import java.util.HashMap;
import java.util.Map;
import java.util.Random;

public enum Move {
    B(Perm.B.build()),
    B2(Perm.B.buildTwo()),
    BP(Perm.B.buildPrime()),
    D(Perm.D.build()),
    D2(Perm.D.buildTwo()),
    DP(Perm.D.buildPrime()),
    F(Perm.F.build()),
    F2(Perm.F.buildTwo()),
    FP(Perm.F.buildPrime()),
    L(Perm.L.build()),
    L2(Perm.L.buildTwo()),
    LP(Perm.L.buildPrime()),
    R(Perm.R.build()),
    R2(Perm.R.buildTwo()),
    RP(Perm.R.buildPrime()),
    U(Perm.U.build()),
    U2(Perm.U.buildTwo()),
    UP(Perm.U.buildPrime());

    private Map<Position, Position> permutations;

    Move(Map<Position, Position> permutations) {
        this.permutations = permutations;
    }

    public Position apply(Position position) {
        return permutations.getOrDefault(position,position);
   }

   public void apply(Piece piece) {
        piece.setPosition(permutations.getOrDefault(piece.getPosition(),piece.getPosition()));
   }

    public Map<Position, Position> getPermutations() {
        return permutations;
    }

    private static Position[] LOOP(Position... positions) {
        return positions;
    }

    private static final Random rng = new Random();

    public static Move random() {
        return values()[rng.nextInt(values().length)];
    }

    private enum Perm {

        R(new Position[][]{
                LOOP(Position.GR, Position.WR, Position.BR, Position.YR),
                LOOP(Position.RG, Position.RW, Position.RB, Position.RY),
                LOOP(Position.GWR, Position.WBR, Position.BYR, Position.YGR),
                LOOP(Position.GYR, Position.WGR, Position.BWR, Position.YBR),
                LOOP(Position.RWG, Position.RWB, Position.RYB, Position.RYG)
        }),
        U(new Position[][]{
                LOOP(Position.WG, Position.WO, Position.WB, Position.WR),
                LOOP(Position.GW, Position.OW, Position.BW, Position.RW),
                LOOP(Position.WGO, Position.WBO, Position.WBR, Position.WGR),
                LOOP(Position.GWO, Position.OWB, Position.BWR, Position.RWG),
                LOOP(Position.OWG, Position.BWO, Position.RWB, Position.GWR)
        }),
        F(new Position[][]{
                LOOP(Position.GW, Position.GR, Position.GY, Position.GO),
                LOOP(Position.WG, Position.RG, Position.YG, Position.OG),
                LOOP(Position.GYR, Position.GYO, Position.GWO, Position.GWR),
                LOOP(Position.YGO, Position.OWG, Position.WGR, Position.RYG),
                LOOP(Position.YGR, Position.OYG, Position.WGO, Position.RWG)
        }),
        L(new Position[][]{
                LOOP(Position.OG, Position.OY, Position.OB, Position.OW),
                LOOP(Position.GO, Position.YO, Position.BO, Position.WO),
                LOOP(Position.OYG, Position.OYB, Position.OWB, Position.OWG),
                LOOP(Position.GYO, Position.YBO, Position.BWO, Position.WGO),
                LOOP(Position.WBO, Position.GWO, Position.YGO, Position.BYO)
        }),
        D(new Position[][]{
                LOOP(Position.YG, Position.YR, Position.YB, Position.YO),
                LOOP(Position.GY, Position.RY, Position.BY, Position.OY),
                LOOP(Position.YGR, Position.YBR, Position.YBO, Position.YGO),
                LOOP(Position.GYR, Position.RYB, Position.BYO, Position.OYG),
                LOOP(Position.RYG, Position.BYR, Position.OYB, Position.GYO)
        }),
        B(new Position[][]{
                LOOP(Position.BW, Position.BO, Position.BY, Position.BR),
                LOOP(Position.WB, Position.OB, Position.YB, Position.RB),
                LOOP(Position.BYO, Position.BYR, Position.BWR, Position.BWO),
                LOOP(Position.YBR, Position.RWB, Position.WBO, Position.OYB),
                LOOP(Position.OWB, Position.YBO, Position.RYB, Position.WBR)
        });

        final Position[][] loops;

        Perm(Position[][] loops) {
            this.loops = loops;
        }

        Map<Position, Position> build() {
            int length = 0;
            for (int i = 0; i < loops.length; i++) {
                length += loops[i].length;
            }
            Map<Position, Position> map = new HashMap<>(length);

            for (Position[] loop : loops) {
                for (int i = 0; i < loop.length; i++) {
                    map.put(loop[i], loop[(i + 1) % loop.length]);
                }
            }

            return map;
        }

        Map<Position, Position> buildPrime() {
            int length = 0;
            for (int i = 0; i < loops.length; i++) {
                length += loops[i].length;
            }
            Map<Position, Position> map = new HashMap<>(length);

            for (Position[] loop : loops) {
                map.put(loop[0], loop[loop.length - 1]);
                for (int i = 1; i < loop.length; i++) {
                    map.put(loop[i], loop[i - 1]);
                }
            }

            return map;
        }

        Map<Position, Position> buildTwo() {
            int length = 0;
            for (int i = 0; i < loops.length; i++) {
                length += loops[i].length;
            }
            Map<Position, Position> map = new HashMap<>(length);

            for (Position[] loop : loops) {
                for (int i = 0; i < loop.length; i++) {
                    map.put(loop[i], loop[(i + 2) % loop.length]);
                }
            }

            return map;
        }
    }
}
