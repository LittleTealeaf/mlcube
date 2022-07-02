package org.tealeaf.cube;

import java.util.Objects;
import java.util.Random;
import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public class RubiksCube {

    private final Set<Piece> pieces = Point.piecePoints.stream().map(Piece::new).collect(Collectors.toSet());

    public RubiksCube() {

    }

    public void move(Move... moves) {
        for(Move move : moves) {
            pieces.forEach(move::apply);
        }
    }

    public void scramble(int count) {
        for(int i = 0; i < count; i++) {
            move(Move.values()[new Random().nextInt(Move.values().length)]);
        }
    }

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
