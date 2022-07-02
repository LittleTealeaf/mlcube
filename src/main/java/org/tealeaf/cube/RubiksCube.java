package org.tealeaf.cube;

import java.util.Objects;
import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public class RubiksCube {

    private final Set<Piece> pieces = Point.piecePoints.stream().map(Piece::new).collect(Collectors.toSet());

    public RubiksCube() {

    }

//    public void move(Stream<Move> moves) {
//        moves.forEach(move -> pieces.parallelStream().forEach(move::apply));
//    }
//
//    public void move(Move... moves) {
//
//        move(Stream.of(moves));
//    }
//
//    public void scramble(int count) {
//        move(IntStream.range(0, count).mapToObj(i -> Move.random()));
//    }

    public Set<Piece> getPieces() {
        return pieces;
    }

    public Point getPiece(Point point) {
        return Objects.requireNonNull(pieces.parallelStream().filter(piece -> piece.getPiece() == point).findFirst().orElse(null)).getPosition();
    }

    public String toString() {
        return pieces.toString();
    }
}
