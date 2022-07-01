package org.tealeaf.cube;

import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public class RubiksCube {


    private final Set<Piece> pieces = Point.piecePoints.stream().map(Piece::new).collect(Collectors.toSet());

    public RubiksCube() {

    }

    public void move(Move... moves) {
        Stream.of(moves).forEach(move -> pieces.parallelStream().forEach(move::apply));
    }

    public void scramble(int count) {
        IntStream.range(0,count).parallel().mapToObj(i -> Move.values()[(int) (Math.random() * Move.values().length)]).forEach(this::move);
    }

    public Set<Piece> getPieces() {
        return pieces;
    }

    public String toString() {
        return pieces.toString();
    }
}
