package org.tealeaf.cube;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

import java.util.HashSet;
import java.util.Set;
import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.assertEquals;

class MoveTest {

    @ParameterizedTest
    @MethodSource("provideLoops")
    void testLoops(int cycles, Move[] moves) {
        RubiksCube rubiksCube = new RubiksCube();
        for (int i = 0; i < cycles; i++) {
            rubiksCube.move(moves);
        }
        Piece.POINTS.forEach(piece -> assertEquals(piece, rubiksCube.getPiece(piece)));
    }

    private static Stream<Arguments> provideNotations() {
        return Stream.of(new String[]{
                "RUR'F'RUR'U'R'FR2U'R'"
        }).map(Arguments::of);
    }

    private static Stream<Arguments> provideLoops() {

        Set<Arguments> arguments = new HashSet<>();

        for (Move move : Move.values()) {
            arguments.add(Arguments.of(4, new Move[] {move}));
        }

        arguments.addAll(Stream.of(new String[]{
                "M2 U M U2 M' U M2",
                "M2 U' M U2 M' U' M2",
                "R U' R U R U R U' R' U' R2",
                "R2 U R U R' U' R' U' R' U R'",
                "x L2 D2 L' U' L D2 L' U L' x'",
                "x' L2 D2 L U L' D2 L U' L x",
                "y x L U' L D2 L' U L D2 L2 x' y'",
                "y x R' U R' D2 R U' R' D2 R2 x' y'",
                "y' x' L' U L' D2 L U' L' D2 L2 x y",
                "y' x' R U' R D2 R' U R D2 R2 x y",
                "y2 R' U R' U' R' U' R' U R U R2 y2",
                "y2 R2 U' R' U' R U R U R U' R y2",
                "y2 x R2 D2 R U R' D2 R U' R x' y2",
                "y2 x' R2 D2 R' U' R D2 R' U R' x y2"
        }).map(Move::interpret).map(moves -> Arguments.of(3, moves.toArray(new Move[0]))).toList());
        arguments.addAll(Stream.of(new String[]{
                "F R U' R' U' R U R' F' R U R' U' R' F R F'",
                "M' U M2 U M2 U M' U2 M2 U'",

                "M2 U M2 U2 M2 U M2",

                "R U R' F' R U R' U' R' F R2 U' R' U'",
                "R U R' F' R U2 R' U2 R' F R U R U2 R'",
                "R U R' U R U R' F' R U R' U' R' F R2 U' R' U2 R U' R'",
                "R U R' U' R' F R2 U' R' U' R U R' F'",
                "R' U R U' R' F' U' F R U R' F R' F' R U' R",
                "R' U R' U' y R' F' R2 U' R' U R' F R F y'",
                "R' U' F' R U R' U' R' F R2 U' R' U' R U R' U R",
                "R2 F R U R U' R' F' R U2 R' U2 R U",
                "x R2 F R F' R U2 r' U r U2 x'",
                "x' L' U L D' L' U' L D L' U' L D' L' U L D x",
                "y' R' U L' U2 R U' R' U2 R L y",
                "y2 L' U' L F L' U' L U L F' L2 U L y2"
        }).map(Move::interpret).map(moves -> Arguments.of(2, moves.toArray(new Move[0]))).toList());

//        Set.of(new Move[][] {
//                {
//                    Move.R,Move.U,Move.RP,Move.FP,Move.R,Move.U,Move.RP,Move.UP,Move.RP,Move.F,Move.R2,Move.UP,Move.RP,Move.UP
//                },{
//                    Move.MP,Move.UP,Move.M2,Move.UP,Move.M2,Move.UP,Move.MP,Move.U2,Move.M2,Move.U
//                },{
//                    Move.RP,Move.UP,Move.FP,Move.R,Move.U,Move.RP,Move.UP,Move.RP,Move.F,Move.R2,Move.UP,Move.RP,Move.UP,Move.R,Move.U,Move.RP,Move.U,Move.R
//                }
//        }).forEach(moves -> loops.add(new Loop(2, moves)));
//        Set.of(new Move[][] {
//                {
//                    Move.R2,Move.U,Move.R,Move.U,Move.RP,Move.UP,Move.RP,Move.UP,Move.RP,Move.U,Move.RP
//                }
//        }).forEach(moves -> loops.add(new Loop(3, moves)));

        return arguments.stream();
    }

    static class Loop {

        final int cycles;
        final Move[] moves;

        public Loop(int cycles, Move... moves) {
            this.cycles = cycles;
            this.moves = moves;
        }
    }
}